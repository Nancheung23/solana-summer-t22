use crate::{error::ErrorCode, Config};
use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{spl_token_2022::state::AccountState, Token2022},
    token_interface::{DefaultAccountStateUpdate, Mint},
};

#[derive(Accounts)]
pub struct UpdateDefaultAccountState<'info> {
    #[account(mut, constraint = admin.key() == config.admin @ ErrorCode::Unauthorized)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [b"authority"],
        bump
    )]
    pub authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    pub config: Account<'info, Config>,
    pub token_program: Program<'info, Token2022>,
}

pub fn handler(ctx: Context<UpdateDefaultAccountState>, state_code: u8) -> Result<()> {
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let mint = ctx.accounts.mint.to_account_info();
    let authority = &ctx.accounts.authority;
    // state from input state code
    // spl-token-2022-interface-2.1.0/src/state.rs => enum AccountState
    let state = match state_code {
        0 => AccountState::Initialized,
        1 => AccountState::Frozen,
        2 => AccountState::Uninitialized,
        _ => return err!(ErrorCode::InvalidState),
    };

    let cpi_bump = ctx.bumps.authority;
    let seeds: &[&[u8]] = &[b"authority", &[cpi_bump]];
    let cpi_accounts = DefaultAccountStateUpdate {
        mint: mint,
        token_program_id: cpi_program.clone(),
        freeze_authority: authority.to_account_info(),
    };
    let signer_seeds = &[&seeds[..]];
    let cpi_ctx = CpiContext::new_with_signer(cpi_program.key(), cpi_accounts, signer_seeds);
    anchor_spl::token_interface::default_account_state_update(cpi_ctx, &state)?;
    Ok(())
}
