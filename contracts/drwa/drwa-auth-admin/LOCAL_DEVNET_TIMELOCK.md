# Local-devnet timelock profile

`drwa-auth-admin` defaults to the production governance delays:

- ordinary governance actions: 24 hours;
- `recovery_admin` caller-slot actions: 48 hours.

For a disposable local DRWA devnet only, the explicit Cargo feature
`local-test-timelock` builds a *different* contract artifact with 5-minute and
10-minute delays respectively. It exists only to make end-to-end governance
tests practical; it must never be used on public Devnet, Testnet, or Mainnet.

From the `mx-sdk-rs` repository root, build that local-only Wasm with:

```bash
cargo build --manifest-path contracts/drwa/drwa-auth-admin/wasm/Cargo.toml \
  --release --target wasm32v1-none \
  --features drwa-auth-admin/local-test-timelock
```

This command only builds an artifact. Deploying it, configuring its unique code
hash in an isolated local policy registry, and exercising the governance flow
are separate deliberate steps.

The feature changes the Wasm code and therefore its code hash. A local
deployment must use its own policy-registry/code-hash configuration and must
not reuse a production deployment manifest or code-hash allow-list.

Before any public deployment, build without this feature and verify the
production 24h/48h constants and corresponding approved code hash. Pending
actions retain the delay captured when proposed, so changing a later build does
not shorten an already-created action.

See `ISSUE-274` for the required removal after local governance testing is
complete.
