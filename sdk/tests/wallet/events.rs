// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::api::PreparedTransactionDataDto,
    types::block::{
        address::{Address, Bech32Address, Ed25519Address},
        core::{
            basic::{MaxBurnedManaAmount, StrongParents},
            BlockHeader,
        },
        input::{Input, UtxoInput},
        output::{
            unlock_condition::AddressUnlockCondition, AccountId, BasicOutput, LeafHash, Output, OutputCommitmentProof,
            OutputIdProof,
        },
        payload::{
            signed_transaction::{Transaction, TransactionHash, TransactionId},
            Payload, SignedTransactionPayload,
        },
        protocol::iota_mainnet_protocol_parameters,
        rand::{
            mana::rand_mana_allotment,
            output::{rand_basic_output, rand_output_metadata},
        },
        slot::{SlotCommitmentId, SlotIndex},
        unlock::{EmptyUnlock, Unlock, Unlocks},
        BlockBody, BlockId, UnsignedBlock,
    },
    wallet::{
        events::types::{
            AddressData, NewOutputEvent, SpentOutputEvent, TransactionInclusionEvent, TransactionProgressEvent,
            WalletEvent,
        },
        types::{InclusionState, OutputData},
    },
};
use pretty_assertions::assert_eq;

const ED25519_ADDRESS: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649";
const TRANSACTION_ID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64900000000";

fn assert_serde_eq(event_0: WalletEvent) {
    let json = serde_json::to_string(&event_0).unwrap();
    let event_1 = serde_json::from_str(&json).unwrap();

    assert_eq!(event_0, event_1);
}

#[test]
fn wallet_events_serde() {
    #[cfg(feature = "ledger_nano")]
    assert_serde_eq(WalletEvent::LedgerAddressGeneration(AddressData {
        address: Bech32Address::try_from_str("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy")
            .unwrap(),
    }));

    let output_data = OutputData {
        output_id: TransactionHash::null().into_transaction_id(0).into_output_id(0),
        metadata: rand_output_metadata(),
        output: Output::from(rand_basic_output(1_813_620_509_061_365)),
        output_id_proof: OutputIdProof {
            slot: SlotIndex(1),
            output_index: 0,
            transaction_commitment: [0u8; 32],
            output_commitment_proof: OutputCommitmentProof::Leaf(LeafHash([0u8; 32])),
        },
        network_id: 42,
        remainder: true,
    };

    assert_serde_eq(WalletEvent::NewOutput(Box::new(NewOutputEvent {
        output: output_data.clone(),
        transaction: None,
        transaction_inputs: None,
    })));

    assert_serde_eq(WalletEvent::SpentOutput(Box::new(SpentOutputEvent {
        output: output_data,
    })));

    assert_serde_eq(WalletEvent::TransactionInclusion(TransactionInclusionEvent {
        transaction_id: TransactionHash::null().into_transaction_id(0),
        inclusion_state: InclusionState::Conflicting,
    }));

    assert_serde_eq(WalletEvent::TransactionProgress(
        TransactionProgressEvent::BuildingTransaction,
    ));

    assert_serde_eq(WalletEvent::TransactionProgress(
        TransactionProgressEvent::GeneratingRemainderDepositAddress(AddressData {
            address: Bech32Address::try_from_str("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy")
                .unwrap(),
        }),
    ));

    {
        let protocol_parameters = iota_mainnet_protocol_parameters();
        let transaction_id = TransactionId::new(prefix_hex::decode(TRANSACTION_ID).unwrap());
        let input1 = Input::Utxo(UtxoInput::new(transaction_id, 0));
        let input2 = Input::Utxo(UtxoInput::new(transaction_id, 1));
        let bytes: [u8; 32] = prefix_hex::decode(ED25519_ADDRESS).unwrap();
        let address = Address::from(Ed25519Address::new(bytes));
        let amount = 1_000_000;
        let output = Output::Basic(
            BasicOutput::build_with_amount(amount)
                .add_unlock_condition(AddressUnlockCondition::new(address))
                .finish()
                .unwrap(),
        );
        let transaction = Transaction::builder(protocol_parameters.network_id())
            .with_inputs(vec![input1, input2])
            .add_output(output)
            .add_mana_allotment(rand_mana_allotment(protocol_parameters))
            .finish_with_params(protocol_parameters)
            .unwrap();

        assert_serde_eq(WalletEvent::TransactionProgress(
            TransactionProgressEvent::PreparedTransaction(Box::new(PreparedTransactionDataDto {
                transaction: (&transaction).into(),
                inputs_data: Vec::new(),
                remainders: Vec::new(),
                mana_rewards: Default::default(),
                issuer_id: None,
            })),
        ));

        let block_payload = SignedTransactionPayload::new(
            transaction,
            Unlocks::new([Unlock::Empty(EmptyUnlock), Unlock::Empty(EmptyUnlock)]).unwrap(),
        )
        .unwrap();
        let payload = Payload::from(block_payload);
        let block = UnsignedBlock::new(
            BlockHeader::new(
                protocol_parameters.version(),
                protocol_parameters.network_id(),
                0u64,
                SlotCommitmentId::new([0; 36]),
                SlotIndex(0),
                AccountId::new([0; 32]),
            ),
            BlockBody::build_basic(
                StrongParents::from_vec(vec![BlockId::new([0; 36])]).unwrap(),
                MaxBurnedManaAmount::Amount(0),
            )
            .with_payload(payload)
            .finish_block_body()
            .unwrap(),
        );

        assert_serde_eq(WalletEvent::TransactionProgress(
            TransactionProgressEvent::PreparedBlockSigningInput(prefix_hex::encode(block.signing_input())),
        ));
    }

    assert_serde_eq(WalletEvent::TransactionProgress(
        TransactionProgressEvent::SigningTransaction,
    ));

    assert_serde_eq(WalletEvent::TransactionProgress(
        TransactionProgressEvent::PreparedTransactionSigningHash(ED25519_ADDRESS.to_string()),
    ));

    assert_serde_eq(WalletEvent::TransactionProgress(TransactionProgressEvent::Broadcasting));
}
