use crate::{error::ErrorCode, state::Config};
use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{spl_token_2022::state::AccountState, Token2022},
    token_interface::{DefaultAccountStateInitialize, Mint},
};

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    // admin
    #[account(mut)]
    pub payer: Signer<'info>,

    // pda
    #[account(
        seeds = [b"config"],
        bump,
        constraint = config.admin == payer.key() @ ErrorCode::Unauthorized,
    )]
    pub config: Account<'info, Config>,

    /// CHECK: PDA used as the mint authority and permanent delegate.
    #[account(
        seeds = [b"authority"],
        bump,
    )]
    pub authority: UncheckedAccount<'info>,

    // create Mint
    #[account(
        init,
        payer = payer,
        mint::decimals = 6,
        mint::authority = authority,
        mint::freeze_authority = authority,
        mint::token_program = token_program,
        // activate delegate authority PDA
        extensions::permanent_delegate::delegate = authority,
        // meta data
        extensions::metadata_pointer::authority = authority,
        extensions::metadata_pointer::metadata_address = mint,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    pub token_program: Program<'info, Token2022>,
    // rent
    pub system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<InitializeMint>) -> Result<()> {
    let cpi_program = _ctx.accounts.token_program.to_account_info();
    let mint = _ctx.accounts.mint.to_account_info();
    let bumps_authority = _ctx.bumps.authority;
    // match seeds of authority
    let seeds: &[&[u8]] = &[b"authority", &[bumps_authority]];
    // set default state
    let default_state = AccountState::Initialized;

    let cpi_accounts = DefaultAccountStateInitialize {
        mint: mint,
        token_program_id: cpi_program.clone(),
    };

    let signer_seeds = &[&seeds[..]];
    let cpi_ctx = CpiContext::new_with_signer(cpi_program.key(), cpi_accounts, signer_seeds);
    // trigger initialization
    anchor_spl::token_interface::default_account_state_initialize(cpi_ctx, &default_state)?;
    Ok(())
}
