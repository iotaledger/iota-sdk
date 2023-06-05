// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::api::PreparedTransactionDataDto,
    types::block::{
        address::{Address, Bech32Address, Ed25519Address},
        input::{Input, UtxoInput},
        output::{unlock_condition::AddressUnlockCondition, BasicOutput, Output, OutputId},
        payload::transaction::{RegularTransactionEssence, TransactionEssence, TransactionId},
        protocol::protocol_parameters,
        rand::output::{rand_basic_output, rand_inputs_commitment, rand_output_metadata},
    },
    wallet::{
        account::types::{InclusionState, OutputData, OutputDataDto},
        events::types::{
            AddressData, NewOutputEvent, SpentOutputEvent, TransactionInclusionEvent, TransactionProgressEvent,
            WalletEvent,
        },
    },
};

const ED25519_ADDRESS: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649";
const TRANSACTION_ID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649";

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
        address: Bech32Address::try_from_str("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy")
            .unwrap(),
    }));

    let output_data_dto = OutputDataDto::from(&OutputData {
        output_id: OutputId::null(),
        metadata: rand_output_metadata(),
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

    assert_serde_eq(WalletEvent::TransactionProgress(
        TransactionProgressEvent::SelectingInputs,
    ));

    assert_serde_eq(WalletEvent::TransactionProgress(
        TransactionProgressEvent::GeneratingRemainderDepositAddress(AddressData {
            address: Bech32Address::try_from_str("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy")
                .unwrap(),
        }),
    ));

    {
        let protocol_parameters = protocol_parameters();
        let transaction_id = TransactionId::new(prefix_hex::decode(TRANSACTION_ID).unwrap());
        let input1 = Input::Utxo(UtxoInput::new(transaction_id, 0).unwrap());
        let input2 = Input::Utxo(UtxoInput::new(transaction_id, 1).unwrap());
        let bytes: [u8; 32] = prefix_hex::decode(ED25519_ADDRESS).unwrap();
        let address = Address::from(Ed25519Address::new(bytes));
        let amount = 1_000_000;
        let output = Output::Basic(
            BasicOutput::build_with_amount(amount)
                .add_unlock_condition(AddressUnlockCondition::new(address))
                .finish(protocol_parameters.token_supply())
                .unwrap(),
        );
        let essence = TransactionEssence::Regular(
            RegularTransactionEssence::builder(protocol_parameters.network_id(), rand_inputs_commitment())
                .with_inputs(vec![input1, input2])
                .add_output(output)
                .finish(&protocol_parameters)
                .unwrap(),
        );

        assert_serde_eq(WalletEvent::TransactionProgress(
            TransactionProgressEvent::PreparedTransaction(Box::new(PreparedTransactionDataDto {
                essence: (&essence).into(),
                inputs_data: Vec::new(),
                remainder: None,
            })),
        ));
    }

    assert_serde_eq(WalletEvent::TransactionProgress(
        TransactionProgressEvent::PreparedTransactionEssenceHash(ED25519_ADDRESS.to_string()),
    ));

    assert_serde_eq(WalletEvent::TransactionProgress(
        TransactionProgressEvent::SigningTransaction,
    ));

    assert_serde_eq(WalletEvent::TransactionProgress(
        TransactionProgressEvent::PerformingPow,
    ));

    assert_serde_eq(WalletEvent::TransactionProgress(TransactionProgressEvent::Broadcasting));
}
