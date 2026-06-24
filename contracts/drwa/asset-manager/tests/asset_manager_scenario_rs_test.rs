use drwa_policy_registry;
use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());
    blockchain.set_current_dir_from_workspace("contracts/drwa/asset-manager");
    blockchain.register_partial_contract::<drwa_asset_manager::AbiProvider, _>(
        "mxsc:output/drwa-asset-manager.mxsc.json",
        drwa_asset_manager::ContractBuilder,
        "drwa-asset-manager",
    );
    blockchain.register_partial_contract::<drwa_policy_registry::AbiProvider, _>(
        "mxsc:../policy-registry/output/drwa-policy-registry.mxsc.json",
        drwa_policy_registry::ContractBuilder,
        "drwa-policy-registry",
    );
    blockchain
}

#[test]
fn asset_manager_init_rs() {
    world().run("scenarios/asset-manager-init.scen.json");
}

#[test]
fn asset_manager_denial_signals_rs() {
    world().run("scenarios/asset-manager-denial-signals.scen.json");
}
