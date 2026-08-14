use drwa_auth_admin::DrwaAuthAdmin;
use multisig::{Multisig, multisig_proxy, multisig_view_proxy};
use multiversx_sc_scenario::imports::*;
use std::{fs, path::PathBuf};

mod compiled_auth_admin_proxy {
    use multiversx_sc::proxy_imports::*;

    pub struct DrwaAuthAdminProxy;

    impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for DrwaAuthAdminProxy
    where
        Env: TxEnv,
        From: TxFrom<Env>,
        To: TxTo<Env>,
        Gas: TxGas<Env>,
    {
        type TxProxyMethods = DrwaAuthAdminProxyMethods<Env, From, To, Gas>;

        fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
            DrwaAuthAdminProxyMethods { wrapped_tx: tx }
        }
    }

    pub struct DrwaAuthAdminProxyMethods<Env, From, To, Gas>
    where
        Env: TxEnv,
        From: TxFrom<Env>,
        To: TxTo<Env>,
        Gas: TxGas<Env>,
    {
        wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
    }

    impl<Env, From, Gas> DrwaAuthAdminProxyMethods<Env, From, (), Gas>
    where
        Env: TxEnv,
        Env::Api: VMApi,
        From: TxFrom<Env>,
        Gas: TxGas<Env>,
    {
        pub fn init<
            Arg0: ProxyArg<usize>,
            Arg1: ProxyArg<u64>,
            Arg2: ProxyArg<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>,
        >(
            self,
            quorum: Arg0,
            proposal_ttl_rounds: Arg1,
            signers: Arg2,
        ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
            self.wrapped_tx
                .payment(NotPayable)
                .raw_deploy()
                .argument(&quorum)
                .argument(&proposal_ttl_rounds)
                .argument(&signers)
                .original_result()
        }
    }

    impl<Env, From, To, Gas> DrwaAuthAdminProxyMethods<Env, From, To, Gas>
    where
        Env: TxEnv,
        Env::Api: VMApi,
        From: TxFrom<Env>,
        To: TxTo<Env>,
        Gas: TxGas<Env>,
    {
        pub fn action_proposer<Arg0: ProxyArg<u64>>(
            self,
            action_id: Arg0,
        ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
            self.wrapped_tx
                .payment(NotPayable)
                .raw_call("getActionProposer")
                .argument(&action_id)
                .original_result()
        }

        pub fn action_signers<Arg0: ProxyArg<u64>>(
            self,
            action_id: Arg0,
        ) -> TxTypedCall<
            Env,
            From,
            To,
            NotPayable,
            Gas,
            MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>,
        > {
            self.wrapped_tx
                .payment(NotPayable)
                .raw_call("getActionSigners")
                .argument(&action_id)
                .original_result()
        }

        pub fn action_approved_at_timestamp_seconds<Arg0: ProxyArg<u64>>(
            self,
            action_id: Arg0,
        ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
            self.wrapped_tx
                .payment(NotPayable)
                .raw_call("getActionApprovedAtTimestampSeconds")
                .argument(&action_id)
                .original_result()
        }
    }
}

const OWNER: TestAddress = TestAddress::new("owner");
const SAFE_MEMBER_ONE: TestAddress = TestAddress::new("safe-member-one");
const SAFE_MEMBER_TWO: TestAddress = TestAddress::new("safe-member-two");
const SAFE_MEMBER_THREE: TestAddress = TestAddress::new("safe-member-three");
const SAFE_MEMBER_FOUR: TestAddress = TestAddress::new("safe-member-four");
const SAFE_MEMBER_FIVE: TestAddress = TestAddress::new("safe-member-five");
const AUTH_SIGNER_TWO: TestAddress = TestAddress::new("auth-signer-two");
const AUTH_SIGNER_THREE: TestAddress = TestAddress::new("auth-signer-three");
const AUTH_SIGNER_FOUR: TestAddress = TestAddress::new("auth-signer-four");
const AUTH_SIGNER_FIVE: TestAddress = TestAddress::new("auth-signer-five");
const NEW_AUTH_SIGNER: TestAddress = TestAddress::new("new-auth-signer");

const SAFE_SC: TestSCAddress = TestSCAddress::new("safe");
const AUTH_ADMIN_SC: TestSCAddress = TestSCAddress::new("drwa-auth-admin");
const SAFE_CODE: MxscPath = MxscPath::new("examples/multisig/output/multisig-full.mxsc.json");
const AUTH_ADMIN_CODE: MxscPath =
    MxscPath::new("drwa/drwa-auth-admin/output/drwa-auth-admin.mxsc.json");

