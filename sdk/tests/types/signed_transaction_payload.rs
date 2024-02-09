// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::{
    address::{Address, Ed25519Address},
    input::{Input, UtxoInput},
    output::{unlock_condition::AddressUnlockCondition, BasicOutput, Output},
    payload::signed_transaction::{SignedTransactionPayload, Transaction, TransactionId},
    protocol::iota_mainnet_v3_protocol_parameters,
    rand::mana::rand_mana_allotment,
    signature::{Ed25519Signature, Signature},
    unlock::{ReferenceUnlock, SignatureUnlock, Unlock, Unlocks},
    Error,
};
use packable::PackableExt;
use pretty_assertions::assert_eq;

const TRANSACTION_ID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64900000000";
const ED25519_ADDRESS: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649";
const ED25519_PUBLIC_KEY: &str = "0x1da5ddd11ba3f961acab68fafee3177d039875eaa94ac5fdbff8b53f0c50bfb9";
const ED25519_SIGNATURE: &str = "0xc6a40edf9a089f42c18f4ebccb35fe4b578d93b879e99b87f63573324a710d3456b03fb6d1fcc027e6401cbd9581f790ee3ed7a3f68e9c225fcb9f1cd7b7110d";

#[test]
fn kind() {
    assert_eq!(SignedTransactionPayload::KIND, 1);
}

// Validate that attempting to construct a `SignedTransactionPayload` with too few unlocks is an error.
#[test]
fn builder_too_few_unlocks() {
    let protocol_parameters = iota_mainnet_v3_protocol_parameters();
    // Construct a transaction with two inputs and one output.
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
        .with_inputs([input1, input2])
        .add_output(output)
        .add_mana_allotment(rand_mana_allotment(&protocol_parameters))
        .finish_with_params(protocol_parameters)
        .unwrap();

    // Construct a list with a single unlock, whereas we have 2 tx inputs.
    let pub_key_bytes = prefix_hex::decode(ED25519_PUBLIC_KEY).unwrap();
    let sig_bytes = prefix_hex::decode(ED25519_SIGNATURE).unwrap();
    let signature = Ed25519Signature::from_bytes(pub_key_bytes, sig_bytes);
    let sig_unlock = Unlock::from(SignatureUnlock::from(Signature::from(signature)));
    let unlocks = Unlocks::new([sig_unlock]).unwrap();

    assert!(matches!(
            SignedTransactionPayload::new(transaction, unlocks),
            Err(Error::InputUnlockCountMismatch{input_count, unlock_count})
            if input_count == 2 && unlock_count == 1));
}

// Validate that attempting to construct a `SignedTransactionPayload` with too many unlocks is an error.
#[test]
fn builder_too_many_unlocks() {
    let protocol_parameters = iota_mainnet_v3_protocol_parameters();
    // Construct a transaction with one input and one output.
    let transaction_id = TransactionId::new(prefix_hex::decode(TRANSACTION_ID).unwrap());
    let input1 = Input::Utxo(UtxoInput::new(transaction_id, 0));
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
        .add_input(input1)
        .add_output(output)
        .add_mana_allotment(rand_mana_allotment(&protocol_parameters))
        .finish_with_params(protocol_parameters)
        .unwrap();

    // Construct a list of two unlocks, whereas we only have 1 tx input.
    let pub_key_bytes = prefix_hex::decode(ED25519_PUBLIC_KEY).unwrap();
    let sig_bytes = prefix_hex::decode(ED25519_SIGNATURE).unwrap();
    let signature = Ed25519Signature::from_bytes(pub_key_bytes, sig_bytes);
    let sig_unlock = Unlock::from(SignatureUnlock::from(Signature::from(signature)));
    let ref_unlock = Unlock::from(ReferenceUnlock::new(0).unwrap());

    let unlocks = Unlocks::new([sig_unlock, ref_unlock]).unwrap();

    assert!(matches!(
            SignedTransactionPayload::new(transaction, unlocks),
            Err(Error::InputUnlockCountMismatch{input_count, unlock_count})
            if input_count == 1 && unlock_count == 2));
}

// Validate that a `unpack` ∘ `pack` round-trip results in the original block.
#[test]
fn pack_unpack_valid() {
    // Construct a transaction with two inputs and one output.
    let protocol_parameters = iota_mainnet_v3_protocol_parameters();
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
        .with_inputs([input1, input2])
        .add_output(output)
        .add_mana_allotment(rand_mana_allotment(&protocol_parameters))
        .finish_with_params(protocol_parameters)
        .unwrap();

    // Construct a list of two unlocks, whereas we only have 1 tx input.
    let pub_key_bytes = prefix_hex::decode(ED25519_PUBLIC_KEY).unwrap();
    let sig_bytes = prefix_hex::decode(ED25519_SIGNATURE).unwrap();
    let signature = Ed25519Signature::from_bytes(pub_key_bytes, sig_bytes);
    let sig_unlock = Unlock::from(SignatureUnlock::from(Signature::from(signature)));
    let ref_unlock = Unlock::from(ReferenceUnlock::new(0).unwrap());
    let unlocks = Unlocks::new([sig_unlock, ref_unlock]).unwrap();

    let tx_payload = SignedTransactionPayload::new(transaction, unlocks).unwrap();
    let packed_tx_payload = tx_payload.pack_to_vec();

    assert_eq!(packed_tx_payload.len(), tx_payload.packed_len());
    assert_eq!(
        tx_payload,
        PackableExt::unpack_bytes_verified(packed_tx_payload.as_slice(), protocol_parameters).unwrap()
    );
}

#[test]
fn getters() {
    let protocol_parameters = iota_mainnet_v3_protocol_parameters();
    // Construct a transaction with two inputs and one output.
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
        .with_inputs([input1, input2])
        .add_output(output)
        .add_mana_allotment(rand_mana_allotment(&protocol_parameters))
        .finish_with_params(protocol_parameters)
        .unwrap();

    // Construct a list of two unlocks, whereas we only have 1 tx input.
    let pub_key_bytes = prefix_hex::decode(ED25519_PUBLIC_KEY).unwrap();
    let sig_bytes = prefix_hex::decode(ED25519_SIGNATURE).unwrap();
    let signature = Ed25519Signature::from_bytes(pub_key_bytes, sig_bytes);
    let sig_unlock = Unlock::from(SignatureUnlock::from(Signature::from(signature)));
    let ref_unlock = Unlock::from(ReferenceUnlock::new(0).unwrap());
    let unlocks = Unlocks::new([sig_unlock, ref_unlock]).unwrap();

    let tx_payload = SignedTransactionPayload::new(transaction.clone(), unlocks.clone()).unwrap();

    assert_eq!(*tx_payload.transaction(), transaction);
    assert_eq!(*tx_payload.unlocks(), unlocks);
}
