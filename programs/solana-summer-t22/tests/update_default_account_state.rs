mod common;
use common::*;
use solana_summer_t22::instruction::UpdateDefaultAccountState;

#[test]
fn update_default_account_state_succeeds() {
    let (mut svm, admin, program_id) = setup();
    init_config(&mut svm, &admin, program_id);
    let mint = init_mint(&mut svm, &admin, program_id);

    let state_code = 1u8;

    let ix = Instruction::new_with_bytes(
        program_id,
        &UpdateDefaultAccountState {
            state_code: state_code,
        }
        .data(),
        solana_summer_t22::accounts::UpdateDefaultAccountState {
            admin: admin.pubkey(),
            authority: authority_pda(&program_id),
            mint: mint.pubkey(),
            config: config_pda(&program_id),
            token_program: TOKEN_2022_ID,
        }
        .to_account_metas(None),
    );

    let t = tx(&svm, &[ix], &admin.pubkey(), &[&admin]);
    let result = svm.send_transaction(t);
    assert!(result.is_ok(), "Error: {:?}", result.err());
}

#[test]
fn update_default_account_state_rejects_non_admin() {
    let (mut svm, admin, program_id) = setup();
    init_config(&mut svm, &admin, program_id);
    let mint = init_mint(&mut svm, &admin, program_id);

    let intruder = funded_keypair(&mut svm);

    let ix = Instruction::new_with_bytes(
        program_id,
        &UpdateDefaultAccountState { state_code: 1 }.data(),
        solana_summer_t22::accounts::UpdateDefaultAccountState {
            admin: intruder.pubkey(),
            authority: authority_pda(&program_id),
            mint: mint.pubkey(),
            config: config_pda(&program_id),
            token_program: TOKEN_2022_ID,
        }
        .to_account_metas(None),
    );

    let t = tx(&svm, &[ix], &intruder.pubkey(), &[&intruder]);
    assert!(
        svm.send_transaction(t).is_err(),
        "only the admin should be able to update state"
    );
}
