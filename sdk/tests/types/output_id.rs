// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use iota_sdk::types::block::{
    output::OutputId,
    payload::signed_transaction::{TransactionHash, TransactionId},
};
use packable::PackableExt;
use pretty_assertions::assert_eq;

const TRANSACTION_ID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64900000000";
const OUTPUT_ID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649000000002a00";

#[test]
fn debug_impl() {
    assert_eq!(
        format!("{:?}", OutputId::from_str(OUTPUT_ID).unwrap()),
        "OutputId { \
            id: \"0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649000000002a00\", \
            transaction_id: TransactionId { \
                id: \"0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64900000000\", \
                slot_index: SlotIndex(0) \
            }, \
            output_index: 42 \
        }"
    );
}

#[test]
fn new() {
    let transaction_id = TransactionId::from_str(TRANSACTION_ID).unwrap();
    let output_id = OutputId::new(transaction_id, 42);

    assert_eq!(*output_id.transaction_id(), transaction_id);
    assert_eq!(output_id.index(), 42);
}

#[test]
fn null() {
    assert_eq!(
        format!("{}", TransactionHash::null().into_transaction_id(0).into_output_id(0)),
        "0x0000000000000000000000000000000000000000000000000000000000000000000000000000"
    );
}

#[test]
fn split_valid() {
    let transaction_id = TransactionId::from_str(TRANSACTION_ID).unwrap();
    let output_id = OutputId::new(transaction_id, 42);
    let (transaction_id_s, index) = output_id.split();

    assert_eq!(transaction_id_s, transaction_id);
    assert_eq!(index, 42);
}

#[test]
fn from_bytes() {
    let transaction_id = TransactionId::from_str(TRANSACTION_ID).unwrap();
    let output_id_bytes: [u8; OutputId::LENGTH] = prefix_hex::decode(OUTPUT_ID).unwrap();
    let output_id = OutputId::from(output_id_bytes);

    assert_eq!(*output_id.transaction_id(), transaction_id);
    assert_eq!(output_id.index(), 42);
}

#[test]
fn from_str() {
    let transaction_id = TransactionId::from_str(TRANSACTION_ID).unwrap();
    let output_id = OutputId::from_str(OUTPUT_ID).unwrap();

    assert_eq!(*output_id.transaction_id(), transaction_id);
    assert_eq!(output_id.index(), 42);
}

#[test]
fn from_str_to_str() {
    let output_id = OutputId::from_str(OUTPUT_ID).unwrap();

    assert_eq!(output_id.to_string(), OUTPUT_ID);
}

#[test]
fn packed_len() {
    let output_id = OutputId::from_str(OUTPUT_ID).unwrap();

    assert_eq!(
        output_id.packed_len(),
        TransactionId::LENGTH + core::mem::size_of::<u16>()
    );
    assert_eq!(
        output_id.pack_to_vec().len(),
        TransactionId::LENGTH + core::mem::size_of::<u16>()
    );
}

#[test]
fn pack_unpack() {
    let output_id_1 = OutputId::from_str(OUTPUT_ID).unwrap();
    let output_id_2 = OutputId::unpack_bytes_verified(output_id_1.pack_to_vec().as_slice(), &()).unwrap();

    assert_eq!(output_id_1, output_id_2);
}
