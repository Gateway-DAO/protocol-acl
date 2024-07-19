import { file_pda, role_pda, WRITE_PERM, rule_pda, seed_pda } from "./common";
import { PROGRAM, ALLOWED_WALLET, namespaces } from "./constants";
import { expect } from "chai";
import { PublicKey } from "@metaplex-foundation/js";

describe("6.- Allow add rules", () => {
  let filePDA: PublicKey | null = null; // Populated on before() block
  let allowedWalletRolePDA = null; // Populated on before() block
  let newRulePDA: PublicKey | null = null; // Populated on before() block
  let nsRoleRulePDA: PublicKey | null = null; // Populated on before() block
  let resourcePermRulePDA: PublicKey | null = null; // Populated on before() block
  let walletSeedPDA = null; // Populated on before() block
  const newResource = "MyNewResource";
  const allPerms = "*";

  before(async () => {
    filePDA = await file_pda();
    allowedWalletRolePDA = await role_pda(
      WRITE_PERM.role,
      ALLOWED_WALLET.publicKey
    );
    newRulePDA = await rule_pda(WRITE_PERM.role, newResource, allPerms);
    nsRoleRulePDA = await rule_pda(
      WRITE_PERM.role,
      `${namespaces.Rule}`,
      WRITE_PERM.role,
      namespaces.AddRuleNSRole
    );
    resourcePermRulePDA = await rule_pda(
      WRITE_PERM.role,
      newResource,
      allPerms,
      namespaces.AddRuleResourcePerm
    );
    walletSeedPDA = await seed_pda(ALLOWED_WALLET.publicKey);
  });

  it("Wallet not allowed to add rule", async () => {
    try {
      await PROGRAM.methods
        .addRule({
          namespace: namespaces.Rule,
          role: WRITE_PERM.role,
          resource: newResource,
          permission: allPerms,
          expiresAt: null,
        })
        .accounts({
          rule: newRulePDA,
          solGatewayFile: filePDA,
          solGatewayRole: allowedWalletRolePDA,
          solGatewayRule: null,
          solGatewayRule2: null,
          solGatewayToken: null,
          solGatewayMetadata: null,
          solGatewaySeed: walletSeedPDA,
          signer: ALLOWED_WALLET.publicKey,
        })
        .signers([ALLOWED_WALLET])
        .rpc();
      throw Error("Unauthorized wallets shouldn't be allowed to create rules!");
    } catch (e) {
      if (!e.hasOwnProperty("error")) {
        throw e;
      }
      expect(e.error.errorCode.code).to.equal("Unauthorized");
    }
  });

  it("Add rule to allow creation of Namespace and Role", async () => {
    // Allows the role "Authenticated" to create following permission:
    // - Role:  "Authenticated" (The role receiving the permission)
    // - Namespace: Rule (The kind of namespace of the permission)
    // - Roles of type: "Authenticated" (The role to which the permission could be applied)
    await PROGRAM.methods
      .addRule({
        namespace: namespaces.AddRuleNSRole,
        role: WRITE_PERM.role,
        resource: `${namespaces.Rule}`,
        permission: WRITE_PERM.role,
        expiresAt: null,
      })
      .accounts({
        rule: nsRoleRulePDA,
        solGatewayFile: filePDA,
        solGatewayRole: null,
        solGatewayRule: null,
        solGatewayRule2: null,
        solGatewayToken: null,
        solGatewayMetadata: null,
        solGatewaySeed: null,
      })
      .rpc();
  });

  it("Add rule to allow creation of Resource and Permission", async () => {
    // Allows the role "Authenticated" to create following permission:
    // - Role:  "Authenticated" (The role receiving the permission)
    // - Resource:  "MyNewResource" (Resource to which will be applied the permission)
    // - Permission: "*" (Wildcard, allowed to create any permission on this resource)
    await PROGRAM.methods
      .addRule({
        namespace: namespaces.AddRuleResourcePerm,
        role: WRITE_PERM.role,
        resource: newResource,
        permission: allPerms,
        expiresAt: null,
      })
      .accounts({
        rule: resourcePermRulePDA,
        solGatewayFile: filePDA,
        solGatewayRole: null,
        solGatewayRule: null,
        solGatewayRule2: null,
        solGatewayToken: null,
        solGatewayMetadata: null,
        solGatewaySeed: null,
      })
      .rpc();
  });

  it("Wallet can create rule for allowed resource", async () => {
    // Allows the role "Authenticated" to create following permission:
    // - Role:  "Authenticated" (The role receiving the permission)
    // - Resource:  "MyNewResource" (Resource to which will be applied the permission)
    // - Permission: "Add"
    await PROGRAM.methods
      .addRule({
        namespace: namespaces.Rule,
        role: WRITE_PERM.role,
        resource: newResource,
        permission: "Add",
        expiresAt: null,
      })
      .accounts({
        rule: await rule_pda(
          WRITE_PERM.role,
          newResource,
          "Add",
          namespaces.Rule
        ),
        solGatewayFile: filePDA,
        solGatewayRole: allowedWalletRolePDA,
        solGatewayRule: nsRoleRulePDA,
        solGatewayRule2: resourcePermRulePDA,
        solGatewayToken: null,
        solGatewayMetadata: null,
        solGatewaySeed: walletSeedPDA,
        signer: ALLOWED_WALLET.publicKey,
      })
      .signers([ALLOWED_WALLET])
      .rpc();
  });

  it("Wallet cannot create rule for other resources", async () => {
    try {
      await PROGRAM.methods
        .addRule({
          namespace: namespaces.Rule,
          role: WRITE_PERM.role,
          resource: WRITE_PERM.resource,
          permission: "Add",
          expiresAt: null,
        })
        .accounts({
          rule: await rule_pda(
            WRITE_PERM.role,
            WRITE_PERM.resource,
            "Add",
            namespaces.Rule
          ),
          solGatewayFile: filePDA,
          solGatewayRole: allowedWalletRolePDA,
          solGatewayRule: nsRoleRulePDA,
          solGatewayRule2: resourcePermRulePDA,
          solGatewayToken: null,
          solGatewayMetadata: null,
          solGatewaySeed: walletSeedPDA,
          signer: ALLOWED_WALLET.publicKey,
        })
        .signers([ALLOWED_WALLET])
        .rpc();
      throw Error(
        "Wallets shouldn't be allowed to create rules on other resources!"
      );
    } catch (e) {
      if (!e.hasOwnProperty("error")) {
        throw e;
      }
      expect(e.error.errorCode.code).to.equal("Unauthorized");
    }
  });
});