fn world() -> ScenarioWorld {
    let mut world = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());
    world.set_current_dir_from_workspace("contracts");
    world.register_contract(SAFE_CODE, multisig::ContractBuilder);
    world.register_contract(AUTH_ADMIN_CODE, drwa_auth_admin::ContractBuilder);
    world
}

fn deploy(world: &mut ScenarioWorld) {
    for account in [
        OWNER,
        SAFE_MEMBER_ONE,
        SAFE_MEMBER_TWO,
        SAFE_MEMBER_THREE,
        SAFE_MEMBER_FOUR,
        SAFE_MEMBER_FIVE,
        AUTH_SIGNER_TWO,
        AUTH_SIGNER_THREE,
        AUTH_SIGNER_FOUR,
        AUTH_SIGNER_FIVE,
        NEW_AUTH_SIGNER,
    ] {
        world.account(account).nonce(1);
    }

    world
        .tx()
        .from(OWNER)
        .raw_deploy()
        .code(SAFE_CODE)
        .new_address(SAFE_SC)
        .whitebox(multisig::contract_obj, |sc| {
            let mut members = ManagedVec::new();
            members.push(SAFE_MEMBER_ONE.to_managed_address());
            members.push(SAFE_MEMBER_TWO.to_managed_address());
            members.push(SAFE_MEMBER_THREE.to_managed_address());
            members.push(SAFE_MEMBER_FOUR.to_managed_address());
            members.push(SAFE_MEMBER_FIVE.to_managed_address());
            sc.init(3, members.into());
        });

    world
        .tx()
        .from(OWNER)
        .raw_deploy()
        .code(AUTH_ADMIN_CODE)
        .new_address(AUTH_ADMIN_SC)
        .whitebox(drwa_auth_admin::contract_obj, |sc| {
            let mut signers = ManagedVec::new();
            signers.push(SAFE_SC.to_managed_address());
            signers.push(AUTH_SIGNER_TWO.to_managed_address());
            signers.push(AUTH_SIGNER_THREE.to_managed_address());
            signers.push(AUTH_SIGNER_FOUR.to_managed_address());
            signers.push(AUTH_SIGNER_FIVE.to_managed_address());
            sc.init(3, 40_000, signers.into());
        });
}

// xSafe's DRWA transaction builder uses the generic multisig
// `proposeAsyncCall` endpoint. Exercise that exact action class here rather
// than the synchronous transfer-execute shortcut.
fn execute_safe_async_call(world: &mut ScenarioWorld, function_call: FunctionCall<StaticApi>) {
    let action_id: usize = world
        .tx()
        .from(SAFE_MEMBER_ONE)
        .to(SAFE_SC)
        .typed(multisig::multisig_proxy::MultisigProxy)
        .propose_async_call(AUTH_ADMIN_SC, 0u64, function_call)
        .returns(ReturnsResult)
        .run();

    world
        .tx()
        .from(SAFE_MEMBER_TWO)
        .to(SAFE_SC)
        .typed(multisig::multisig_proxy::MultisigProxy)
        .sign(action_id)
        .run();
    world
        .tx()
        .from(SAFE_MEMBER_THREE)
        .to(SAFE_SC)
        .typed(multisig::multisig_proxy::MultisigProxy)
        .sign(action_id)
        .run();
    world
        .tx()
        .from(SAFE_MEMBER_ONE)
        .to(SAFE_SC)
        .typed(multisig::multisig_proxy::MultisigProxy)
        .perform_action_endpoint(action_id)
        .run();
}

