use crate::{error::ErrorCode, state::Config};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::system_program::{create_account, CreateAccount};
use anchor_spl::token_2022::spl_token_2022::instruction as token_2022_inst;
use anchor_spl::{
    token_2022::{
        initialize_mint2,
        spl_token_2022::{extension::ExtensionType, pod::PodMint, state::AccountState},
        InitializeMint2,
    },
    token_interface::{default_account_state_initialize, DefaultAccountStateInitialize, Token2022},
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
    #[account(mut)]
    pub mint: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub token_program: Program<'info, Token2022>,
    // rent
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeMint>) -> Result<()> {
    let payer = &ctx.accounts.payer;
    let mint = &ctx.accounts.mint;
    let authority = &ctx.accounts.authority;
    let token_program = &ctx.accounts.token_program;

    // current extensions
    let extensions = [
        ExtensionType::PermanentDelegate,
        ExtensionType::DefaultAccountState,
    ];
    // calculate size
    let mint_size = ExtensionType::try_calculate_account_len::<PodMint>(&extensions)?;
    // convert to lamport
    let lamports = (Rent::get()?).minimum_balance(mint_size);

    // create mint account, pay rent
    create_account(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info().key(),
            CreateAccount {
                from: payer.to_account_info(),
                to: ctx.accounts.mint.to_account_info(),
            },
        ),
        lamports,
        mint_size as u64,
        &ctx.accounts.token_program.key(),
    )?;

    // permanent delegate
    invoke(
        &token_2022_inst::initialize_permanent_delegate(
            token_program.key,
            mint.key,
            authority.key,
        )?,
        &[mint.to_account_info()],
    )?;

    // initialize default_account_state
    default_account_state_initialize(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info().key(),
            DefaultAccountStateInitialize {
                token_program_id: ctx.accounts.token_program.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
        ),
        &AccountState::Initialized,
    )?;

    // mint setup
    invoke(
        &token_2022_inst::initialize_mint(
            token_program.key,
            mint.key,
            authority.key,
            Some(authority.key),
            6,
        )?,
        &[mint.to_account_info(), ctx.accounts.rent.to_account_info()],
    )?;
    Ok(())
}
