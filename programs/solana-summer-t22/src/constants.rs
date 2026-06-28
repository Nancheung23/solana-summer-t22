use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

/// Only this address is allowed to initialize a `Config`.
pub const ADMIN: Pubkey = pubkey!("AHYic562KhgtAEkb1rSesqS87dFYRcfXb4WwWus3Zc9C");
