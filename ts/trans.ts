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
    
  const splitProgramId = getProgramId();
  const mint_pubkey = new PublicKey("4yx2gD7heLLTHGLN4CGPCaGYsCRfMnadcq8nLm1aWXDk");


  //-- Mint Auth & Payer
  const src_keyPair = getKeypair("id");
  const ata1_pubkey = new PublicKey("7vC7aiFev4CbR7tXdF8XoAJVmsKJJkrRF8XQLv45v9hs");

  // const connection = new Connection("http://localhost:8899", "confirmed");
  // console.log("Requesting SOL for Source Wallet...");
  // await connection.requestAirdrop(src_keyPair.publicKey, LAMPORTS_PER_SOL * 100);

  const connection = new Connection("https://api.devnet.solana.com", "confirmed");

  const dest_pubkey = new PublicKey("FjJVM5knq46rr3MxSsDFNARKp6YDSqhS1DcKm6APuPyW");
  const ata2_pubkey = new PublicKey("7Tvwrh1P7fgfJTnx9TngWwrwBXadRspmhy63L8ZxRHas");


  //-- Marketing          3%
  const fee_ata1_pubkey = new PublicKey("3ZgyamrcUybDhvuLurtwqnkFxtuXxKAEVKAeWHNfvbz1");
  const fee1 = 3
  //-- Development        2%
  const fee_ata2_pubkey = new PublicKey("7fmKSZ3WFJ1HsNbVRPwRezZmDQggnpmPUArsSngbcPk3");
  const fee2 = 2
  //-- Liquidity Pool     2%
  const fee_ata3_pubkey = new PublicKey("GfUcEZssAT9ePfJRTkJjx9peVuduCqWW4uYheQKK9RQo");
  const fee3 = 2
  //-- Buy Back           2%
  const fee_ata4_pubkey = new PublicKey("8JRBBtvHBTZZ3F4KbyvA8ztyhHnJondjjhcaaHbqGj5y");
  const fee4 = 2
  //-- Charity            1%
  const fee_ata5_pubkey = new PublicKey("41zm3ka3y5HVj3bY2W8ZHhn4U7cSxC53ihuXUY6hotsn");
  const fee5 = 1
  
  const transTokenIx = new TransactionInstruction({
    programId: splitProgramId,
    keys: [
      { pubkey: src_keyPair.publicKey, isSigner: true, isWritable: false },
      { pubkey: dest_pubkey, isSigner: false, isWritable: false },
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
    [src_keyPair],
    { skipPreflight: false, preflightCommitment: "confirmed" }
  );
  
  console.log("trans token completed!");
};

transToken();