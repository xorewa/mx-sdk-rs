use multiversx_sc_scenario::imports::*;

use drwa_asset_manager::drwa_asset_manager_proxy::DrwaAssetManagerProxy;
use drwa_policy_registry::drwa_policy_registry_proxy::DrwaPolicyRegistryProxy;

const OWNER: TestAddress = TestAddress::new("owner");
const GOVERNANCE: TestAddress = TestAddress::new("governance");
const HOLDER: TestAddress = TestAddress::new("holder");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("drwa-asset-manager-upgrade");
const POLICY_SC_ADDRESS: TestSCAddress = TestSCAddress::new("drwa-policy-registry-upgrade");
const CODE_PATH: MxscPath = MxscPath::new("mxsc:output/drwa-asset-manager.mxsc.json");
const POLICY_CODE_PATH: MxscPath =
    MxscPath::new("mxsc:../policy-registry/output/drwa-policy-registry.mxsc.json");
const TOKEN_ID: &[u8] = b"HOTEL-ab12cd";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());
    blockchain.set_current_dir_from_workspace("contracts/drwa/asset-manager");
    blockchain.register_contract(CODE_PATH, drwa_asset_manager::ContractBuilder);
    blockchain.register_contract(POLICY_CODE_PATH, drwa_policy_registry::ContractBuilder);
    blockchain
}

#[test]
fn asset_manager_upgrade_preserves_asset_holder_and_storage_version() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);
    world.account(GOVERNANCE).nonce(1).balance(1_000_000u64);
    world.account(HOLDER).nonce(1).balance(1_000_000u64);

    world
        .tx()
        .from(OWNER)
        .typed(DrwaPolicyRegistryProxy)
        .init(GOVERNANCE)
        .code(POLICY_CODE_PATH)
        .new_address(POLICY_SC_ADDRESS)
        .run();

    world
        .tx()
        .from(OWNER)
        .typed(DrwaAssetManagerProxy)
        .init(GOVERNANCE)
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .run();

    world
        .tx()
        .from(GOVERNANCE)
        .to(SC_ADDRESS)
        .typed(DrwaAssetManagerProxy)
        .set_policy_registry_address(POLICY_SC_ADDRESS)
        .run();

    world
        .tx()
        .from(GOVERNANCE)
        .to(POLICY_SC_ADDRESS)
        .typed(DrwaPolicyRegistryProxy)
        .set_token_policy(
            ManagedBuffer::from(TOKEN_ID),
            true,
            false,
            false,
            false,
            ManagedVec::<StaticApi, ManagedBuffer<StaticApi>>::new(),
            ManagedVec::<StaticApi, ManagedBuffer<StaticApi>>::new(),
            false,
            false,
        )
        .run();

    world
        .tx()
        .from(GOVERNANCE)
        .to(SC_ADDRESS)
        .typed(DrwaAssetManagerProxy)
        .register_asset(
            ManagedBuffer::from(TOKEN_ID),
            ManagedBuffer::from(b"ESDT"),
            ManagedBuffer::from(b"Hospitality"),
        )
        .run();

    world
        .tx()
        .from(GOVERNANCE)
        .to(SC_ADDRESS)
        .typed(DrwaAssetManagerProxy)
        .sync_holder_compliance(
            ManagedBuffer::from(TOKEN_ID),
            HOLDER.to_managed_address(),
            ManagedBuffer::from(b"approved"),
            ManagedBuffer::from(b"clear"),
            ManagedBuffer::from(b"accredited"),
            ManagedBuffer::from(b"SG"),
            500u64,
            false,
            false,
            false,
            0u64,
            false,
            false,
            ManagedBuffer::new(),
            ManagedBuffer::new(),
            0u32,
        )
        .run();

    world
        .tx()
        .from(OWNER)
        .to(SC_ADDRESS)
        .typed(DrwaAssetManagerProxy)
        .upgrade()
        .code(CODE_PATH)
        .run();

    // These assertions intentionally use public views rather than `whitebox`.
    // The upgrade must be verified against the compiled Wasm artifact as well as
    // the debug executor, and `whitebox` only exists in the latter.
    let storage_version = world
        .query()
        .to(SC_ADDRESS)
        .raw_call("getStorageVersion")
        .returns(ReturnsRawResult)
        .run();
    // MultiversX top-level integer encoding is minimal-width, so `1u32` is
    // returned as the single byte `0x01` rather than four big-endian bytes.
    assert_eq!(storage_version, vec![vec![1u8]].into());

    let asset: drwa_asset_manager::AssetRecord<StaticApi> = world
        .query()
        .to(SC_ADDRESS)
        .typed(DrwaAssetManagerProxy)
        .asset(ManagedBuffer::from(TOKEN_ID))
        .returns(ReturnsResult)
        .run();
    assert!(asset.regulated);

    let mirror: drwa_common::DrwaHolderMirror<StaticApi> = world
        .query()
        .to(SC_ADDRESS)
        .typed(DrwaAssetManagerProxy)
        .get_holder_mirror(ManagedBuffer::from(TOKEN_ID), HOLDER.to_managed_address())
        .returns(ReturnsResult)
        .run();
    assert_eq!(mirror.holder_policy_version, 1u64);
    assert!(!mirror.auditor_authorized);
}
