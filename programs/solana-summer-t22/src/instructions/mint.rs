use anchor_lang::prelude::*;
use anchor_spl::{token_2022::Token2022, token_interface::Mint};

use crate::{error::ErrorCode, state::Config};

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
        mint::token_program = token_program,
        // activate delegate authority PDA
        extensions::permanent_delegate::delegate = authority,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    pub token_program: Program<'info, Token2022>,
    // rent
    pub system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<InitializeMint>) -> Result<()> {
    Ok(())
}
