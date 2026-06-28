mod common;
use common::*;

#[test]
fn forced_transfer_by_delegate() {
    let (mut svm, admin, program_id) = setup();
    init_config(&mut svm, &admin, program_id);
    let mint = init_mint(&mut svm, &admin, program_id);

    // `victim` never signs — the permanent delegate moves their tokens.
    let victim = Pubkey::new_unique();
    let from = fund_token_account(&mut svm, &mint.pubkey(), &victim, 1_000_000);
    let to = fund_token_account(&mut svm, &mint.pubkey(), &Pubkey::new_unique(), 0);

    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_summer_t22::instruction::ForcedTransfer { amount: 300_000 }.data(),
        solana_summer_t22::accounts::ForcedTransfer {
            admin: admin.pubkey(),
            config: config_pda(&program_id),
            mint: mint.pubkey(),
            from,
            to,
            authority: authority_pda(&program_id),
            token_program: TOKEN_2022_ID,
        }
        .to_account_metas(None),
    );
    let t = tx(&svm, &[ix], &admin.pubkey(), &[&admin]);
    let res = svm.send_transaction(t);
    assert!(res.is_ok(), "forced_transfer failed: {:?}", res.err());

    assert_eq!(token_amount(&svm, &from), 700_000);
    assert_eq!(token_amount(&svm, &to), 300_000);
}

#[test]
fn forced_transfer_rejects_non_admin() {
    let (mut svm, admin, program_id) = setup();
    init_config(&mut svm, &admin, program_id);
    let mint = init_mint(&mut svm, &admin, program_id);

    let intruder = funded_keypair(&mut svm);
    let victim = Pubkey::new_unique();
    let from = fund_token_account(&mut svm, &mint.pubkey(), &victim, 1_000_000);
    let to = fund_token_account(&mut svm, &mint.pubkey(), &Pubkey::new_unique(), 0);

    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_summer_t22::instruction::ForcedTransfer { amount: 300_000 }.data(),
        solana_summer_t22::accounts::ForcedTransfer {
            admin: intruder.pubkey(),
            config: config_pda(&program_id),
            mint: mint.pubkey(),
            from,
            to,
            authority: authority_pda(&program_id),
            token_program: TOKEN_2022_ID,
        }
        .to_account_metas(None),
    );
    let t = tx(&svm, &[ix], &intruder.pubkey(), &[&intruder]);
    assert!(
        svm.send_transaction(t).is_err(),
        "only the admin should be able to force a transfer"
    );
}
