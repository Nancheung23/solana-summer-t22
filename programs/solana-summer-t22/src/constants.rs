use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

/// Only this address is allowed to initialize a `Config`.
pub const ADMIN: Pubkey = pubkey!("HNZsqu8wnc1kmRBxeFAT91ka9KBtvZ7vkELN5jJELa8c");
