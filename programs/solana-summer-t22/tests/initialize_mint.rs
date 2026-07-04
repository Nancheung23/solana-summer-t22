mod common;
use common::*;

#[test]
fn initialize_mint_succeeds() {
    let (mut svm, admin, program_id) = setup();
    init_config(&mut svm, &admin, program_id);
    let mint = init_mint(&mut svm, &admin, program_id);

    let account = svm.get_account(&mint.pubkey()).expect("mint not created");
    assert_eq!(account.owner, TOKEN_2022_ID);
    // Base mint (82) + account-type byte + permanent-delegate extension means the
    // account is larger than a plain legacy mint.
    assert!(account.data.len() > 82, "mint should carry an extension");
}

#[test]
fn initialize_mint_rejects_non_admin() {
    let (mut svm, admin, program_id) = setup();
    init_config(&mut svm, &admin, program_id);

    let intruder = funded_keypair(&mut svm);
    let mint = Keypair::new();
    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_summer_t22::instruction::InitializeMint {
            args: solana_summer_t22::instructions::mint::TokenMetadataArgs {
                name: "Test Token".to_string(),
                symbol: "TEST".to_string(),
                uri: "https://example.com/TST.json".to_string(),
            },
        }
        .data(),
        solana_summer_t22::accounts::InitializeMint {
            payer: intruder.pubkey(),
            config: config_pda(&program_id),
            authority: authority_pda(&program_id),
            rent: anchor_lang::prelude::rent::ID,
            mint: mint.pubkey(),
            token_program: TOKEN_2022_ID,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    let t = tx(&svm, &[ix], &intruder.pubkey(), &[&intruder, &mint]);
    assert!(
        svm.send_transaction(t).is_err(),
        "only the admin should be able to create the mint"
    );
}
