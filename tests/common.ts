import * as anchor from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { FILE_ID, METADATA_PROGRAM_ID, PROGRAM, PROVIDER } from "./constants";

export async function file_pda(fileId: PublicKey = FILE_ID) {
  return (
    await PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("file"), fileId.toBuffer()],
      PROGRAM.programId
    )
  )[0];
}

export async function metadata_pda(fileId: PublicKey) {
  return (
    await PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("metadata"), fileId.toBuffer()],
      PROGRAM.programId
    )
  )[0];
}

export async function rule_pda(
  role,
  resource,
  permission,
  namespace: number = 0
) {
  return (
    await PublicKey.findProgramAddressSync(
      [
        new Uint8Array([namespace]),
        anchor.utils.bytes.utf8.encode(role),
        anchor.utils.bytes.utf8.encode(resource),
        anchor.utils.bytes.utf8.encode(permission),
        FILE_ID.toBuffer(),
      ],
      PROGRAM.programId
    )
  )[0];
}

export async function seed_pda(signer: PublicKey) {
  return (
    await PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("seed"), signer.toBuffer()],
      PROGRAM.programId
    )
  )[0];
}

/**
 *  Empty Addresses are considered wildcards "*" (role will be applied to all users)
 */
export async function role_pda(role, address: PublicKey | null) {
  return (
    await PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode(role),
        address ? address.toBuffer() : anchor.utils.bytes.utf8.encode("*"),
        FILE_ID.toBuffer(),
      ],
      PROGRAM.programId
    )
  )[0];
}

export async function safe_airdrop(
  connection: anchor.web3.Connection,
  destination: anchor.web3.PublicKey,
  lamports = 100_000_000
) {
  // Maximum amount of Lamports per transaction (Devnet allows up to 2SOL per transaction)
  const maxSolPerTx = 2_000_000_000;
  let balance = await connection.getBalance(destination);
  while (balance < lamports) {
    try {
      const latestBlockHash = await connection.getLatestBlockhash();
      // Request Airdrop for user
      await connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: await connection.requestAirdrop(
          destination,
          Math.min(lamports - balance, maxSolPerTx)
        ),
      });
      balance = await connection.getBalance(destination);
    } catch {}
  }
}

export const WRITE_PERM = {
  role: "Authenticated",
  resource: "Homepage",
  permission: "Write",
};

export const READ_PERM = {
  role: "Anonymous",
  resource: "*",
  permission: "Read",
};

export async function tx_size(
  tx: anchor.web3.Transaction,
  signer: anchor.web3.Keypair
) {
  tx.feePayer = signer.publicKey;
  tx.recentBlockhash = (
    await PROVIDER.connection.getLatestBlockhash()
  ).blockhash;
  tx.sign(signer);
  return `Transaction size: ${tx.serialize().length} bytes`;
}
