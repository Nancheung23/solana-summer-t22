use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::Token2022,
    token_interface::{self, Mint, TokenAccount},
};

use crate::{error::ErrorCode, state::Config};

#[derive(Accounts)]
pub struct ForcedBurn<'info> {
    pub admin: Signer<'info>,

    #[account(
        seeds = [b"config"],
        bump,
        constraint = config.admin == admin.key() @ ErrorCode::Unauthorized,
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::token_program = token_program,
    )]
    pub from: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: permanent delegate PDA; signs the burn via the program.
    #[account(
        seeds = [b"authority"],
        bump,
    )]
    pub authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token2022>,
}

pub fn handler(ctx: Context<ForcedBurn>, amount: u64) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[b"authority", &[ctx.bumps.authority]]];

    let cpi_accounts = token_interface::Burn {
        mint: ctx.accounts.mint.to_account_info(),
        from: ctx.accounts.from.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.key(),
        cpi_accounts,
        signer_seeds,
    );

    token_interface::burn(cpi_ctx, amount)
}
