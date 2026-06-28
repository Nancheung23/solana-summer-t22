mod common;
use common::*;

#[test]
fn admin_handoff_propose_then_confirm() {
    let (mut svm, admin, program_id) = setup();
    init_config(&mut svm, &admin, program_id);
    let new_admin = funded_keypair(&mut svm);

    // propose
    let propose = Instruction::new_with_bytes(
        program_id,
        &solana_summer_t22::instruction::ProposeAdmin {
            new_admin: new_admin.pubkey(),
        }
        .data(),
        solana_summer_t22::accounts::ProposeAdmin {
            admin: admin.pubkey(),
            config: config_pda(&program_id),
        }
        .to_account_metas(None),
    );
    let t = tx(&svm, &[propose], &admin.pubkey(), &[&admin]);
    svm.send_transaction(t).expect("propose_admin failed");

    let config = read_config(&svm, &program_id);
    assert_eq!(config.admin, admin.pubkey());
    assert_eq!(config.new_admin, Some(new_admin.pubkey()));

    // confirm
    let confirm = Instruction::new_with_bytes(
        program_id,
        &solana_summer_t22::instruction::ConfirmAdmin {}.data(),
        solana_summer_t22::accounts::ConfirmAdmin {
            new_admin: new_admin.pubkey(),
            config: config_pda(&program_id),
        }
        .to_account_metas(None),
    );
    let t = tx(&svm, &[confirm], &new_admin.pubkey(), &[&new_admin]);
    svm.send_transaction(t).expect("confirm_admin failed");

    let config = read_config(&svm, &program_id);
    assert_eq!(config.admin, new_admin.pubkey());
    assert!(config.new_admin.is_none());
}

#[test]
fn confirm_admin_rejects_wrong_signer() {
    let (mut svm, admin, program_id) = setup();
    init_config(&mut svm, &admin, program_id);
    let new_admin = funded_keypair(&mut svm);
    let attacker = funded_keypair(&mut svm);

    let propose = Instruction::new_with_bytes(
        program_id,
        &solana_summer_t22::instruction::ProposeAdmin {
            new_admin: new_admin.pubkey(),
        }
        .data(),
        solana_summer_t22::accounts::ProposeAdmin {
            admin: admin.pubkey(),
            config: config_pda(&program_id),
        }
        .to_account_metas(None),
    );
    let t = tx(&svm, &[propose], &admin.pubkey(), &[&admin]);
    svm.send_transaction(t).expect("propose_admin failed");

    // Someone other than the proposed admin tries to confirm.
    let confirm = Instruction::new_with_bytes(
        program_id,
        &solana_summer_t22::instruction::ConfirmAdmin {}.data(),
        solana_summer_t22::accounts::ConfirmAdmin {
            new_admin: attacker.pubkey(),
            config: config_pda(&program_id),
        }
        .to_account_metas(None),
    );
    let t = tx(&svm, &[confirm], &attacker.pubkey(), &[&attacker]);
    assert!(
        svm.send_transaction(t).is_err(),
        "only the proposed admin should be able to confirm"
    );
}
