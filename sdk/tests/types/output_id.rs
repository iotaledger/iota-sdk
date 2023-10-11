// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use iota_sdk::types::block::{
    output::OutputId,
    payload::transaction::{TransactionHash, TransactionId},
    Error,
};
use packable::{bounded::InvalidBoundedU16, error::UnpackError, PackableExt};

const TRANSACTION_ID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64900000000";
const OUTPUT_ID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649000000002a00";
const OUTPUT_ID_INVALID_INDEX: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649000000008000";

#[test]
fn debug_impl() {
    assert_eq!(
        format!("{:?}", OutputId::from_str(OUTPUT_ID).unwrap()),
        "OutputId(0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649000000002a00)"
    );
}

#[test]
fn new_valid() {
    let transaction_id = TransactionId::from_str(TRANSACTION_ID).unwrap();
    let output_id = OutputId::new(transaction_id, 42).unwrap();

    assert_eq!(*output_id.transaction_id(), transaction_id);
    assert_eq!(output_id.index(), 42);
}

#[test]
fn null() {
    assert_eq!(
        format!(
            "{:?}",
            TransactionHash::null().with_slot_index(0).with_output_index(0).unwrap()
        ),
        "OutputId(0x0000000000000000000000000000000000000000000000000000000000000000000000000000)"
    );
}

#[test]
fn split_valid() {
    let transaction_id = TransactionId::from_str(TRANSACTION_ID).unwrap();
    let output_id = OutputId::new(transaction_id, 42).unwrap();
    let (transaction_id_s, index) = output_id.split();

    assert_eq!(transaction_id_s, transaction_id);
    assert_eq!(index, 42);
}

#[test]
fn new_invalid() {
    let transaction_id = TransactionId::from_str(TRANSACTION_ID).unwrap();

    assert!(matches!(
        OutputId::new(transaction_id, 128),
        Err(Error::InvalidInputOutputIndex(InvalidBoundedU16(128)))
    ));
}

#[test]
fn try_from_valid() {
    let transaction_id = TransactionId::from_str(TRANSACTION_ID).unwrap();
    let output_id_bytes: [u8; OutputId::LENGTH] = prefix_hex::decode(OUTPUT_ID).unwrap();
    let output_id = OutputId::try_from(output_id_bytes).unwrap();

    assert_eq!(*output_id.transaction_id(), transaction_id);
    assert_eq!(output_id.index(), 42);
}

#[test]
fn try_from_invalid() {
    let output_id_bytes: [u8; OutputId::LENGTH] = prefix_hex::decode(OUTPUT_ID_INVALID_INDEX).unwrap();

    assert!(matches!(
        OutputId::try_from(output_id_bytes),
        Err(Error::InvalidInputOutputIndex(InvalidBoundedU16(128)))
    ));
}

#[test]
fn from_str_valid() {
    let transaction_id = TransactionId::from_str(TRANSACTION_ID).unwrap();
    let output_id = OutputId::from_str(OUTPUT_ID).unwrap();

    assert_eq!(*output_id.transaction_id(), transaction_id);
    assert_eq!(output_id.index(), 42);
}

#[test]
fn from_str_invalid_index() {
    assert!(matches!(
        OutputId::from_str(OUTPUT_ID_INVALID_INDEX),
        Err(Error::InvalidInputOutputIndex(InvalidBoundedU16(128)))
    ));
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
fn pack_unpack_valid() {
    let output_id_1 = OutputId::from_str(OUTPUT_ID).unwrap();
    let output_id_2 = OutputId::unpack_verified(output_id_1.pack_to_vec().as_slice(), &()).unwrap();

    assert_eq!(output_id_1, output_id_2);
}

#[test]
fn pack_unpack_invalid() {
    let bytes = vec![
        82, 253, 252, 7, 33, 130, 101, 79, 22, 63, 95, 15, 154, 98, 29, 114, 149, 102, 199, 77, 16, 3, 124, 77, 123,
        187, 4, 7, 209, 226, 198, 73, 0, 0, 0, 0, 128, 0,
    ];

    assert!(matches!(
        OutputId::unpack_verified(bytes.as_slice(), &()),
        Err(UnpackError::Packable(Error::InvalidInputOutputIndex(
            InvalidBoundedU16(128)
        )))
    ));
}
