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
  getPublicKey,
} from "./utils";

const mintToken = async () => {
    
  const connection = new Connection("http://localhost:8899", "confirmed");
  // const connection = new Connection("https://api.devnet.solana.com", "confirmed");

  const pcKeypair = getKeypair("id");
  console.log("Requesting SOL for Bob...");
  await connection.requestAirdrop(pcKeypair.publicKey, LAMPORTS_PER_SOL * 100);
  const ata_pubkey = new PublicKey("GKpwQ5rxcbUTjmxvN1PEsK49eEc9STYCb7ZkU9MC6aHF");

  const splitProgramId = getProgramId();
  const mint_pubkey = new PublicKey("5y6MtEwF2NQxxgxd48xP6kMmrvQXfDx5gCr8fQmsa5d4");

  const wallet1_pubkey = getPublicKey("wallet1");
  const ata1_pubkey = new PublicKey("7e3MpJsu8gg7GPVKfgMSxm9r1jeoiGBcb9UqMWHwFn2n");

  const wallet2_pubkey = getPublicKey("wallet2");
  const ata2_pubkey = new PublicKey("DSNZ4b6HaA5U3tRucu2er4ACf5kgoPYE7JVuYs7yC6Xu");

  const wallet3_pubkey = getPublicKey("wallet3");
  const ata3_pubkey = new PublicKey("65LeMTf2Pb1Ub8bpLJnqFBUjZoE12UypH7XsNt7HeWy4");

  const wallet4_pubkey = getPublicKey("wallet4");
  const ata4_pubkey = new PublicKey("31GomYLLkwyfK9LwE92N3R8FvKoCBGr24b2n4DorBUc3");
 

  const mintTokenIx = new TransactionInstruction({
    programId: splitProgramId,
    keys: [
      { pubkey: pcKeypair.publicKey, isSigner: true, isWritable: false },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },

      { pubkey: mint_pubkey, isSigner: false, isWritable: true },
      { pubkey: ata_pubkey, isSigner: false, isWritable: true },

      { pubkey: wallet1_pubkey, isSigner: false, isWritable: true },
      { pubkey: ata1_pubkey, isSigner: false, isWritable: true },
      { pubkey: wallet2_pubkey, isSigner: false, isWritable: true },
      { pubkey: ata2_pubkey, isSigner: false, isWritable: true },
      { pubkey: wallet3_pubkey, isSigner: false, isWritable: true },
      { pubkey: ata3_pubkey, isSigner: false, isWritable: true },
      { pubkey: wallet4_pubkey, isSigner: false, isWritable: true },
      { pubkey: ata4_pubkey, isSigner: false, isWritable: true },

      { pubkey: SystemProgram.programId, isSigner:false, isWritable:false }
    ],
    data: Buffer.from(
        Uint8Array.of(
          0,  // mint Token - fee split
          ...new BN(100000).toArray("le", 8),     //token amount
          0,
          0,
          0,
          0,
          0,
        )
    ),
  });

  const tx = new Transaction().add(
    mintTokenIx
  );

  console.log(tx);
  console.log("Sending Mint transaction...");
  await connection.sendTransaction(
    tx,
    [pcKeypair],
    { skipPreflight: false, preflightCommitment: "confirmed" }
  );
  
  console.log("mint completed!");
};

mintToken();