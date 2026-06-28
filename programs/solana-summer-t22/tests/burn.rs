mod common;
use common::*;

#[test]
fn normal_burn_reduces_balance_and_supply() {
    let (mut svm, admin, program_id) = setup();
    init_config(&mut svm, &admin, program_id);
    let mint = init_mint(&mut svm, &admin, program_id);

    let owner = funded_keypair(&mut svm);
    let from = fund_token_account(&mut svm, &mint.pubkey(), &owner.pubkey(), 1_000_000);
    set_mint_supply(&mut svm, &mint.pubkey(), 1_000_000);

    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_summer_t22::instruction::Burn { amount: 250_000 }.data(),
        solana_summer_t22::accounts::Burn {
            owner: owner.pubkey(),
            mint: mint.pubkey(),
            from,
            token_program: TOKEN_2022_ID,
        }
        .to_account_metas(None),
    );
    let t = tx(&svm, &[ix], &owner.pubkey(), &[&owner]);
    let res = svm.send_transaction(t);
    assert!(res.is_ok(), "burn failed: {:?}", res.err());

    assert_eq!(token_amount(&svm, &from), 750_000);
    assert_eq!(mint_supply(&svm, &mint.pubkey()), 750_000);
}
