use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::Experimental);
    blockchain.set_current_dir_from_workspace("contracts/drwa/drwa-auth-admin");
    blockchain
}

// These scenarios run with the crate's explicitly local-only accelerated
// timelock feature in tests. They still use the production signer floor and
// quorum (five signers, three approvals), and advance block timestamps past
// the production 24-hour delay before successful execution. Experimental
// execution deliberately avoids ContractBuilder registration: these scenarios
// must exercise the compiled artifact named by each scenario file.

#[test]
fn drwa_auth_admin_init_rs() {
    world().run("scenarios/drwa-auth-admin-init.scen.json");
}

#[test]
fn drwa_auth_admin_denial_signals_rs() {
    world().run("scenarios/drwa-auth-admin-denial-signals.scen.json");
}

#[test]
fn drwa_auth_admin_change_quorum_rs() {
    world().run("scenarios/drwa-auth-admin-change-quorum.scen.json");
}

#[test]
fn drwa_auth_admin_replay_protection_rs() {
    world().run("scenarios/drwa-auth-admin-replay-protection.scen.json");
}

#[test]
fn drwa_auth_admin_add_remove_signer_rs() {
    world().run("scenarios/drwa-auth-admin-add-remove-signer.scen.json");
}
