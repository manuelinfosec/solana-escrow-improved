// program-specific errors
use std::fmt;

use solana_program::program_error::ProgramError;

#[derive(Debug)]
pub enum EscrowError {
    /// Invalid Instruction
    InvalidInstruction,
}

// Implement Display for EscrowError
impl fmt::Display for EscrowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EscrowError::InvalidInstruction => write!(f, "Invalid Instruction"),
        }
    }
}

impl std::error::Error for EscrowError {}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
