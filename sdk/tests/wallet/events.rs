// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::wallet::events::types::{AddressData, TransactionProgressEvent, WalletEvent};

fn assert_serde_eq(event_0: WalletEvent) {
    let json = serde_json::to_string(&event_0).unwrap();
    println!("{json}");
    let event_1 = serde_json::from_str(&json).unwrap();

    assert_eq!(event_0, event_1);
}

#[test]
fn wallet_events_serde() {
    let event = WalletEvent::ConsolidationRequired;

    assert_serde_eq(event);

    let event = WalletEvent::LedgerAddressGeneration(AddressData {
        address: "rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy".to_string(),
    });

    assert_serde_eq(event);

    // let event = WalletEvent::NewOutput(NewOutputEvent {output:, transaction:None, transaction_inputs:None});

    // NewOutput(Box<NewOutputEvent>),

    // assert_serde_eq(event);
    // SpentOutput(Box<SpentOutputEvent>),

    // assert_serde_eq(event);
    // TransactionInclusion(TransactionInclusionEvent),

    // assert_serde_eq(event);
    // TransactionProgress(TransactionProgressEvent),

    let event = WalletEvent::TransactionProgress(TransactionProgressEvent::Broadcasting);

    assert_serde_eq(event);
}
