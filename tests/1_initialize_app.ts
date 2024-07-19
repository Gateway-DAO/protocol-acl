import * as anchor from "@project-serum/anchor";
import { expect, assert } from "chai";
import { app_pda, safe_airdrop } from "./common";
import {
  FILE_ID,
  NFTS,
  PROVIDER,
  RECOVERY_KEYPAIR,
  METAPLEX,
  PROGRAM,
  PROVIDER_WALLET,
  WALLET_WITH_NFTS,
  ALLOWED_WALLET,
  ANOTHER_WALLET,
  accountTypes,
} from "./constants";

describe("1.- Initialize APP", () => {
  let filePDA = null; // Populated on before() block
  const unauthorized_keypair = anchor.web3.Keypair.generate();

  // Create NFTs for testing access rules afterwards.
  before(async () => {
    appPDA = await app_pda();
    PROVIDER.connection.requestAirdrop(
      WALLET_WITH_NFTS.publicKey,
      1_000_000_000 // 1SOL
    );
    // Async airdrop for wallet user
    safe_airdrop(
      PROVIDER.connection,
      ALLOWED_WALLET.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL // 2 SOL
    );
    // Async airdrop for wallet with NFT user
    safe_airdrop(
      PROVIDER.connection,
      WALLET_WITH_NFTS.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL // 2 SOL
    );
    // Async airdrop for another wallet user
    safe_airdrop(
      PROVIDER.connection,
      ANOTHER_WALLET.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL // 2 SOL
    );
    await safe_airdrop(
      PROVIDER.connection,
      PROVIDER.wallet.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL // 2 SOL
    );
    const collection = await METAPLEX.nfts().create({
      name: "Collection",
      sellerFeeBasisPoints: 0,
      uri: "https://arweave.net/collection1-hash",
      isMutable: true,
      isCollection: true,
    });
    NFTS["allowedNFT"] = await METAPLEX.nfts().create({
      name: "Allowed NFT",
      sellerFeeBasisPoints: 0,
      uri: "https://arweave.net/nft1-hash",
      tokenOwner: WALLET_WITH_NFTS.publicKey,
      isMutable: true,
    });
    NFTS["allowedCollection"] = await METAPLEX.nfts().create({
      name: "Allowed collection",
      sellerFeeBasisPoints: 0,
      uri: "https://arweave.net/nft2-hash",
      tokenOwner: WALLET_WITH_NFTS.publicKey,
      isMutable: true,
      collection: collection.mintAddress,
      collectionAuthority: PROVIDER_WALLET.payer, // This will set the Collection verified flag to true
    });
    NFTS["notAllowedNFT"] = await METAPLEX.nfts().create({
      name: "Not allowed NFT",
      sellerFeeBasisPoints: 0,
      uri: "https://arweave.net/nft3-hash",
      tokenOwner: WALLET_WITH_NFTS.publicKey,
      isMutable: true,
    });
  });

  it("Init", async () => {
    const fileName = "test";
    try {
      await PROGRAM.account.app.fetch(filePDA);
    } catch (_err) {
      expect(_err.toString()).to.include("Account does not exist");
    }
    const tx = await PROGRAM.methods
      .initializeFile({
        id: FILE_ID,
        recovery: RECOVERY_KEYPAIR.publicKey,
        name: fileName,
        cached: false,
      })
      .accounts({
        file: filePDA,
      })
      .rpc();
    let file = await PROGRAM.account.file.fetch(filePDA);
    expect(file.id.toBase58()).to.equal(FILE_ID.toBase58());
    expect(file.authority.toBase58()).to.equal(
      PROVIDER.wallet.publicKey.toBase58()
    );
    expect(file.name).to.equal(fileName);
  });

  it("Update authority", async () => {
    try {
      // Unauthorized users shouldn't be able to update App authority
      await PROGRAM.methods
        .updateFile({
          authority: unauthorized_keypair.publicKey,
          recovery: RECOVERY_KEYPAIR.publicKey,
          name: "myfile-recovered",
          cached: false,
          fee: null,
          accountType: accountTypes.Basic,
          expiresAt: null,
        })
        .accounts({
          app: filePDA,
          signer: unauthorized_keypair.publicKey,
        })
        .signers([unauthorized_keypair])
        .rpc();
      throw new Error(
        "Unauthorized users shouldn't be able to update File authority!"
      );
    } catch (error) {
      expect(error.error.errorCode.code).to.equal(
        "UnauthorizedAuthorityUpdate"
      );
    }

    // Verify current Authority can update the authority of the APP
    await PROGRAM.methods
      .updateFile({
        authority: unauthorized_keypair.publicKey,
        recovery: RECOVERY_KEYPAIR.publicKey,
        name: "myfile-recovered1",
        cached: true,
        fee: null,
        accountType: accountTypes.Basic,
        expiresAt: null,
      })
      .accounts({
        file: filePDA,
      })
      .rpc();
    let file = await PROGRAM.account.file.fetch(filePDA);
    expect(file.name).to.equal("myfile-recovered1");
    assert.isTrue(file.cached);
    expect(file.authority.toBase58()).to.equal(
      unauthorized_keypair.publicKey.toBase58()
    );
    // Verify recovery can also update the authority of the APP
    await PROGRAM.methods
      .updateFile({
        authority: PROVIDER.wallet.publicKey,
        recovery: RECOVERY_KEYPAIR.publicKey,
        name: "myfile-recovered2",
        cached: false,
        fee: null,
        accountType: accountTypes.Basic,
        expiresAt: null,
      })
      .accounts({
        file: filePDA,
        signer: RECOVERY_KEYPAIR.publicKey,
      })
      .signers([RECOVERY_KEYPAIR])
      .rpc();
    file = await PROGRAM.account.file.fetch(filePDA);
    expect(file.name).to.equal("myfile-recovered2");
    assert.isFalse(file.cached);
    expect(file.authority.toBase58()).to.equal(
      PROVIDER.wallet.publicKey.toBase58()
    );
  });
});
