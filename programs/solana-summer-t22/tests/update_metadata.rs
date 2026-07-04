mod common;
use common::*;

#[test]
fn update_metadata_succeeds() {
    let (mut svm, admin, program_id) = setup();
    init_config(&mut svm, &admin, program_id);
    let mint = init_mint(&mut svm, &admin, program_id);

    let new_name = "Updated Summer Coin".to_string();

    // update_metadata
    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_summer_t22::instruction::UpdateMetadata {
            field_code: "name".to_string(),
            value: new_name.clone(),
        }
        .data(),
        solana_summer_t22::accounts::UpdateMetadata {
            admin: admin.pubkey(),
            mint: mint.pubkey(),
            config: config_pda(&program_id),
            authority: authority_pda(&program_id),
            token_program: TOKEN_2022_ID,
        }
        .to_account_metas(None),
    );

    let t = tx(&svm, &[ix], &admin.pubkey(), &[&admin]);
    svm.send_transaction(t).expect("update_metadata failed");

    let account = svm
        .get_account(&mint.pubkey())
        .expect("mint account missing");

    assert!(
        account
            .data
            .windows(new_name.len())
            .any(|window| window == new_name.as_bytes()),
        "Mint account data should contain the updated name"
    );
}

#[test]
fn update_metadata_rejects_non_admin() {
    let (mut svm, admin, program_id) = setup();
    init_config(&mut svm, &admin, program_id);
    let mint = init_mint(&mut svm, &admin, program_id);

    let intruder = funded_keypair(&mut svm);

    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_summer_t22::instruction::UpdateMetadata {
            field_code: "name".to_string(),
            value: "Malicious Name".to_string(),
        }
        .data(),
        solana_summer_t22::accounts::UpdateMetadata {
            admin: intruder.pubkey(),
            mint: mint.pubkey(),
            config: config_pda(&program_id),
            authority: authority_pda(&program_id),
            token_program: TOKEN_2022_ID,
        }
        .to_account_metas(None),
    );

    let t = tx(&svm, &[ix], &intruder.pubkey(), &[&intruder]);
    assert!(
        svm.send_transaction(t).is_err(),
        "Non-admin should not be able to update metadata"
    );
}
