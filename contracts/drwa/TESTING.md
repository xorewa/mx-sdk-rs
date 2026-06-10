# DRWA Test Classification

This directory intentionally uses several test styles. Do not mechanically
replace `ExecutorConfig::full_suite()` with a lighter executor just because a
test also performs whitebox assertions. Full-suite tests are the DRWA guardrail
against VM, Wasmer, and Supernova routing regressions.

Use this file when reviewing DRWA test failures or adding new tests.

## Categories

- `whitebox`: calls contract methods directly through `.whitebox(...)`; best for
  mapper-level assertions and panic-message coverage.
- `blackbox typed proxy`: executes through typed proxies and `.run()`; preserves
  endpoint ABI and transaction behavior.
- `scenario JSON`: runs `.scen.json` fixtures through `ScenarioWorld::run`.
- `VM/full-suite compatibility`: keeps `ExecutorConfig::full_suite()` to exercise
  the same VM routing path as generated contract artifacts.
- `gas bench`: uses the experimental executor for gas/profiling coverage.
- `chain simulator`: requires a running chain simulator and is ignored unless
  the matching feature is enabled.
- `plain Rust`: no scenario VM; validates codec/helper logic directly.
- `mixed`: combines full-suite VM setup with whitebox assertions. Keep these
  until a focused refactor splits VM coverage from mapper assertions.

## Current Inventory

| Test file | Current style | Keep / change guidance |
| --- | --- | --- |
| `asset-manager/tests/asset_manager_blackbox_test.rs` | blackbox typed proxy, full-suite VM | Keep full-suite coverage. |
| `asset-manager/tests/asset_manager_gas_bench_test.rs` | gas bench, experimental executor | Keep experimental executor. |
| `asset-manager/tests/asset_manager_integration_rs_test.rs` | mixed full-suite plus whitebox | Keep until split into typed-proxy integration and whitebox mapper tests. |
| `asset-manager/tests/asset_manager_scenario_go_test.rs` | scenario JSON, Go executor | Keep scenario fixture coverage. |
| `asset-manager/tests/asset_manager_scenario_rs_test.rs` | scenario JSON, full-suite Rust path | Keep full-suite scenario coverage. |
| `asset-manager/tests/asset_manager_upgrade_test.rs` | mixed upgrade flow | Keep full-suite upgrade coverage; split final whitebox storage checks only if needed. |
| `asset-manager/tests/asset_manager_whitebox_test.rs` | mixed whitebox, full-suite world | Candidate for debug/default executor if it is split away from VM coverage. |
| `attestation/tests/attestation_blackbox_test.rs` | blackbox typed proxy, full-suite VM | Keep full-suite coverage. |
| `attestation/tests/attestation_gas_bench_test.rs` | gas bench, experimental executor | Keep experimental executor. |
| `attestation/tests/attestation_scenario_go_test.rs` | scenario JSON, Go executor | Keep scenario fixture coverage. |
| `attestation/tests/attestation_scenario_rs_test.rs` | scenario JSON, full-suite Rust path | Keep full-suite scenario coverage. |
| `attestation/tests/attestation_upgrade_test.rs` | mixed upgrade flow | Keep full-suite upgrade coverage; split whitebox storage checks only if needed. |
| `attestation/tests/attestation_whitebox_test.rs` | mixed whitebox, full-suite world | Candidate for debug/default executor if it is split away from VM coverage. |
| `common/test-harness/tests/common_sync_gas_bench_test.rs` | gas bench, experimental executor | Keep experimental executor. |
| `common/test-harness/tests/common_validation_test.rs` | mixed validation whitebox, full-suite world | Candidate for debug/default executor after preserving at least one VM validation path. |
| `common/tests/drwa_lifecycle_integration_test.rs` | full-suite multi-contract lifecycle with whitebox checks | Keep as cross-contract VM compatibility coverage. |
| `common/tests/drwa_sync_envelope_codec_test.rs` | plain Rust codec/helper tests | Keep outside scenario VM. |
| `drwa-auth-admin/tests/drwa_auth_admin_property_test.rs` | mixed property-style whitebox, full-suite world | Keep until property coverage is separated from VM coverage. |
| `drwa-auth-admin/tests/drwa_auth_admin_scenario_rs_test.rs` | scenario JSON, full-suite Rust path, currently ignored for pre-B-03 fixtures | Keep ignored status until fixtures are migrated. |
| `drwa-auth-admin/tests/drwa_auth_admin_whitebox_test.rs` | mixed whitebox, full-suite world | Candidate for debug/default executor if split away from VM coverage. |
| `identity-registry/tests/identity_registry_blackbox_test.rs` | blackbox typed proxy, full-suite VM | Keep full-suite coverage. |
| `identity-registry/tests/identity_registry_gas_bench_test.rs` | gas bench, experimental executor | Keep experimental executor. |
| `identity-registry/tests/identity_registry_scenario_go_test.rs` | scenario JSON, Go executor | Keep scenario fixture coverage. |
| `identity-registry/tests/identity_registry_scenario_rs_test.rs` | scenario JSON, full-suite Rust path | Keep full-suite scenario coverage. |
| `identity-registry/tests/identity_registry_upgrade_test.rs` | mixed upgrade flow | Keep full-suite upgrade coverage; split whitebox storage checks only if needed. |
| `identity-registry/tests/identity_registry_whitebox_test.rs` | mixed whitebox, full-suite world | Candidate for debug/default executor if it is split away from VM coverage. |
| `interactor/tests/chain_simulator_drwa_test.rs` | chain simulator, feature-gated | Keep ignored unless `chain-simulator-tests` is enabled and simulator is running. |
| `policy-registry/tests/policy_registry_blackbox_test.rs` | blackbox typed proxy, full-suite VM | Keep full-suite coverage. |
| `policy-registry/tests/policy_registry_gas_bench_test.rs` | gas bench, experimental executor | Keep experimental executor. |
| `policy-registry/tests/policy_registry_integration_rs_test.rs` | mixed full-suite plus whitebox | Keep until split into typed-proxy integration and whitebox mapper tests. |
| `policy-registry/tests/policy_registry_scenario_go_test.rs` | scenario JSON, Go executor | Keep scenario fixture coverage. |
| `policy-registry/tests/policy_registry_scenario_rs_test.rs` | scenario JSON, full-suite Rust path | Keep full-suite scenario coverage. |
| `policy-registry/tests/policy_registry_upgrade_test.rs` | mixed upgrade flow | Keep full-suite upgrade coverage; split whitebox storage checks only if needed. |
| `policy-registry/tests/policy_registry_whitebox_test.rs` | mixed whitebox, full-suite world | Candidate for debug/default executor if it is split away from VM coverage. |