#[test]
fn safe_three_of_five_async_execution_counts_as_one_auth_admin_approval() {
    let mut world = world();
    deploy(&mut world);

    execute_safe_async_call(
        &mut world,
        FunctionCall::new("proposeAddSigner")
            .argument(&NEW_AUTH_SIGNER.to_managed_address::<StaticApi>()),
    );

    world
        .query()
        .to(AUTH_ADMIN_SC)
        .whitebox(drwa_auth_admin::contract_obj, |sc| {
            let action_id = 1u64;
            assert_eq!(sc.action_proposer(action_id).get(), SAFE_SC);
            assert_eq!(sc.current_action_signer_count(action_id), 1);
            assert!(
                sc.action_signers(action_id)
                    .contains(&SAFE_SC.to_managed_address())
            );
            assert!(
                !sc.action_signers(action_id)
                    .contains(&SAFE_MEMBER_ONE.to_managed_address())
            );
            assert!(
                !sc.action_signers(action_id)
                    .contains(&SAFE_MEMBER_TWO.to_managed_address())
            );
            assert!(
                !sc.action_signers(action_id)
                    .contains(&SAFE_MEMBER_THREE.to_managed_address())
            );
            assert!(
                sc.action_approved_at_timestamp_seconds(action_id)
                    .is_empty()
            );
        });

    execute_safe_async_call(&mut world, FunctionCall::new("sign").argument(&1u64));

    world
        .query()
        .to(AUTH_ADMIN_SC)
        .whitebox(drwa_auth_admin::contract_obj, |sc| {
            assert_eq!(sc.current_action_signer_count(1), 1);
            assert!(sc.action_approved_at_timestamp_seconds(1).is_empty());
        });
}

fn exact_xsafe_embedded_wasm() -> BytesValue {
    let constants_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../xSafe/src/helpers/constants.ts");
    let source = fs::read_to_string(&constants_path).unwrap_or_else(|err| {
        panic!(
            "cannot read xSafe embedded artifact at {}: {err}",
            constants_path.display()
        )
    });
    let marker = "export const smartContractCode =\n  '";
    let encoded = source
        .split_once(marker)
        .and_then(|(_, rest)| rest.split_once("';"))
        .map(|(value, _)| value)
        .expect("cannot locate xSafe smartContractCode literal");
    BytesValue::from_hex(encoded)
}

fn exact_auth_admin_wasm() -> BytesValue {
    let artifact_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("output/drwa-auth-admin.wasm");
    let bytes = fs::read(&artifact_path).unwrap_or_else(|err| {
        panic!(
            "cannot read current auth-admin artifact at {}; run `cargo run -- build --codehash` from its meta directory first: {err}",
            artifact_path.display()
        )
    });
    BytesValue::from(bytes)
}

fn perform_safe_action(world: &mut ScenarioWorld, action_id: usize) {
    world
        .tx()
        .from(SAFE_MEMBER_ONE)
        .to(SAFE_SC)
        .typed(multisig::multisig_proxy::MultisigProxy)
        .perform_action_endpoint(action_id)
        .run();
}

