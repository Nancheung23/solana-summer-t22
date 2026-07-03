use anchor_lang::prelude::*;
use anchor_spl::token_interface;
use anchor_spl::{token_2022::Token2022, token_interface::Mint};

use crate::error::ErrorCode;
use crate::Config;

#[derive(Accounts)]
pub struct CloseMint<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    // mint token
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    // config PDA
    #[account(
        seeds = [b"config"],
        bump,
        constraint = config.admin == admin.key() @ ErrorCode::Unauthorized
    )]
    pub config: Account<'info, Config>,

    // mint pda
    /// CHECK: PDA is validated via seeds and bump constraints.
    #[account(
        seeds = [b"authority"],
        bump,
    )]
    pub authority: UncheckedAccount<'info>,

    // program
    pub token_program: Program<'info, Token2022>,
}

pub fn handler(ctx: Context<CloseMint>) -> Result<()> {
    // info
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_mint = ctx.accounts.mint.to_account_info();
    let cpi_bumps = ctx.bumps.authority;
    let seeds: &[&[u8]] = &[b"authority", &[cpi_bumps]];

    // require
    let supply = ctx.accounts.mint.supply;
    require!(supply == 0, ErrorCode::CloseMintError);

    // cpi accounts
    let cpi_accounts = token_interface::CloseAccount {
        account: cpi_mint,
        destination: ctx.accounts.admin.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    // context
    let signer_seeds = &[&seeds[..]];
    let cpi_ctx = CpiContext::new_with_signer(cpi_program.key(), cpi_accounts, signer_seeds);

    token_interface::close_account(cpi_ctx)?;
    Ok(())
}
