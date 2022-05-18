// use core::num;

use solana_program::program_error::ProgramError;

use crate::error::SplitError::InvalidInstruction;

pub enum SplitInstruction {
    /// Starts the trade by creating and populating an Split account and transferring ownership of the given temp token account to the PDA
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the Split
    /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer
    /// 2. `[]` The initializer's token account for the token they will receive should the trade go through
    /// 3. `[writable]` The Split account, it will hold all necessary info about the trade.
    /// 4. `[]` The rent sysvar
    /// 5. `[]` The token program
    MintAccount {
        /// The amount party A expects to receive of token Y
        amount : u64,
    },
    /// Starts the trade by creating and populating an Split account and transferring ownership of the given temp token account to the PDA
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the Split
    /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer
    /// 2. `[]` The initializer's token account for the token they will receive should the trade go through
    /// 3. `[writable]` The Split account, it will hold all necessary info about the trade.
    /// 4. `[]` The rent sysvar
    /// 5. `[]` The token program
    TransferAccount {
        /// The amount party A expects to receive of token Y
        amount : u64,
        fee1 : u8,
        fee2 : u8,
        fee3 : u8,
        fee4 : u8,
        fee5 : u8,
    },
}

impl SplitInstruction {
    /// Unpacks a byte buffer into a [SplitInstruction](enum.SplitInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {

        let tag = &input[0];
        let amount = Self::unpack_amount(&input[1..=8])?;
        let fee_1 = &input[9];
        let fee_2 = &input[10];
        let fee_3 = &input[11];
        let fee_4 = &input[12];
        let fee_5 = &input[13];
        
        Ok(match tag {
            0 => Self::MintAccount {
                // amount: Self::unpack_amount(rest)?,
                amount: amount
            },
            1 => Self::TransferAccount {
                // amount: Self::unpack_amount(rest)?,
                amount : amount,
                fee1 : *fee_1,
                fee2 : *fee_2,
                fee3 : *fee_3,
                fee4 : *fee_4,
                fee5 : *fee_5,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}