use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub admin: Pubkey,
    pub new_admin: Option<Pubkey>,
}