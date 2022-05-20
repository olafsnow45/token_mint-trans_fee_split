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

  const pcKeypair = getKeypair("id");

  // const connection = new Connection("http://localhost:8899", "confirmed");
  // console.log("Requesting SOL for Bob...");
  // await connection.requestAirdrop(pcKeypair.publicKey, LAMPORTS_PER_SOL * 100);
  const connection = new Connection("https://api.devnet.solana.com", "confirmed");

  const splitProgramId = getProgramId();
  const mint_pubkey = new PublicKey("4yx2gD7heLLTHGLN4CGPCaGYsCRfMnadcq8nLm1aWXDk");

  //----- Mint Auth ATA
  const ata_pubkey = new PublicKey("7vC7aiFev4CbR7tXdF8XoAJVmsKJJkrRF8XQLv45v9hs");

  //----- Public Wallet   50%
  const wallet1_pubkey = new PublicKey("AABNCe2Qe7ot33jf2kAfhHJ2tgzLg4RT6DfcxYz82GSq");
  const ata1_pubkey = new PublicKey("AKLskMXxyG5XLudrVuJR8rdQLNqbbFFbDwsMBXWbT5DY");

  //----- Founders Wallet 20%
  const wallet2_pubkey = new PublicKey("EAHu7zYNsK88wp14cwhpUKd2bMrt8qhcGkpVzks5FL2r");
  const ata2_pubkey = new PublicKey("CdLbsLMLwWM92eWzQp6AFR9fCn6CzygMskeA4BzCo2RB");

  //----- Reserve Wallet  20%
  const wallet3_pubkey = new PublicKey("AVgQpyz4YmnhXH3KnzSBSeaT2w7dUq1aJFumstCYKdMy");
  const ata3_pubkey = new PublicKey("CtSUc6nTFsgLQsNzyJkwczJCzkZjYtTRLRKEEbeJVu5P");

  //----- Team Wallet     10%
  const wallet4_pubkey = new PublicKey("FjJVM5knq46rr3MxSsDFNARKp6YDSqhS1DcKm6APuPyW");
  const ata4_pubkey = new PublicKey("7Tvwrh1P7fgfJTnx9TngWwrwBXadRspmhy63L8ZxRHas");
 

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