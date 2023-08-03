// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::{
    address::{Address, Ed25519Address},
    input::{Input, UtxoInput},
    mana::Allotment,
    output::{unlock_condition::AddressUnlockCondition, BasicOutput, Output},
    payload::{
        transaction::{RegularTransactionEssence, TransactionEssence, TransactionId, TransactionPayload},
        Payload, TaggedDataPayload,
    },
    protocol::protocol_parameters,
    rand::{
        bytes::rand_bytes,
        output::{rand_account_id, rand_inputs_commitment},
    },
    signature::{Ed25519Signature, Signature},
    unlock::{ReferenceUnlock, SignatureUnlock, Unlock, Unlocks},
};
use packable::PackableExt;

const TRANSACTION_ID: &str = "0x24a1f46bdb6b2bf38f1c59f73cdd4ae5b418804bb231d76d06fbf246498d5883";
const ED25519_ADDRESS: &str = "0xe594f9a895c0e0a6760dd12cffc2c3d1e1cbf7269b328091f96ce3d0dd550b75";
const ED25519_PUBLIC_KEY: &str = "0x1da5ddd11ba3f961acab68fafee3177d039875eaa94ac5fdbff8b53f0c50bfb9";
const ED25519_SIGNATURE: &str = "0xc6a40edf9a089f42c18f4ebccb35fe4b578d93b879e99b87f63573324a710d3456b03fb6d1fcc027e6401cbd9581f790ee3ed7a3f68e9c225fcb9f1cd7b7110d";

#[test]
fn transaction() {
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
            .finish_with_params(&protocol_parameters)
            .unwrap(),
    );
    let essence = TransactionEssence::Regular(
        RegularTransactionEssence::builder(protocol_parameters.network_id(), rand_inputs_commitment())
            .with_inputs(vec![input1, input2])
            .add_output(output)
            .add_allotment(Allotment::new(rand_account_id(), 10).unwrap())
            .finish_with_params(&protocol_parameters)
            .unwrap(),
    );

    let pub_key_bytes = prefix_hex::decode(ED25519_PUBLIC_KEY).unwrap();
    let sig_bytes = prefix_hex::decode(ED25519_SIGNATURE).unwrap();
    let signature = Ed25519Signature::try_from_bytes(pub_key_bytes, sig_bytes).unwrap();
    let sig_unlock = Unlock::from(SignatureUnlock::from(Signature::from(signature)));
    let ref_unlock = Unlock::from(ReferenceUnlock::new(0).unwrap());
    let unlocks = Unlocks::new(vec![sig_unlock, ref_unlock]).unwrap();

    let tx_payload = TransactionPayload::new(essence, unlocks).unwrap();

    let payload: Payload = tx_payload.into();
    let packed = payload.pack_to_vec();

    assert_eq!(payload.kind(), 6);
    assert_eq!(payload.packed_len(), packed.len());
    assert!(matches!(payload, Payload::Transaction(_)));
    assert_eq!(
        payload,
        PackableExt::unpack_verified(packed.as_slice(), &protocol_parameters).unwrap()
    );
}

#[test]
fn tagged_data() {
    let payload: Payload = TaggedDataPayload::new(rand_bytes(32), vec![]).unwrap().into();

    let packed = payload.pack_to_vec();

    assert_eq!(payload.kind(), 5);
    assert_eq!(payload.packed_len(), packed.len());
    assert!(matches!(payload, Payload::TaggedData(_)));
}
