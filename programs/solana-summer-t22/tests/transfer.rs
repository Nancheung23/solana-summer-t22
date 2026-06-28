mod common;
use common::*;

#[test]
fn normal_transfer_moves_tokens() {
    let (mut svm, admin, program_id) = setup();
    init_config(&mut svm, &admin, program_id);
    let mint = init_mint(&mut svm, &admin, program_id);

    let owner = funded_keypair(&mut svm);
    let from = fund_token_account(&mut svm, &mint.pubkey(), &owner.pubkey(), 1_000_000);
    let to = fund_token_account(&mut svm, &mint.pubkey(), &Pubkey::new_unique(), 0);

    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_summer_t22::instruction::Transfer { amount: 400_000 }.data(),
        solana_summer_t22::accounts::Transfer {
            owner: owner.pubkey(),
            mint: mint.pubkey(),
            from,
            to,
            token_program: TOKEN_2022_ID,
        }
        .to_account_metas(None),
    );
    let t = tx(&svm, &[ix], &owner.pubkey(), &[&owner]);
    let res = svm.send_transaction(t);
    assert!(res.is_ok(), "transfer failed: {:?}", res.err());

    assert_eq!(token_amount(&svm, &from), 600_000);
    assert_eq!(token_amount(&svm, &to), 400_000);
}

#[test]
fn normal_transfer_requires_owner_signature() {
    let (mut svm, admin, program_id) = setup();
    init_config(&mut svm, &admin, program_id);
    let mint = init_mint(&mut svm, &admin, program_id);

    let owner = funded_keypair(&mut svm);
    let attacker = funded_keypair(&mut svm);
    let from = fund_token_account(&mut svm, &mint.pubkey(), &owner.pubkey(), 1_000_000);
    let to = fund_token_account(&mut svm, &mint.pubkey(), &Pubkey::new_unique(), 0);

    // attacker signs but doesn't own `from`.
    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_summer_t22::instruction::Transfer { amount: 400_000 }.data(),
        solana_summer_t22::accounts::Transfer {
            owner: attacker.pubkey(),
            mint: mint.pubkey(),
            from,
            to,
            token_program: TOKEN_2022_ID,
        }
        .to_account_metas(None),
    );
    let t = tx(&svm, &[ix], &attacker.pubkey(), &[&attacker]);
    assert!(
        svm.send_transaction(t).is_err(),
        "transfer should require the token account owner"
    );
}
