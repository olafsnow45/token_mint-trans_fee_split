use solana_program::{
    account_info::{next_account_info, AccountInfo },
    entrypoint::ProgramResult,
    msg,
    program::{invoke},
    pubkey::{Pubkey},
};

use spl_token::{
    instruction,
};

use crate::{error::SplitError, instruction::SplitInstruction};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Process -> Instruction");
        let instruction = SplitInstruction::unpack(instruction_data)?;

        msg!("Instruction -> Init");
        match instruction {
            SplitInstruction::MintAccount { amount } => {
                msg!("Instruction: MintAccount");
                Self::process_mint_token( accounts, amount, program_id )
            }
            SplitInstruction::TransferAccount { amount, fee1, fee2, fee3, fee4, fee5} => {
                msg!("Instruction: TransferToken");
                Self::process_transfer_token( accounts, amount, fee1, fee2, fee3, fee4, fee5, program_id )
            }
        }
    }

    fn process_mint_token(
        accounts: &[AccountInfo],
        amount: u64,
        _program_id: &Pubkey,
    ) -> ProgramResult {

        //--- Public    50%
        let wallet1_pubkey = "AABNCe2Qe7ot33jf2kAfhHJ2tgzLg4RT6DfcxYz82GSq";
        //--- Founders  20%
        let wallet2_pubkey = "EAHu7zYNsK88wp14cwhpUKd2bMrt8qhcGkpVzks5FL2r";
        //--- Reserve   20%
        let wallet3_pubkey = "AVgQpyz4YmnhXH3KnzSBSeaT2w7dUq1aJFumstCYKdMy";
        //--- Team      10%
        let wallet4_pubkey = "FjJVM5knq46rr3MxSsDFNARKp6YDSqhS1DcKm6APuPyW";

        let amount_mint = amount;
        msg!("amount_mint : {}", amount_mint);

        let account_info_iter = &mut accounts.iter();

        let pc_account = next_account_info(account_info_iter)?;
        msg!("pc_account Pubkey : {}", pc_account.key);

        let token_program = next_account_info(account_info_iter)?;
        msg!("token_program : {}", token_program.key);

        let mint_account = next_account_info(account_info_iter)?;
        msg!("Mint_account Pubkey : {}", mint_account.key);

        let ata_account = next_account_info(account_info_iter)?;
        msg!("Ata_account Pubkey : {}", ata_account.key);
        
        let wallet1_account = next_account_info(account_info_iter)?;
        if wallet1_account.key.to_string() != wallet1_pubkey  {
            msg!("Wrong Wallet1 Address");
            return Err(SplitError::InvalidWalletAddress.into());
        }        
        msg!("wallet1_account Pubkey : {}", wallet1_account.key);
        let ata1_account = next_account_info(account_info_iter)?;
        msg!("Ata1_account Pubkey : {}", ata1_account.key);

        let wallet2_account = next_account_info(account_info_iter)?;
        if wallet2_account.key.to_string() != wallet2_pubkey  {
            msg!("Wrong Wallet2 Address");
            return Err(SplitError::InvalidWalletAddress.into());
        }        
        msg!("wallet2_account Pubkey : {}", wallet2_account.key);
        let ata2_account = next_account_info(account_info_iter)?;
        msg!("Ata2_account Pubkey : {}", ata2_account.key);

        let wallet3_account = next_account_info(account_info_iter)?;
        if wallet3_account.key.to_string() != wallet3_pubkey  {
            msg!("Wrong Wallet3 Address");
            return Err(SplitError::InvalidWalletAddress.into());
        }        
        msg!("wallet3_account Pubkey : {}", wallet3_account.key);
        let ata3_account = next_account_info(account_info_iter)?;
        msg!("Ata3_account Pubkey : {}", ata3_account.key);

        let wallet4_account = next_account_info(account_info_iter)?;
        if wallet4_account.key.to_string() != wallet4_pubkey  {
            msg!("Wrong Wallet4 Address");
            return Err(SplitError::InvalidWalletAddress.into());
        }        
        msg!("wallet4_account Pubkey : {}", wallet4_account.key);
        let ata4_account = next_account_info(account_info_iter)?;
        msg!("Ata4_account Pubkey : {}", ata4_account.key);

        let system_program_account = next_account_info(account_info_iter)?;
        msg!("system_program_account Pubkey : {}", system_program_account.key );

        let mint_to_ix = instruction::mint_to(
            token_program.key, 
            mint_account.key, 
            ata_account.key, 
            pc_account.key, 
            &[&pc_account.key], 
            amount
        )
        .unwrap();

        msg!("mint_ix : {:?}", mint_to_ix );
        
        invoke(
            &mint_to_ix,
            &[
                pc_account.clone(),
                mint_account.clone(),
                ata_account.clone(),
                token_program.clone()
            ],
        )?;

        let amount1 = ((amount as f64) * 50.0 / 100.0) as u64;
        let amount2 = ((amount as f64) * 20.0 / 100.0) as u64;
        let amount3 = ((amount as f64) * 20.0 / 100.0) as u64;
        let amount4 = amount - amount1 - amount2 - amount3;

        let transfer1_ix = spl_token::instruction::transfer(
            token_program.key,
            ata_account.key,
            ata1_account.key,
            pc_account.key,
            &[&pc_account.key],
            amount1,
        )?;
        msg!("Calling the token program to transfer ATA ---> ATA1");
        invoke(
            &transfer1_ix,
            &[
                pc_account.clone(),
                ata_account.clone(),
                ata1_account.clone(),
                token_program.clone(),
            ],
        )?;

        let transfer2_ix = spl_token::instruction::transfer(
            token_program.key,
            ata_account.key,
            ata2_account.key,
            pc_account.key,
            &[&pc_account.key],
            amount2,
        )?;
        msg!("Calling the token program to transfer ATA ---> ATA2");
        invoke(
            &transfer2_ix,
            &[
                pc_account.clone(),
                ata_account.clone(),
                ata2_account.clone(),
                token_program.clone(),
            ],
        )?;

        let transfer3_ix = spl_token::instruction::transfer(
            token_program.key,
            ata_account.key,
            ata3_account.key,
            pc_account.key,
            &[&pc_account.key],
            amount3,
        )?;
        msg!("Calling the token program to transfer ATA ---> ATA3");
        invoke(
            &transfer3_ix,
            &[
                pc_account.clone(),
                ata_account.clone(),
                ata3_account.clone(),
                token_program.clone(),
            ],
        )?;

        let transfer4_ix = spl_token::instruction::transfer(
            token_program.key,
            ata_account.key,
            ata4_account.key,
            pc_account.key,
            &[&pc_account.key],
            amount4,
        )?;
        msg!("Calling the token program to transfer ATA ---> ATA4");
        invoke(
            &transfer4_ix,
            &[
                pc_account.clone(),
                ata_account.clone(),
                ata4_account.clone(),
                token_program.clone(),
            ],
        )?;

        Ok(())
    }
    //==========================================================================
    fn process_transfer_token(
        accounts: &[AccountInfo],
        amount: u64,
        fee1: u8,
        fee2: u8,
        fee3: u8,
        fee4: u8,
        fee5: u8,
        _program_id: &Pubkey,
    ) -> ProgramResult {

        let amount_1 = ((amount as f64) * (fee1 as f64) / 100.0) as u64;
        let amount_2 = ((amount as f64) * (fee2 as f64) / 100.0) as u64;
        let amount_3 = ((amount as f64) * (fee3 as f64) / 100.0) as u64;
        let amount_4 = ((amount as f64) * (fee4 as f64) / 100.0) as u64;
        let amount_5 = ((amount as f64) * (fee5 as f64) / 100.0) as u64;
        let amount_t = amount - amount_1 - amount_2 - amount_3 - amount_4 - amount_5;

        let account_info_iter = &mut accounts.iter();

        let src_account = next_account_info(account_info_iter)?;
        msg!("src_account Pubkey : {}", src_account.key);

        let dest_account = next_account_info(account_info_iter)?;
        msg!("dest_account Pubkey : {}", dest_account.key);

        let token_program = next_account_info(account_info_iter)?;
        msg!("token_program : {}", token_program.key);

        let ata1_account = next_account_info(account_info_iter)?;
        msg!("ata1_account Pubkey : {}", ata1_account.key);
        
        let ata2_account = next_account_info(account_info_iter)?;
        msg!("ata2_account Pubkey : {}", ata2_account.key);

        let fee_ata1_account = next_account_info(account_info_iter)?;
        msg!("fee_ata1_account Pubkey : {}", fee_ata1_account.key);
        
        let fee_ata2_account = next_account_info(account_info_iter)?;
        msg!("fee_ata2_account Pubkey : {}", fee_ata2_account.key);
        
        let fee_ata3_account = next_account_info(account_info_iter)?;
        msg!("fee_ata3_account Pubkey : {}", fee_ata3_account.key);
        
        let fee_ata4_account = next_account_info(account_info_iter)?;
        msg!("fee_ata4_account Pubkey : {}", fee_ata4_account.key);
        
        let fee_ata5_account = next_account_info(account_info_iter)?;
        msg!("fee_ata5_account Pubkey : {}", fee_ata5_account.key);
                
        let system_program_account = next_account_info(account_info_iter)?;
        msg!("system_program_account Pubkey : {}", system_program_account.key );

        let transfer_ix = spl_token::instruction::transfer(
            token_program.key,
            ata1_account.key,
            ata2_account.key,
            src_account.key,
            &[&src_account.key],
            amount_t,
        )?;
        msg!("Calling the token program to transfer ATA1 ---> ATA2");
        invoke(
            &transfer_ix,
            &[
                src_account.clone(),
                ata1_account.clone(),
                ata2_account.clone(),
                token_program.clone(),
            ],
        )?;

        let transfer_fee1_ix = spl_token::instruction::transfer(
            token_program.key,
            ata1_account.key,
            fee_ata1_account.key,
            src_account.key,
            &[&src_account.key],
            amount_1,
        )?;
        msg!("Calling the token program to transfer ATA1 ---> feeATA1");
        invoke(
            &transfer_fee1_ix,
            &[
                src_account.clone(),
                ata1_account.clone(),
                fee_ata1_account.clone(),
                token_program.clone(),
            ],
        )?;

        let transfer_fee2_ix = spl_token::instruction::transfer(
            token_program.key,
            ata1_account.key,
            fee_ata2_account.key,
            src_account.key,
            &[&src_account.key],
            amount_2,
        )?;
        msg!("Calling the token program to transfer ATA1 ---> feeATA2");
        invoke(
            &transfer_fee2_ix,
            &[
                src_account.clone(),
                ata1_account.clone(),
                fee_ata2_account.clone(),
                token_program.clone(),
            ],
        )?;

        let transfer_fee3_ix = spl_token::instruction::transfer(
            token_program.key,
            ata1_account.key,
            fee_ata3_account.key,
            src_account.key,
            &[&src_account.key],
            amount_3,
        )?;
        msg!("Calling the token program to transfer ATA1 ---> feeATA3");
        invoke(
            &transfer_fee3_ix,
            &[
                src_account.clone(),
                ata1_account.clone(),
                fee_ata3_account.clone(),
                token_program.clone(),
            ],
        )?;

        let transfer_fee4_ix = spl_token::instruction::transfer(
            token_program.key,
            ata1_account.key,
            fee_ata4_account.key,
            src_account.key,
            &[&src_account.key],
            amount_4,
        )?;
        msg!("Calling the token program to transfer ATA1 ---> feeATA4");
        invoke(
            &transfer_fee4_ix,
            &[
                src_account.clone(),
                ata1_account.clone(),
                fee_ata4_account.clone(),
                token_program.clone(),
            ],
        )?;

        let transfer_fee5_ix = spl_token::instruction::transfer(
            token_program.key,
            ata1_account.key,
            fee_ata5_account.key,
            src_account.key,
            &[&src_account.key],
            amount_5,
        )?;
        msg!("Calling the token program to transfer ATA1 ---> feeATA5");
        invoke(
            &transfer_fee5_ix,
            &[
                src_account.clone(),
                ata1_account.clone(),
                fee_ata5_account.clone(),
                token_program.clone(),
            ],
        )?;

        Ok(())
    }

}