## Review Rules

1. Keep `ExecutorConfig::full_suite()` when the test is meant to exercise WASM,
   generated `.mxsc.json` artifacts, cross-contract routing, scenario execution,
   upgrade behavior, or Supernova compatibility.
2. Prefer a debug/default executor only for tests that are purely whitebox and
   do not need VM artifact coverage.
3. Split mixed tests instead of weakening them: keep one VM/full-suite test for
   execution compatibility and move mapper/storage assertions into a whitebox
   test only when there is a concrete failure or maintainability reason.
4. Do not rewrite `MxscPath::new("mxsc:...")` paths broadly. Fix path setup only
   when a failing test proves resolution is wrong for the intended working
   directory.
5. Keep Go and Rust scenario tests when both exist; they protect different
   execution paths.
6. Keep the chain-simulator suite feature-gated. It is an integration boundary,
   not a regular unit-test dependency.

## Supernova Readiness Checks

Before publishing a DRWA/Supernova SDK retrofit, run at least:

```bash
cargo test -p drwa-attestation --test attestation_upgrade_test attestation_upgrade_preserves_auditor_record_and_storage_version
cargo test -p drwa-asset-manager --test asset_manager_integration_rs_test asset_manager_init_rs
cargo test -p drwa-asset-manager --test asset_manager_blackbox_test asset_manager_blackbox_sync_holder_compliance
cargo test -p drwa-policy-registry --test policy_registry_integration_rs_test policy_registry_init_rs
```

For a broader DRWA pass, add the package-level blackbox, scenario, upgrade, and
common codec tests. Run chain-simulator tests only with a live simulator:

```bash
cargo test --features chain-simulator-tests -p drwa-interactor
```

## Native Hook Availability

DRWA native governance coverage depends on `managedDRWANativeGovernanceQuery`
being present in both the SDK-facing VM hook traits and the executor import
registration. If a DRWA test fails with a missing function/import error, check
the VM hook generator and Rust executor imports before weakening test coverage.
