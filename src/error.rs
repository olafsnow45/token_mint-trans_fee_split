use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum SplitError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    /// Not Rent Exempt
    #[error("Not Rent Exempt")]
    NotRentExempt,
    /// Expected Amount Mismatch
    #[error("Expected Amount Mismatch")]
    ExpectedAmountMismatch,
    /// Invalid wallet
    #[error("Invalid Wallet Address")]
    InvalidWalletAddress,
}

impl From<SplitError> for ProgramError {
    fn from(e: SplitError) -> Self {
        ProgramError::Custom(e as u32)
    }
}