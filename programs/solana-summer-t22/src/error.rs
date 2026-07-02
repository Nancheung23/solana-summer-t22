use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Only the admin can perform this action")]
    Unauthorized,
    #[msg("Invalid state input, [0, 1, 2]")]
    InvalidState,
}
