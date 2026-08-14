use drwa_asset_manager::drwa_asset_manager_proxy::DrwaAssetManagerProxy;
use multiversx_sc_scenario::imports::*;

const DEPLOYER_OWNER: TestAddress = TestAddress::new("deployer-owner");
const BOUNDED_ADAPTER: TestAddress = TestAddress::new("bounded-adapter");
const ASSET_MANAGER: TestSCAddress = TestSCAddress::new("drwa-asset-manager");
const POLICY_REGISTRY: TestSCAddress = TestSCAddress::new("drwa-policy-registry");
const CODE_PATH: MxscPath = MxscPath::new("asset-manager/output/drwa-asset-manager.mxsc.json");

// Characterizes the current init/bootstrap boundary. When governance is set to
// the future bounded-adapter address at deployment, the deployer/chain owner is
// no longer authorized to perform the separately required policy-registry
// binding. This is evidence for changing the fresh-profile init shape; it is not
// an acceptance test for that future implementation.
#[test]
fn configured_adapter_governance_blocks_deployer_policy_registry_bootstrap() {
    let mut world = ScenarioWorld::new();
    world.set_current_dir_from_workspace("contracts/drwa");
    world.register_contract(CODE_PATH, drwa_asset_manager::ContractBuilder);
    world.account(DEPLOYER_OWNER).nonce(1).balance(1_000_000u64);
    world.account(BOUNDED_ADAPTER).nonce(1);

    world
        .tx()
        .from(DEPLOYER_OWNER)
        .typed(DrwaAssetManagerProxy)
        .init(BOUNDED_ADAPTER)
        .code(CODE_PATH)
        .new_address(ASSET_MANAGER)
        .run();

    world
        .tx()
        .from(DEPLOYER_OWNER)
        .to(ASSET_MANAGER)
        .typed(DrwaAssetManagerProxy)
        .set_policy_registry_address(POLICY_REGISTRY)
        .returns(ExpectError(4u64, "caller not authorized"))
        .run();

    // The configured governance address can perform the call, proving that the
    // failure is the bootstrap authority topology rather than the endpoint or
    // address format. A genesis-bound Safe has no approved action at this point.
    world
        .tx()
        .from(BOUNDED_ADAPTER)
        .to(ASSET_MANAGER)
        .typed(DrwaAssetManagerProxy)
        .set_policy_registry_address(POLICY_REGISTRY)
        .run();
}
