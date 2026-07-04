use crate::error::ErrorCode;
use crate::Config;
use anchor_lang::prelude::*;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::spl_token_metadata_interface::state::Field;
use anchor_spl::token_interface::{token_metadata_update_field, Mint, TokenMetadataUpdateField};

#[derive(Accounts)]
pub struct UpdateMetadata<'info> {
    // signer
    #[account(mut)]
    pub admin: Signer<'info>,
    // mint
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    // config pda
    #[account(
        seeds = [b"config"],
        bump,
        constraint = config.admin == admin.key() @ErrorCode::Unauthorized
    )]
    pub config: Account<'info, Config>,
    // delegate authority pda
    /// CHECK: PDA validated via seeds
    #[account(
        seeds = [b"authority"],
        bump,
    )]
    pub authority: UncheckedAccount<'info>,
    // program
    pub token_program: Program<'info, Token2022>,
}

pub fn handler(ctx: Context<UpdateMetadata>, field_code: String, value: String) -> Result<()> {
    let field = match field_code.as_str() {
        "name" => Field::Name,
        "symbol" => Field::Symbol,
        "uri" => Field::Uri,
        _ => Field::Key(field_code),
    };
    let authority = ctx.accounts.authority.to_account_info();
    let cpi_program = &ctx.accounts.token_program;
    let cpi_bump = ctx.bumps.authority;
    let seeds: &[&[u8]] = &[b"authority", &[cpi_bump]];
    let cpi_accounts = TokenMetadataUpdateField {
        program_id: cpi_program.to_account_info(),
        metadata: ctx.accounts.mint.to_account_info(),
        update_authority: authority,
    };
    let signer_seeds = &[&seeds[..]];
    let cpi_ctx = CpiContext::new_with_signer(cpi_program.key(), cpi_accounts, signer_seeds);
    token_metadata_update_field(cpi_ctx, field, value)?;
    Ok(())
}
