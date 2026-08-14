use multiversx_sc_snippets::imports::{HttpInteractor, SetStateAccount};

// DRWA's current interactor calls `set_state` after contract deployment. In
// real-chain mode this SDK method deliberately returns success without issuing
// a request. The caller therefore cannot use its Ok result as proof that the
// native DRWA authorized-caller registry changed.
#[tokio::test]
async fn real_chain_set_state_returns_successful_noop() {
    let interactor = HttpInteractor::empty();
    assert!(!interactor.use_chain_simulator);

    let result = interactor
        .set_state(vec![SetStateAccount::from_address(
            "erd1lllllllllllllllllllllllllllllllllllllllllllllllllllsckry7t".to_owned(),
        )])
        .await;

    assert_eq!(result.unwrap(), "no-simulator");
}
