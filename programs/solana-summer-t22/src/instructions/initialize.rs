use anchor_lang::prelude::*;

use crate::{error::ErrorCode, state::Config};

#[derive(Accounts)]
pub struct Initialize<'info> {
    // admin
    #[account(
        mut,
        address = crate::constants::ADMIN @ErrorCode::Unauthorized,
    )]
    pub admin: Signer<'info>,

    // init config pda by admin
    #[account(
        init,
        payer = admin,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"config"],
        bump,
    )]
    pub config: Account<'info, Config>,

    // program
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.admin.key(),
        crate::constants::ADMIN,
        ErrorCode::Unauthorized
    );
    ctx.accounts.config.set_inner(Config {
        admin: ctx.accounts.admin.key(),
        new_admin: None,
    });

    Ok(())
}
