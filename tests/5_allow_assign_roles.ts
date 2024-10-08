import {
  file_pda,
  role_pda,
  WRITE_PERM,
  rule_pda,
  READ_PERM,
  seed_pda,
} from "./common";
import {
  ANOTHER_WALLET,
  FILE_ID,
  PROGRAM,
  ALLOWED_WALLET,
  addressType,
  namespaces,
} from "./constants";
import { expect } from "chai";

describe("5.- Allow assign roles", () => {
  let filePDA = null; // Populated on before() block
  let writeRulePDA = null; // Populated on before() block
  let allowedWalletRole = null; // Populated on before() block
  let allowedWalletSeedPDA = null; // Populated on before() block
  let anotherWalletRole = null; // Populated on before() block
  let anotherWalletSeedPDA = null; // Populated on before() block

  before(async () => {
    filePDA = await file_pda();
    writeRulePDA = await rule_pda(
      WRITE_PERM.role,
      WRITE_PERM.resource,
      WRITE_PERM.permission
    );
    allowedWalletRole = await role_pda(
      WRITE_PERM.role,
      ALLOWED_WALLET.publicKey
    );
    anotherWalletRole = await role_pda(
      WRITE_PERM.role,
      ANOTHER_WALLET.publicKey
    );
    allowedWalletSeedPDA = await seed_pda(ALLOWED_WALLET.publicKey);
    anotherWalletSeedPDA = await seed_pda(ANOTHER_WALLET.publicKey);
  });

  it("Wallet not allowed to assign role", async () => {
    try {
      // ALLOWED_WALLET does not have permission to assign  the "Authenticated" role
      await PROGRAM.methods
        .assignRole({
          address: ANOTHER_WALLET.publicKey,
          role: WRITE_PERM.role,
          addressType: addressType.Wallet,
          expiresAt: null,
        })
        .accounts({
          role: anotherWalletRole,
          solGatewayFile: filePDA,
          solGatewayRole: allowedWalletRole,
          solGatewayRule: null,
          solGatewayToken: null,
          solGatewayMetadata: null,
          solGatewaySeed: allowedWalletSeedPDA,
          contributor: ALLOWED_WALLET.publicKey,
        })
        .signers([ALLOWED_WALLET])
        .rpc();
      throw Error("Unauthorized wallets shouldn't be allowed to assign roles!");
    } catch (e) {
      if (!e.hasOwnProperty("error")) {
        throw e;
      }
      expect(e.error.errorCode.code).to.equal("Unauthorized");
    }
  });

  it("Wallet allowed to assign role", async () => {
    let rulePDA = await rule_pda(
      WRITE_PERM.role,
      "Wallet",
      WRITE_PERM.role,
      namespaces.AssignRole
    );
    // Allow role "Authenticated" to assign the same role "Authenticated" to other wallets
    await PROGRAM.methods
      .addRule({
        namespace: namespaces.AssignRole,
        role: WRITE_PERM.role,
        resource: "Wallet",
        permission: WRITE_PERM.role,
        expiresAt: null,
      })
      .accounts({
        rule: rulePDA,
        solGatewayFile: filePDA,
        solGatewayRole: null,
        solGatewayRule: null,
        solGatewayRule2: null,
        solGatewayToken: null,
        solGatewayMetadata: null,
        solGatewaySeed: null,
      })
      .rpc();

    // Assign role "Authenticated" to another wallet
    await PROGRAM.methods
      .assignRole({
        role: WRITE_PERM.role,
        address: ANOTHER_WALLET.publicKey,
        addressType: addressType.Wallet,
        expiresAt: null,
      })
      .accounts({
        role: anotherWalletRole,
        solGatewayFile: filePDA,
        solGatewayRole: allowedWalletRole,
        solGatewayRule: rulePDA,
        solGatewayToken: null,
        solGatewayMetadata: null,
        solGatewaySeed: allowedWalletSeedPDA,
        contributor: ALLOWED_WALLET.publicKey,
      })
      .signers([ALLOWED_WALLET])
      .rpc();

    // Verify the new wallet is finally allowed to write
    await PROGRAM.methods
      .allowed({
        fileId: FILE_ID,
        namespace: namespaces.Rule,
        resource: WRITE_PERM.resource,
        permission: WRITE_PERM.permission,
      })
      .accounts({
        solGatewayFile: filePDA,
        solGatewayRole: anotherWalletRole,
        solGatewayRule: writeRulePDA,
        solGatewayToken: null,
        solGatewayMetadata: null,
        solGatewaySeed: anotherWalletSeedPDA,
        signer: ANOTHER_WALLET.publicKey,
      })
      .signers([ANOTHER_WALLET])
      .rpc();
  });
});
