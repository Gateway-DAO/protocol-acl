import { BN } from "bn.js";
import { expect } from "chai";
import { file_pda, role_pda, WRITE_PERM, READ_PERM } from "./common";
import {
  addressType,
  FILE_ID,
  PROGRAM,
  PROVIDER,
  ALLOWED_WALLET,
  ANOTHER_WALLET,
} from "./constants";
import * as anchor from "@project-serum/anchor";

describe("3.- Assign roles", () => {
  let filePDA = null; // Populated on before() block

  before(async () => {
    filePDA = await file_pda();
  });

  it("Assign role to File", async () => {
    const rolePDA = await role_pda(WRITE_PERM.role, PROVIDER.wallet.publicKey);
    const oneHourLater = Math.floor(new Date().getTime() / 1000) + 60 * 60;
    let listener = null;
    let [event, _]: any = await new Promise((resolve, reject) => {
      listener = PROGRAM.addEventListener("RolesChanged", (event, slot) => {
        PROGRAM.removeEventListener(listener);
        resolve([event, slot]);
      });
      PROGRAM.methods
        .assignRole({
          address: PROVIDER.wallet.publicKey,
          role: WRITE_PERM.role,
          addressType: addressType.Wallet,
          expiresAt: new BN(oneHourLater),
        })
        .accounts({
          role: rolePDA,
          solGatewayFile: filePDA,
          solGatewayRole: null,
          solGatewayRule: null,
          solGatewayToken: null,
          solGatewayMetadata: null,
          solGatewaySeed: null,
        })
        .rpc();
      setTimeout(() => {
        reject(new Error("Failed to assign role"));
      }, 2000);
    });

    const role = await PROGRAM.account.role.fetch(rolePDA);
    expect(FILE_ID.toBase58()).to.equal(event.fileId.toBase58());

    expect(role.role).to.equal(WRITE_PERM.role);
    expect(role.addressType).to.deep.equal(addressType.Wallet);
    expect(role.expiresAt.toNumber()).to.equal(oneHourLater);
  });

  // TODO: fix this test
  it("Assign role to Wallet", async () => {
    const rolePDA = await role_pda(WRITE_PERM.role, ALLOWED_WALLET.publicKey);
    await PROGRAM.methods
      .assignRole({
        address: ALLOWED_WALLET.publicKey,
        role: WRITE_PERM.role,
        addressType: addressType.Wallet,
        expiresAt: null,
      })
      .accounts({
        role: rolePDA,
        solGatewayFile: filePDA,
        solGatewayRole: null,
        solGatewayRule: null,
        solGatewayToken: null,
        solGatewayMetadata: null,
        solGatewaySeed: null,
      })
      .rpc();
  });

  it("Assign role to Wallet (w/ different rent payer)", async () => {
    let listener = PROGRAM.addEventListener("RolesChanged", (event, slot) => {
      expect(FILE_ID.toBase58()).to.equal(event.fileId.toBase58());
      PROGRAM.removeEventListener(listener);
    });

    const permissionedWallet = anchor.web3.Keypair.generate();
    const rolePDA = await role_pda(
      WRITE_PERM.role,
      permissionedWallet.publicKey
    );
    const oneHourLater = Math.floor(Date.now() / 1000) + 60 * 60;

    let tx = await PROGRAM.methods
      .assignRole({
        address: permissionedWallet.publicKey,
        role: WRITE_PERM.role,
        addressType: addressType.Wallet,
        expiresAt: new BN(oneHourLater),
      })
      .accounts({
        role: rolePDA,
        solGatewayFile: filePDA,
        solGatewayRole: null,
        solGatewayRule: null,
        solGatewayToken: null,
        solGatewayMetadata: null,
        solGatewaySeed: null,
        rentPayer: ANOTHER_WALLET.publicKey,
      })
      .signers([ANOTHER_WALLET])
      .transaction();

    // Implement manual fee payer and transaction processing (encapsulated by Anchor's implementation by default)
    tx.feePayer = ANOTHER_WALLET.publicKey;
    tx.recentBlockhash = (
      await PROVIDER.connection.getLatestBlockhash()
    ).blockhash;
    tx.partialSign(ANOTHER_WALLET);

    tx = await PROVIDER.wallet.signTransaction(tx);
    const rawTx = tx.serialize();

    // Send transaction and confirm
    // TODO: Check if this is the correct way to send a raw transaction
    const txResponse = await PROVIDER.connection.sendRawTransaction(rawTx);
    await PROVIDER.connection.confirmTransaction(txResponse, "confirmed");

    const role = await PROGRAM.account.role.fetch(rolePDA);

    expect(role.fileId.toBase58()).to.equal(FILE_ID.toBase58());
    expect(role.role).to.equal(WRITE_PERM.role);
    expect(role.addressType).to.deep.equal(addressType.Wallet);
    expect(role.expiresAt.toNumber()).to.equal(oneHourLater);
    expect(role.address.toBase58()).to.equal(
      permissionedWallet.publicKey.toBase58()
    );
  });

  // TODO: remove this possibility
  it("Assign role to All", async () => {
    const rolePDA = await role_pda(READ_PERM.role, null);
    await PROGRAM.methods
      .assignRole({
        address: null,
        role: READ_PERM.role,
        addressType: addressType.Wallet,
        expiresAt: null,
      })
      .accounts({
        role: rolePDA,
        solGatewayFile: filePDA,
        solGatewayRole: null,
        solGatewayRule: null,
        solGatewayToken: null,
        solGatewayMetadata: null,
        solGatewaySeed: null,
      })
      .rpc();
  });
});
