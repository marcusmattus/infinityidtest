use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    #[msg("Invalid operation.")]
    InvalidOperation,
}