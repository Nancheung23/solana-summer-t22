pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("13c1eyCGtdqnzoaespctdLU1G8TrG4StGPugUa7BTx8C");

#[program]
pub mod solana_summer_t22 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    // add metadata pointer
    pub fn initialize_mint(ctx: Context<InitializeMint>, args: TokenMetadataArgs) -> Result<()> {
        mint::handler(ctx, args)
    }

    pub fn propose_admin(ctx: Context<ProposeAdmin>, new_admin: Pubkey) -> Result<()> {
        propose_admin::handler(ctx, new_admin)
    }

    pub fn confirm_admin(ctx: Context<ConfirmAdmin>) -> Result<()> {
        confirm_admin::handler(ctx)
    }

    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        transfer::handler(ctx, amount)
    }

    pub fn burn(ctx: Context<Burn>, amount: u64) -> Result<()> {
        burn::handler(ctx, amount)
    }

    pub fn forced_transfer(ctx: Context<ForcedTransfer>, amount: u64) -> Result<()> {
        forced_transfer::handler(ctx, amount)
    }

    pub fn forced_burn(ctx: Context<ForcedBurn>, amount: u64) -> Result<()> {
        forced_burn::handler(ctx, amount)
    }

    pub fn update_default_account_state(
        ctx: Context<UpdateDefaultAccountState>,
        state_code: u8,
    ) -> Result<()> {
        update_default_account_state::handler(ctx, state_code)
    }

    pub fn update_metadata(
        ctx: Context<UpdateMetadata>,
        field_code: String,
        value: String,
    ) -> Result<()> {
        update_metadata::handler(ctx, field_code, value)
    }

    pub fn close_mint(ctx: Context<CloseMint>) -> Result<()> {
        close_mint::handler(ctx)
    }
}
