use crate::{error::ErrorCode, state::Config};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::system_program::{create_account, CreateAccount};
// metadata pointer
use anchor_spl::token_2022::spl_token_2022::extension::metadata_pointer::instruction::initialize as initialize_metadata_pointer_inst;
use anchor_spl::token_2022::spl_token_2022::instruction as token_2022_inst;
use anchor_spl::token_interface::{token_metadata_initialize, TokenMetadataInitialize};
use anchor_spl::{
    token_2022::spl_token_2022::{extension::ExtensionType, pod::PodMint, state::AccountState},
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

// metadata struct
#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct TokenMetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

pub fn handler(ctx: Context<InitializeMint>, args: TokenMetadataArgs) -> Result<()> {
    // metadata implement and space cauculation
    let TokenMetadataArgs { name, symbol, uri } = args;
    // TLV(4) + update_authority(32) + mint(32)
    // + name (4) + string
    // + symbol (4) + string
    // + uri (4) + string
    // + additional_metadata [] (4)
    let exact_metadata_space = 4 + 32 + 32 + 4 + name.len() + 4 + symbol.len() + 4 + uri.len() + 4;

    let payer = &ctx.accounts.payer;
    let mint = &ctx.accounts.mint;
    let authority = &ctx.accounts.authority;
    let token_program = &ctx.accounts.token_program;

    // current extensions
    let extensions = [
        ExtensionType::MintCloseAuthority,
        ExtensionType::DefaultAccountState,
        ExtensionType::PermanentDelegate,
        ExtensionType::MetadataPointer,
        ExtensionType::TokenMetadata,
    ];
    // let base_size = 228;
    let mint_size =
        ExtensionType::try_calculate_account_len::<PodMint>(&extensions)? + exact_metadata_space;
    // let mint_size = base_size + exact_metadata_space;
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
    // mint close authority: 3
    invoke(
        &token_2022_inst::initialize_mint_close_authority(
            token_program.key,
            mint.key,
            Some(authority.key),
        )?,
        &[mint.to_account_info()],
    )?;
    // initialize default_account_state: 6
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
    // permanent delegate: 12
    invoke(
        &token_2022_inst::initialize_permanent_delegate(
            token_program.key,
            mint.key,
            authority.key,
        )?,
        &[mint.to_account_info()],
    )?;
    // setup metadata pointer, link metadata_address to mint address: 18
    invoke(
        &initialize_metadata_pointer_inst(
            token_program.key,
            mint.key,
            Some(authority.key()),
            Some(mint.key()),
        )?,
        &[mint.to_account_info()],
    )?;
    // mint setup
    invoke(
        &token_2022_inst::initialize_mint2(
            token_program.key,
            mint.key,
            authority.key,
            Some(authority.key),
            6,
        )?,
        &[mint.to_account_info(), ctx.accounts.rent.to_account_info()],
    )?;
    let cpi_bumps = ctx.bumps.authority;
    let signer_seeds: &[&[&[u8]]] = &[&[b"authority", &[cpi_bumps]]];
    // metadata content must be initialized after mint
    token_metadata_initialize(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info().key(),
            TokenMetadataInitialize {
                program_id: ctx.accounts.token_program.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                metadata: ctx.accounts.mint.to_account_info(),
                mint_authority: ctx.accounts.authority.to_account_info(),
                update_authority: ctx.accounts.authority.to_account_info(),
            },
            signer_seeds,
        ),
        name,
        symbol,
        uri,
    )?;
    Ok(())
}
