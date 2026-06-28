use anchor_lang::prelude::*;

use crate::{error::ErrorCode, state::Config};

#[derive(Accounts)]
pub struct ConfirmAdmin<'info> {
    pub new_admin: Signer<'info>,

    #[account(
        mut,
        constraint = config.new_admin == Some(new_admin.key()) @ ErrorCode::Unauthorized,
    )]
    pub config: Account<'info, Config>,
}

pub fn handler(ctx: Context<ConfirmAdmin>) -> Result<()> {
    let config = &mut ctx.accounts.config;

    config.admin = ctx.accounts.new_admin.key();
    config.new_admin = None;

    Ok(())
}