#[test]
fn exact_xsafe_and_auth_admin_wasm_preserve_single_outer_approval() {
    let mut world = ScenarioWorld::new().executor_config(ExecutorConfig::Experimental);
    for account in [
        OWNER,
        SAFE_MEMBER_ONE,
        SAFE_MEMBER_TWO,
        SAFE_MEMBER_THREE,
        SAFE_MEMBER_FOUR,
        SAFE_MEMBER_FIVE,
        AUTH_SIGNER_TWO,
        AUTH_SIGNER_THREE,
        AUTH_SIGNER_FOUR,
        AUTH_SIGNER_FIVE,
        NEW_AUTH_SIGNER,
    ] {
        world.account(account).nonce(1);
    }

    // Mirror xSafe's real constructor encoding: U8 quorum followed by the
    // deployer's address, producing its current generic one-of-one Safe.
    world
        .tx()
        .from(SAFE_MEMBER_ONE)
        .raw_deploy()
        .code(exact_xsafe_embedded_wasm())
        .argument(&1u8)
        .argument(&SAFE_MEMBER_ONE.to_managed_address::<StaticApi>())
        .new_address(SAFE_SC)
        .run();

    // Reach the requested 3-of-5 state using the embedded artifact's actual
    // add-member and quorum-change lifecycle, rather than direct test init.
    for member in [
        SAFE_MEMBER_TWO,
        SAFE_MEMBER_THREE,
        SAFE_MEMBER_FOUR,
        SAFE_MEMBER_FIVE,
    ] {
        let action_id: usize = world
            .tx()
            .from(SAFE_MEMBER_ONE)
            .to(SAFE_SC)
            .typed(multisig::multisig_proxy::MultisigProxy)
            .propose_add_board_member(member)
            .returns(ReturnsResult)
            .run();
        perform_safe_action(&mut world, action_id);
    }
    let quorum_action: usize = world
        .tx()
        .from(SAFE_MEMBER_ONE)
        .to(SAFE_SC)
        .typed(multisig::multisig_proxy::MultisigProxy)
        .propose_change_quorum(3usize)
        .returns(ReturnsResult)
        .run();
    perform_safe_action(&mut world, quorum_action);

    world
        .query()
        .to(SAFE_SC)
        .typed(multisig_proxy::MultisigProxy)
        .quorum()
        .returns(ExpectValue(3usize))
        .run();
    world
        .query()
        .to(SAFE_SC)
        .typed(multisig_view_proxy::MultisigProxy)
        .get_all_board_members()
        .returns(ExpectValue(MultiValueVec::from(vec![
            SAFE_MEMBER_ONE,
            SAFE_MEMBER_TWO,
            SAFE_MEMBER_THREE,
            SAFE_MEMBER_FOUR,
            SAFE_MEMBER_FIVE,
        ])))
        .run();

    let auth_signers = MultiValueVec::from(vec![
        SAFE_SC.to_managed_address::<StaticApi>(),
        AUTH_SIGNER_TWO.to_managed_address::<StaticApi>(),
        AUTH_SIGNER_THREE.to_managed_address::<StaticApi>(),
        AUTH_SIGNER_FOUR.to_managed_address::<StaticApi>(),
        AUTH_SIGNER_FIVE.to_managed_address::<StaticApi>(),
    ]);
    world
        .tx()
        .from(OWNER)
        .typed(compiled_auth_admin_proxy::DrwaAuthAdminProxy)
        .init(3usize, 40_000u64, auth_signers)
        .code(exact_auth_admin_wasm())
        .new_address(AUTH_ADMIN_SC)
        .run();

    let outer_action: usize = world
        .tx()
        .from(SAFE_MEMBER_ONE)
        .to(SAFE_SC)
        .typed(multisig_proxy::MultisigProxy)
        .propose_async_call(
            AUTH_ADMIN_SC,
            0u64,
            FunctionCall::new("proposeAddSigner")
                .argument(&NEW_AUTH_SIGNER.to_managed_address::<StaticApi>()),
        )
        .returns(ReturnsResult)
        .run();

    world
        .tx()
        .from(SAFE_MEMBER_ONE)
        .to(SAFE_SC)
        .typed(multisig_proxy::MultisigProxy)
        .perform_action_endpoint(outer_action)
        .returns(ExpectError(4u64, "quorum has not been reached"))
        .run();
    world
        .tx()
        .from(SAFE_MEMBER_TWO)
        .to(SAFE_SC)
        .typed(multisig_proxy::MultisigProxy)
        .sign(outer_action)
        .run();
    world
        .tx()
        .from(SAFE_MEMBER_ONE)
        .to(SAFE_SC)
        .typed(multisig_proxy::MultisigProxy)
        .perform_action_endpoint(outer_action)
        .returns(ExpectError(4u64, "quorum has not been reached"))
        .run();
    world
        .tx()
        .from(SAFE_MEMBER_THREE)
        .to(SAFE_SC)
        .typed(multisig_proxy::MultisigProxy)
        .sign(outer_action)
        .run();
    perform_safe_action(&mut world, outer_action);

    world
        .query()
        .to(AUTH_ADMIN_SC)
        .typed(compiled_auth_admin_proxy::DrwaAuthAdminProxy)
        .action_proposer(1u64)
        .returns(ExpectValue(SAFE_SC))
        .run();
    world
        .query()
        .to(AUTH_ADMIN_SC)
        .typed(compiled_auth_admin_proxy::DrwaAuthAdminProxy)
        .action_signers(1u64)
        .returns(ExpectValue(MultiValueVec::from(vec![SAFE_SC])))
        .run();
    // This endpoint is part of the current timestamp-timelock artifact. The
    // pre-B-03 round-timelock WASM does not export it, so a stale local output
    // cannot silently pass this compiled-artifact characterization.
    world
        .query()
        .to(AUTH_ADMIN_SC)
        .typed(compiled_auth_admin_proxy::DrwaAuthAdminProxy)
        .action_approved_at_timestamp_seconds(1u64)
        .returns(ExpectValue(0u64))
        .run();

    execute_safe_async_call(&mut world, FunctionCall::new("sign").argument(&1u64));
    world
        .query()
        .to(AUTH_ADMIN_SC)
        .typed(compiled_auth_admin_proxy::DrwaAuthAdminProxy)
        .action_signers(1u64)
        .returns(ExpectValue(MultiValueVec::from(vec![SAFE_SC])))
        .run();
}
