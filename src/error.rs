use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Clone, Copy, Debug, Error)]
pub enum EscrowError {
	#[error("Invalid Instruction")]
	InvalidInstruction,
	#[error("Not Rent Exempt")]
	NotRentExempt,
}

impl From<EscrowError> for ProgramError {
	fn from(e: EscrowError) -> Self {
		ProgramError::Custom(e as u32)
	}
}
