import {
  file_pda,
  role_pda,
  WRITE_PERM,
  rule_pda,
  READ_PERM,
  seed_pda,
} from "./common";
import {
  FILE_ID,
  PROGRAM,
  PROVIDER,
  ALLOWED_WALLET,
  namespaces,
  FEE,
} from "./constants";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import { burnChecked } from "@solana/spl-token";
import { expect } from "chai";
import { BN } from "bn.js";
import { Transaction } from "@solana/web3.js";

describe("4.- Check permissions", () => {
  let filePDA = null; // Populated on before() block
  let writeRulePDA = null; // Populated on before() block
  let readRulePDA = null; // Populated on before() block
  let walletSeedPDA = null; // Populated on before() block
  

  before(async () => {
    filePDA = await file_pda();
    writeRulePDA = await rule_pda(
      WRITE_PERM.role,
      WRITE_PERM.resource,
      WRITE_PERM.permission
    );
    readRulePDA = await rule_pda(
      READ_PERM.role,
      READ_PERM.resource,
      READ_PERM.permission
    );
    walletSeedPDA = await seed_pda(ALLOWED_WALLET.publicKey);
   
  });

  it("Check allowed Authority", async () => {
    const before_balance = await PROVIDER.connection.getBalance(
      PROVIDER.publicKey
    );
    // Allowed to Write
    const ix = PROGRAM.methods
      .allowed({
        fileId: FILE_ID,
        namespace: namespaces.Rule,
        resource: WRITE_PERM.resource,
        permission: WRITE_PERM.permission,
      })
      .accounts({
        solGatewayFile: filePDA,
        solGatewayRule: writeRulePDA,
        solGatewayRole: null,
        solGatewayToken: null,
        solGatewayMetadata: null,
        solGatewaySeed: null,
      });
    const recentBlockhash = await PROVIDER.connection.getLatestBlockhash();
    const fee = await new Transaction({
      feePayer: PROVIDER.publicKey,
      blockhash: recentBlockhash.blockhash,
      lastValidBlockHeight: recentBlockhash.lastValidBlockHeight,
    })
      .add(await ix.instruction())
      .getEstimatedFee(PROVIDER.connection);
    await ix.rpc({ commitment: "confirmed" });
    const after_balance = await PROVIDER.connection.getBalance(
      PROVIDER.publicKey
    );
    // Only Transaction fee was taken because authority pays non program fees
    expect(
      new BN(before_balance.toString())
        .sub(new BN(after_balance.toString()))
        .toNumber()
    ).to.equal(fee);
  });

  it("Check allowed wallet", async () => {
    const rolePDA = await role_pda(WRITE_PERM.role, ALLOWED_WALLET.publicKey);
    const before_balance = await PROVIDER.connection.getBalance(
      ALLOWED_WALLET.publicKey
    );
    const ix = PROGRAM.methods
      .allowed({
        fileId: FILE_ID,
        namespace: namespaces.Rule,
        resource: WRITE_PERM.resource,
        permission: WRITE_PERM.permission,
      })
      .accounts({
        solGatewayFile: filePDA,
        solGatewayRole: rolePDA,
        solGatewayRule: writeRulePDA,
        solGatewayToken: null,
        solGatewayMetadata: null,
        solGatewaySeed: walletSeedPDA,
        signer: ALLOWED_WALLET.publicKey,
      })
      .signers([ALLOWED_WALLET]);
    const recentBlockhash = await PROVIDER.connection.getLatestBlockhash();
    const rent_exemption_price =
      await PROVIDER.connection.getMinimumBalanceForRentExemption(9);
    const tx_fee = await new Transaction({
      feePayer: ALLOWED_WALLET.publicKey,
      blockhash: recentBlockhash.blockhash,
      lastValidBlockHeight: recentBlockhash.lastValidBlockHeight,
    })
      .add(await ix.instruction())
      .getEstimatedFee(PROVIDER.connection);
    await ix.rpc({ commitment: "confirmed" });
    const after_balance = await PROVIDER.connection.getBalance(
      ALLOWED_WALLET.publicKey
    );

    // First "Allowed" check only charges the Rent exemption for the created "Seed account"
    // Note that the transaction fee is payed by the anchor Provider.
    expect(
      new BN(before_balance.toString())
        .sub(new BN(after_balance.toString()))
        .toNumber()
    ).to.equal(rent_exemption_price);

    // Allowed to Read (Applied to all via wildcard)
    await PROGRAM.methods
      .allowed({
        fileId: FILE_ID,
        namespace: namespaces.Rule,
        resource: READ_PERM.resource,
        permission: READ_PERM.permission,
      })
      .accounts({
        solGatewayFile: filePDA,
        solGatewayRole: await role_pda(READ_PERM.role, null), // Null address represents the wildcard "*"
        solGatewayRule: readRulePDA,
        solGatewayToken: null,
        solGatewayMetadata: null,
        solGatewaySeed: walletSeedPDA,
        signer: ALLOWED_WALLET.publicKey,
      })
      .signers([ALLOWED_WALLET])
      .rpc();
    const last_balance = await PROVIDER.connection.getBalance(
      ALLOWED_WALLET.publicKey
    );

    // Posterior checks only cost the Program fee.
    // Note that the transaction fee is payed by the anchor Provider.
    expect(
      new BN(after_balance.toString())
        .sub(new BN(last_balance.toString()))
        .toNumber()
    ).to.equal(FEE);
  });
});
