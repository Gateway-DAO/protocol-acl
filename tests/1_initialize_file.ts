import * as anchor from "@project-serum/anchor";
import { expect, assert } from "chai";
import { file_pda, safe_airdrop, metadata_pda } from "./common";
import {
  FILE_ID,
  PROVIDER,
  RECOVERY_KEYPAIR,
  PROGRAM,
  ALLOWED_WALLET,
  ANOTHER_WALLET,
  accountTypes,
} from "./constants";

describe("1.- Initialize FILE and Metadata", () => {
  let filePDA = null;
  let metadataPDA = null;
  const unauthorized_keypair = anchor.web3.Keypair.generate();

  before(async () => {
    filePDA = await file_pda();
    metadataPDA = await metadata_pda(FILE_ID);

    // Airdrop to wallets
    await Promise.all([
      safe_airdrop(
        PROVIDER.connection,
        ALLOWED_WALLET.publicKey,
        2 * anchor.web3.LAMPORTS_PER_SOL
      ),
      safe_airdrop(
        PROVIDER.connection,
        ANOTHER_WALLET.publicKey,
        2 * anchor.web3.LAMPORTS_PER_SOL
      ),
      safe_airdrop(
        PROVIDER.connection,
        PROVIDER.wallet.publicKey,
        2 * anchor.web3.LAMPORTS_PER_SOL
      ),
    ]);
  });

  it("Init file without metadata", async () => {
    const fileName = "file1";

    const tx = await PROGRAM.methods
      .initializeFiles({
        id: FILE_ID,
        recovery: new anchor.web3.PublicKey(
          "CeAr18Canypu3Fmsn6stce12JriVGzRDHNFSW3Nzx9vk"
        ),
        name: fileName,
        cached: false,
        size: new anchor.BN(1073741824), // 1 GB
        checksum: "351101afcc166d0be1299d55bdfa61a4",
        metadata: null,
        expiresAt: new anchor.BN(Math.floor(Date.now() / 1000) + 31536000), // Add this line
      })
      .accounts({
        file: filePDA,
        fileMetadata: null, // Note: here we NEED to pass null or else the program will try to create a metadata account
      })
      .rpc();

    let file = await PROGRAM.account.file.fetch(filePDA);

    expect(file.id.toBase58()).to.equal(FILE_ID.toBase58());
    expect(file.authority.toBase58()).to.equal(
      PROVIDER.wallet.publicKey.toBase58()
    );
    expect(file.name).to.equal(fileName);

    // Verify metadata account doesn't exist
    try {
      await PROGRAM.account.fileMetadata.fetch(metadataPDA);
      assert.fail("Metadata account should not exist");
    } catch (error) {
      expect(error.toString()).to.include(
        "Account does not exist or has no data"
      );
    }
  });

  it("Init file with metadata", async () => {
    const newFileId = anchor.web3.Keypair.generate().publicKey;
    const newFilePDA = await file_pda(newFileId);
    const newMetadataPDA = await metadata_pda(newFileId);
    const fileName = "file2";

    const tx = await PROGRAM.methods
      .initializeFiles({
        id: newFileId,
        recovery: new anchor.web3.PublicKey(
          "CeAr18Canypu3Fmsn6stce12JriVGzRDHNFSW3Nzx9vk"
        ),
        name: fileName,
        cached: false,
        size: new anchor.BN(1073741824), // 1 GB
        checksum: "351101afcc166d0be1299d55bdfa61a4",
        metadata: [
          { key: "author", value: "John Doe" },
          { key: "version", value: "1.0" },
        ],
        expiresAt: new anchor.BN(Math.floor(Date.now() / 1000) + 31536000), // Add this line
      })
      .accounts({
        file: newFilePDA,
        fileMetadata: newMetadataPDA,
      })
      .rpc();

    let file = await PROGRAM.account.file.fetch(newFilePDA);
    let metadata = await PROGRAM.account.fileMetadata.fetch(newMetadataPDA);

    expect(file.id.toBase58()).to.equal(newFileId.toBase58());
    expect(file.authority.toBase58()).to.equal(
      PROVIDER.wallet.publicKey.toBase58()
    );
    expect(file.name).to.equal(fileName);

    expect(metadata.fileId.toBase58()).to.equal(newFileId.toBase58());
    expect(metadata.metadata).to.deep.equal([
      { key: "author", value: "John Doe" },
      { key: "version", value: "1.0" },
    ]);
  });

  it("Init file with different rent payer", async () => {
    const newFileId = anchor.web3.Keypair.generate().publicKey;
    const newFilePDA = await file_pda(newFileId);

    const fileName = "file3";

    let tx = await PROGRAM.methods
      .initializeFiles({
        id: newFileId,
        recovery: RECOVERY_KEYPAIR.publicKey,
        name: fileName,
        cached: false,
        size: new anchor.BN(1073741824), // 1 GB
        checksum: "351101afcc166d0be1299d55bdfa61a4",
        metadata: null,
        expiresAt: new anchor.BN(Math.floor(Date.now() / 1000) + 31536000),
      })
      .accounts({
        file: newFilePDA,
        fileMetadata: null,
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

    let file = await PROGRAM.account.file.fetch(newFilePDA);

    expect(file.id.toBase58()).to.equal(newFileId.toBase58());
    expect(file.authority.toBase58()).to.equal(
      PROVIDER.wallet.publicKey.toBase58()
    );
    expect(file.name).to.equal(fileName);
  });

  it("Update file authority", async () => {
    try {
      await PROGRAM.methods
        .updateFile({
          authority: unauthorized_keypair.publicKey,
          recovery: RECOVERY_KEYPAIR.publicKey,
          name: "file1",
          cached: false,
          fee: null,
          size: new anchor.BN(564), // 564 bytes
          checksum: "351101afcc166d0be1299d55bdfa61a4",
          accountType: accountTypes.Basic,
          expiresAt: new anchor.BN(Math.floor(Date.now() / 1000) + 31536000),
        })
        .accounts({
          file: filePDA,
          signer: unauthorized_keypair.publicKey,
        })
        .signers([unauthorized_keypair])
        .rpc();
      assert.fail(
        "Unauthorized users shouldn't be able to update File authority!"
      );
    } catch (error) {
      expect(error.toString()).to.include("UnauthorizedAuthorityUpdate");
    }

    // Verify current Authority can update the authority of the FILE
    await PROGRAM.methods
      .updateFile({
        authority: unauthorized_keypair.publicKey,
        recovery: RECOVERY_KEYPAIR.publicKey,
        name: "file1",
        cached: true,
        fee: null,
        size: new anchor.BN(564), // 564 bytes
        checksum: "351101afcc166d0be1299d55bdfa61a4",
        accountType: accountTypes.Basic,
        expiresAt: new anchor.BN(Math.floor(Date.now() / 1000) + 31536000),
      })
      .accounts({
        file: filePDA,
      })
      .rpc();
    let file = await PROGRAM.account.file.fetch(filePDA);
    expect(file.name).to.equal("file1");
    assert.isTrue(file.cached);
    expect(file.authority.toBase58()).to.equal(
      unauthorized_keypair.publicKey.toBase58()
    );

    // Verify recovery can also update the authority of the FILE
    await PROGRAM.methods
      .updateFile({
        authority: PROVIDER.wallet.publicKey,
        recovery: RECOVERY_KEYPAIR.publicKey,
        name: "file2",
        cached: false,
        fee: null,
        size: new anchor.BN(1073741824), // 1 GB
        checksum: "351101afcc166d0be1299d55bdfa61a4",
        accountType: accountTypes.Basic,
        expiresAt: new anchor.BN(Math.floor(Date.now() / 1000) + 31536000),
      })
      .accounts({
        file: filePDA,
        signer: RECOVERY_KEYPAIR.publicKey,
      })
      .signers([RECOVERY_KEYPAIR])
      .rpc();
    file = await PROGRAM.account.file.fetch(filePDA);
    expect(file.name).to.equal("file2");
    assert.isFalse(file.cached);
    expect(file.authority.toBase58()).to.equal(
      PROVIDER.wallet.publicKey.toBase58()
    );
  });

  it("Update file metadata", async () => {
    const fileId = anchor.web3.Keypair.generate().publicKey;
    const filePDA = await file_pda(fileId);
    const metadataPDA = await metadata_pda(fileId);

    // Initialize file with metadata
    await PROGRAM.methods
      .initializeFiles({
        id: fileId,
        recovery: RECOVERY_KEYPAIR.publicKey,
        name: "file1",
        cached: false,
        size: new anchor.BN(1073741824), // 1 GB
        checksum: "351101afcc166d0be1299d55bdfa61a4",
        metadata: [
          { key: "author", value: "John Doe" },
          { key: "version", value: "1.0" },
        ],
        expiresAt: new anchor.BN(Math.floor(Date.now() / 1000) + 31536000),
      })
      .accounts({
        file: filePDA,
        fileMetadata: metadataPDA,
      })
      .rpc();

    // Update metadata
    await PROGRAM.methods
      .updateFileMetadata({
        metadata: [
          { key: "author", value: "Jane Doe" },
          { key: "version", value: "2.0" },
          { key: "status", value: "updated" },
        ],
      })
      .accounts({
        file: filePDA,
        fileMetadata: metadataPDA,
        signer: PROVIDER.wallet.publicKey,
      })
      .rpc();

    let metadata = await PROGRAM.account.fileMetadata.fetch(metadataPDA);
    expect(metadata.metadata).to.deep.equal([
      { key: "author", value: "Jane Doe" },
      { key: "version", value: "2.0" },
      { key: "status", value: "updated" },
    ]);
  });

  it("Delete file", async () => {
    const fileIdToDelete = anchor.web3.Keypair.generate().publicKey;
    const filePDAToDelete = await file_pda(fileIdToDelete);

    await PROGRAM.methods
      .initializeFiles({
        id: fileIdToDelete,
        recovery: RECOVERY_KEYPAIR.publicKey,
        name: "file_del",
        cached: false,
        size: new anchor.BN(1048576), // 1 MB
        checksum: "123456789abcdef0123456789abcdef0",
        metadata: null,
        expiresAt: new anchor.BN(Math.floor(Date.now() / 1000) + 31536000),
      })
      .accounts({
        file: filePDAToDelete,
        fileMetadata: null,
      })
      .rpc();

    let file = await PROGRAM.account.file.fetch(filePDAToDelete);
    expect(file.name).to.equal("file_del");

    await PROGRAM.methods
      .deleteFile()
      .accounts({
        file: filePDAToDelete,
        authority: PROVIDER.wallet.publicKey,
        collector: PROVIDER.wallet.publicKey,
      })
      .rpc();

    try {
      await PROGRAM.account.file.fetch(filePDAToDelete);
      assert.fail("The file was not deleted");
    } catch (erro) {
      expect(erro.message).to.include("Account does not exist");
    }
  });
});
