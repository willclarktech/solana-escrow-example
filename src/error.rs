use thiserror::Error;

#[derive(Clone, Copy, Debug, Error)]
pub enum EscrowError {
	#[error("Invalid Instruction")]
	InvalidInstruction,
}
