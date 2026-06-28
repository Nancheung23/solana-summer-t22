use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::Token2022,
    token_interface::{self, Mint, TokenAccount},
};

#[derive(Accounts)]
pub struct Burn<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = owner,
        token::token_program = token_program,
    )]
    pub from: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,
}

pub fn handler(ctx: Context<Burn>, amount: u64) -> Result<()> {
    let cpi_accounts = token_interface::Burn {
        mint: ctx.accounts.mint.to_account_info(),
        from: ctx.accounts.from.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.key(), cpi_accounts);

    token_interface::burn(cpi_ctx, amount)
}
