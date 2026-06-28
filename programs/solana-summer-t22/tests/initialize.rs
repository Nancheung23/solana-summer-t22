mod common;
use common::*;

#[test]
fn initialize_sets_admin() {
    let (mut svm, admin, program_id) = setup();
    init_config(&mut svm, &admin, program_id);

    let config = read_config(&svm, &program_id);
    assert_eq!(config.admin, admin.pubkey());
    assert!(config.new_admin.is_none());
}

#[test]
fn initialize_rejects_non_admin() {
    let (mut svm, _admin, program_id) = setup();
    let intruder = funded_keypair(&mut svm);

    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_summer_t22::instruction::Initialize {}.data(),
        solana_summer_t22::accounts::Initialize {
            admin: intruder.pubkey(),
            config: config_pda(&program_id),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    let t = tx(&svm, &[ix], &intruder.pubkey(), &[&intruder]);
    assert!(
        svm.send_transaction(t).is_err(),
        "non-admin should not be able to initialize the config"
    );
}
