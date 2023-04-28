// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    types::block::{
        address::{Address, Ed25519Address},
        output::{dto::OutputMetadataDto, Output, OutputId},
        payload::transaction::TransactionId,
        rand::output::{rand_basic_output, rand_output_metadata},
    },
    wallet::{
        account::types::{InclusionState, OutputData, OutputDataDto},
        events::types::{
            AddressData, NewOutputEvent, SpentOutputEvent, TransactionInclusionEvent, TransactionProgressEvent,
            WalletEvent,
        },
    },
};

fn assert_serde_eq(event_0: WalletEvent) {
    let json = serde_json::to_string(&event_0).unwrap();
    let event_1 = serde_json::from_str(&json).unwrap();

    assert_eq!(event_0, event_1);
}

#[test]
fn wallet_events_serde() {
    assert_serde_eq(WalletEvent::ConsolidationRequired);

    #[cfg(feature = "ledger_nano")]
    assert_serde_eq(WalletEvent::LedgerAddressGeneration(AddressData {
        address: "rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy".to_string(),
    }));

    let output_data_dto = OutputDataDto::from(&OutputData {
        output_id: OutputId::null(),
        metadata: OutputMetadataDto::from(&rand_output_metadata()),
        output: Output::from(rand_basic_output(1_813_620_509_061_365)),
        is_spent: false,
        address: Address::Ed25519(Ed25519Address::new([0; Ed25519Address::LENGTH])),
        network_id: 42,
        remainder: true,
        chain: None,
    });

    assert_serde_eq(WalletEvent::NewOutput(Box::new(NewOutputEvent {
        output: output_data_dto.clone(),
        transaction: None,
        transaction_inputs: None,
    })));

    assert_serde_eq(WalletEvent::SpentOutput(Box::new(SpentOutputEvent {
        output: output_data_dto,
    })));

    assert_serde_eq(WalletEvent::TransactionInclusion(TransactionInclusionEvent {
        transaction_id: TransactionId::null(),
        inclusion_state: InclusionState::Conflicting,
    }));

    assert_serde_eq(WalletEvent::TransactionProgress(TransactionProgressEvent::Broadcasting));
}
