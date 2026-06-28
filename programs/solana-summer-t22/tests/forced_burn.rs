mod common;
use common::*;

#[test]
fn forced_burn_by_delegate() {
    let (mut svm, admin, program_id) = setup();
    init_config(&mut svm, &admin, program_id);
    let mint = init_mint(&mut svm, &admin, program_id);

    let victim = Pubkey::new_unique();
    let from = fund_token_account(&mut svm, &mint.pubkey(), &victim, 1_000_000);
    set_mint_supply(&mut svm, &mint.pubkey(), 1_000_000);

    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_summer_t22::instruction::ForcedBurn { amount: 600_000 }.data(),
        solana_summer_t22::accounts::ForcedBurn {
            admin: admin.pubkey(),
            config: config_pda(&program_id),
            mint: mint.pubkey(),
            from,
            authority: authority_pda(&program_id),
            token_program: TOKEN_2022_ID,
        }
        .to_account_metas(None),
    );
    let t = tx(&svm, &[ix], &admin.pubkey(), &[&admin]);
    let res = svm.send_transaction(t);
    assert!(res.is_ok(), "forced_burn failed: {:?}", res.err());

    assert_eq!(token_amount(&svm, &from), 400_000);
    assert_eq!(mint_supply(&svm, &mint.pubkey()), 400_000);
}
