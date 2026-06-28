use anchor_lang::prelude::*;

use crate::{constants::ADMIN, error::ErrorCode, state::Config};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        mut,
        address = ADMIN @ ErrorCode::Unauthorized,
    )]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"config"],
        bump,
    )]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.config.set_inner(Config {
        admin: ctx.accounts.admin.key(),
        new_admin: None,
    });

    Ok(())
}
