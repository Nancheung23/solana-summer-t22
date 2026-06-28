//! Shared test harness: program loading, signing, PDAs, and Token-2022 account
//! seeding. Each use-case test file does `mod common;` / `use common::*;`.
//!
//! `dead_code` is allowed because every test binary pulls in this whole module
//! but only uses the helpers relevant to its use case.
#![allow(dead_code)]

pub use {
    anchor_lang::{
        prelude::Pubkey,
        solana_program::{instruction::Instruction, system_program},
        AccountDeserialize, InstructionData, ToAccountMetas,
    },
    anchor_spl::token_2022::ID as TOKEN_2022_ID,
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_signer::Signer,
};

use {
    solana_account::Account,
    solana_message::{Message, VersionedMessage},
    solana_transaction::versioned::VersionedTransaction,
};

pub const PROGRAM_SO: &[u8] = include_bytes!("../../../../target/deploy/solana_summer_t22.so");

// SPL token account byte offsets (legacy / Token-2022 base layout, 165 bytes).
const TOKEN_ACCOUNT_LEN: usize = 165;
const ACCOUNT_AMOUNT_OFFSET: usize = 64;
const ACCOUNT_STATE_OFFSET: usize = 108;
// Mint base layout: supply lives right after the COption<Pubkey> mint authority.
const MINT_SUPPLY_OFFSET: usize = 36;

/// The program gates several instructions to a hardcoded `ADMIN` pubkey, which
/// is the local CLI wallet. Load it so the tests can sign as the admin.
pub fn load_admin() -> Keypair {
    let home = std::env::var("HOME").expect("HOME not set");
    let path = format!("{home}/.config/solana/id.json");
    let json = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("could not read admin keypair at {path}: {e}"));
    let bytes: Vec<u8> = json
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|b| b.trim().parse::<u8>().expect("invalid byte in keypair file"))
        .collect();
    Keypair::try_from(&bytes[..]).expect("invalid admin keypair")
}

pub fn setup() -> (LiteSVM, Keypair, Pubkey) {
    let program_id = solana_summer_t22::id();
    let mut svm = LiteSVM::new();
    svm.add_program(program_id, PROGRAM_SO).unwrap();

    let admin = load_admin();
    svm.airdrop(&admin.pubkey(), 100_000_000_000).unwrap();

    (svm, admin, program_id)
}

pub fn funded_keypair(svm: &mut LiteSVM) -> Keypair {
    let kp = Keypair::new();
    svm.airdrop(&kp.pubkey(), 10_000_000_000).unwrap();
    kp
}

pub fn config_pda(program_id: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"config"], program_id).0
}

pub fn authority_pda(program_id: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"authority"], program_id).0
}

pub fn tx(
    svm: &LiteSVM,
    ixs: &[Instruction],
    payer: &Pubkey,
    signers: &[&Keypair],
) -> VersionedTransaction {
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(ixs, Some(payer), &blockhash);
    VersionedTransaction::try_new(VersionedMessage::Legacy(msg), signers).unwrap()
}

pub fn read_config(svm: &LiteSVM, program_id: &Pubkey) -> solana_summer_t22::Config {
    let account = svm
        .get_account(&config_pda(program_id))
        .expect("config account missing");
    solana_summer_t22::Config::try_deserialize(&mut account.data.as_slice())
        .expect("could not deserialize config")
}

pub fn token_amount(svm: &LiteSVM, account: &Pubkey) -> u64 {
    let account = svm.get_account(account).expect("token account missing");
    u64::from_le_bytes(
        account.data[ACCOUNT_AMOUNT_OFFSET..ACCOUNT_AMOUNT_OFFSET + 8]
            .try_into()
            .unwrap(),
    )
}

pub fn mint_supply(svm: &LiteSVM, mint: &Pubkey) -> u64 {
    let account = svm.get_account(mint).expect("mint account missing");
    u64::from_le_bytes(
        account.data[MINT_SUPPLY_OFFSET..MINT_SUPPLY_OFFSET + 8]
            .try_into()
            .unwrap(),
    )
}

/// Creates the global `Config` (admin-gated).
pub fn init_config(svm: &mut LiteSVM, admin: &Keypair, program_id: Pubkey) {
    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_summer_t22::instruction::Initialize {}.data(),
        solana_summer_t22::accounts::Initialize {
            admin: admin.pubkey(),
            config: config_pda(&program_id),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    let t = tx(svm, &[ix], &admin.pubkey(), &[admin]);
    svm.send_transaction(t).expect("init_config failed");
}

/// Creates a Token-2022 mint with the permanent-delegate extension (delegate &
/// mint authority = the `authority` PDA). Returns the mint keypair.
pub fn init_mint(svm: &mut LiteSVM, admin: &Keypair, program_id: Pubkey) -> Keypair {
    let mint = Keypair::new();
    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_summer_t22::instruction::InitializeMint {}.data(),
        solana_summer_t22::accounts::InitializeMint {
            payer: admin.pubkey(),
            config: config_pda(&program_id),
            authority: authority_pda(&program_id),
            mint: mint.pubkey(),
            token_program: TOKEN_2022_ID,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    let t = tx(svm, &[ix], &admin.pubkey(), &[admin, &mint]);
    svm.send_transaction(t).expect("init_mint failed");
    mint
}

/// Directly writes a Token-2022 token account holding `amount` tokens of `mint`,
/// owned by `owner`. The mint authority is a PDA we can't sign for outside the
/// program, so we seed balances by setting account state directly.
pub fn fund_token_account(svm: &mut LiteSVM, mint: &Pubkey, owner: &Pubkey, amount: u64) -> Pubkey {
    let address = Keypair::new().pubkey();
    let mut data = vec![0u8; TOKEN_ACCOUNT_LEN];
    data[0..32].copy_from_slice(mint.as_ref());
    data[32..64].copy_from_slice(owner.as_ref());
    data[ACCOUNT_AMOUNT_OFFSET..ACCOUNT_AMOUNT_OFFSET + 8].copy_from_slice(&amount.to_le_bytes());
    data[ACCOUNT_STATE_OFFSET] = 1; // AccountState::Initialized

    let account = Account {
        lamports: svm.minimum_balance_for_rent_exemption(data.len()),
        data,
        owner: TOKEN_2022_ID,
        executable: false,
        rent_epoch: 0,
    };
    svm.set_account(address, account).unwrap();
    address
}

/// Patches a mint's `supply` field so burns don't underflow (since we seed
/// balances directly rather than minting).
pub fn set_mint_supply(svm: &mut LiteSVM, mint: &Pubkey, supply: u64) {
    let mut account = svm.get_account(mint).expect("mint account missing");
    account.data[MINT_SUPPLY_OFFSET..MINT_SUPPLY_OFFSET + 8].copy_from_slice(&supply.to_le_bytes());
    svm.set_account(*mint, account).unwrap();
}
