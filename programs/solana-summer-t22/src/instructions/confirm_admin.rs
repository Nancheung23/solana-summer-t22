use anchor_lang::prelude::*;

use crate::{error::ErrorCode, state::Config};

#[derive(Accounts)]
pub struct ConfirmAdmin<'info> {
    // admin wallet
    pub new_admin: Signer<'info>,

    // admin pda
    #[account(
        mut,
        constraint = config.new_admin == Some(new_admin.key()) @ ErrorCode::Unauthorized,
    )]
    pub config: Account<'info, Config>,
}

pub fn handler(ctx: Context<ConfirmAdmin>) -> Result<()> {
    // borrow mut
    let config = &mut ctx.accounts.config;

    // give admin new authority
    config.admin = ctx.accounts.new_admin.key();
    config.new_admin = None;

    Ok(())
}
