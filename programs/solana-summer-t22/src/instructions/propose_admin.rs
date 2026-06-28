use anchor_lang::prelude::*;

use crate::{error::ErrorCode, state::Config};

#[derive(Accounts)]
pub struct ProposeAdmin<'info> {
    pub admin: Signer<'info>,

    #[account(
        mut,
        has_one = admin @ ErrorCode::Unauthorized,
    )]
    pub config: Account<'info, Config>,
}

pub fn handler(ctx: Context<ProposeAdmin>, new_admin: Pubkey) -> Result<()> {
    ctx.accounts.config.new_admin = Some(new_admin);

    Ok(())
}
