mod common;
use anchor_lang::solana_program;
use common::*;
use solana_program::instruction::Instruction;
use solana_summer_t22::instruction::CloseMint;

#[test]
fn close_mint_succeeds() {
    let (mut svm, admin, program_id) = setup();
    init_config(&mut svm, &admin, program_id);

    let mint = init_mint(&mut svm, &admin, program_id);

    let ix = Instruction::new_with_bytes(
        program_id,
        &CloseMint {}.data(),
        solana_summer_t22::accounts::CloseMint {
            admin: admin.pubkey(),
            mint: mint.pubkey(),
            config: config_pda(&program_id),
            authority: authority_pda(&program_id),
            token_program: TOKEN_2022_ID,
        }
        .to_account_metas(None),
    );

    let t = tx(&svm, &[ix], &admin.pubkey(), &[&admin]);
    let result = svm.send_transaction(t);

    assert!(result.is_ok(), "Error closing mint: {:?}", result.err());

    let deleted_mint_account = svm.get_account(&mint.pubkey());
    assert!(
        deleted_mint_account.is_none(),
        "Mint account should be completely removed from the SVM"
    );
}

#[test]
fn close_mint_rejects_non_admin() {
    let (mut svm, admin, program_id) = setup();
    init_config(&mut svm, &admin, program_id);
    let mint = init_mint(&mut svm, &admin, program_id);

    let intruder = funded_keypair(&mut svm);

    let ix = Instruction::new_with_bytes(
        program_id,
        &CloseMint {}.data(),
        solana_summer_t22::accounts::CloseMint {
            admin: intruder.pubkey(),
            mint: mint.pubkey(),
            config: config_pda(&program_id),
            authority: authority_pda(&program_id),
            token_program: TOKEN_2022_ID,
        }
        .to_account_metas(None),
    );

    let t = tx(&svm, &[ix], &intruder.pubkey(), &[&intruder]);
    let result = svm.send_transaction(t);

    assert!(
        result.is_err(),
        "Transaction should fail because the intruder is not the admin configured in the config PDA"
    );
}
