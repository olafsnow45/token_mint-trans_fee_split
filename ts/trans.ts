import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import {
  Connection,
  PublicKey,
  SystemProgram,
  LAMPORTS_PER_SOL,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";

import BN = require("bn.js");

import {
  getKeypair,
  getProgramId,
} from "./utils";

const transToken = async () => {
    
  const connection = new Connection("http://localhost:8899", "confirmed");
  // const connection = new Connection("https://api.devnet.solana.com", "confirmed");

  const splitProgramId = getProgramId();
  const mint_pubkey = new PublicKey("5y6MtEwF2NQxxgxd48xP6kMmrvQXfDx5gCr8fQmsa5d4");


  const src_keyPair = getKeypair("wallet1");
  const ata1_pubkey = new PublicKey("7e3MpJsu8gg7GPVKfgMSxm9r1jeoiGBcb9UqMWHwFn2n");

  console.log("Requesting SOL for Source Wallet...");
  await connection.requestAirdrop(src_keyPair.publicKey, LAMPORTS_PER_SOL * 100);

  const dest_keyPair = getKeypair("wallet4");
  const ata2_pubkey = new PublicKey("31GomYLLkwyfK9LwE92N3R8FvKoCBGr24b2n4DorBUc3");


  const fee_ata1_pubkey = new PublicKey("GY8XNRa6SroCXmMEjfTCpFJ4HGcRaVduhySFdbUXNWgG");
  const fee1 = 3
  const fee_ata2_pubkey = new PublicKey("2MZ5tky2GPiZeMGTyFg8HXdaBb638eTYMenTQExHcBsa");
  const fee2 = 3
  const fee_ata3_pubkey = new PublicKey("CAyQ6MyZKkzcrkYF2BYKRJHcqSkJAiEPfZWmmMkW3AUw");
  const fee3 = 2
  const fee_ata4_pubkey = new PublicKey("4rKGNZ6uubxJApNVTQ5vZbMmgkZRRPT6ExfcEcBEVY8m");
  const fee4 = 2
  const fee_ata5_pubkey = new PublicKey("AdhKyNeuASxhmgwNVGMZnybpPfPoPdA13ZiTqzsnns6h");
  const fee5 = 1
  
  const transTokenIx = new TransactionInstruction({
    programId: splitProgramId,
    keys: [
      { pubkey: src_keyPair.publicKey, isSigner: true, isWritable: false },
      { pubkey: dest_keyPair.publicKey, isSigner: false, isWritable: false },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },

      { pubkey: ata1_pubkey, isSigner: false, isWritable: true },
      { pubkey: ata2_pubkey, isSigner: false, isWritable: true },

      { pubkey: fee_ata1_pubkey, isSigner: false, isWritable: true },
      { pubkey: fee_ata2_pubkey, isSigner: false, isWritable: true },
      { pubkey: fee_ata3_pubkey, isSigner: false, isWritable: true },
      { pubkey: fee_ata4_pubkey, isSigner: false, isWritable: true },
      { pubkey: fee_ata5_pubkey, isSigner: false, isWritable: true },

      { pubkey: SystemProgram.programId, isSigner:false, isWritable:false }
    ],
    data: Buffer.from(
        Uint8Array.of(
          1,  // mint Token - fee split
          ...new BN(1000).toArray("le", 8),     //token amount
          fee1,
          fee2,
          fee3,
          fee4,
          fee5,
        )
    ),
  });

  const tx = new Transaction().add(
    transTokenIx
  );

  console.log(tx);
  console.log("Sending Mint transaction...");
  await connection.sendTransaction(
    tx,
    [src_keyPair, dest_keyPair],
    { skipPreflight: false, preflightCommitment: "confirmed" }
  );
  
  console.log("trans token completed!");
};

transToken();