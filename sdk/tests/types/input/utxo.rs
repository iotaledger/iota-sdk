// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use iota_sdk::types::block::{
    input::{Input, UtxoInput},
    output::OutputId,
};
use packable::PackableExt;

const OUTPUT_ID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c6492a00";

#[test]
fn kind() {
    assert_eq!(UtxoInput::KIND, 0);

    let input = Input::from(UtxoInput::from_str(OUTPUT_ID).unwrap());

    assert_eq!(input.kind(), UtxoInput::KIND);
}

#[test]
fn is_methods() {
    let input = Input::from(UtxoInput::from_str(OUTPUT_ID).unwrap());

    assert!(input.is_utxo());
}

#[test]
fn as_methods() {
    let utxo_input = UtxoInput::from_str(OUTPUT_ID).unwrap();
    let input = Input::from(utxo_input);

    assert_eq!(input.as_utxo(), &utxo_input);
}

#[test]
fn new_output_id() {
    let output_id = OutputId::from_str(OUTPUT_ID).unwrap();
    let input = UtxoInput::new(*output_id.transaction_id(), output_id.index()).unwrap();

    assert_eq!(*input.output_id(), output_id);
}

#[test]
fn from() {
    let output_id = OutputId::from_str(OUTPUT_ID).unwrap();
    let input = UtxoInput::from(output_id);

    assert_eq!(*input.output_id(), output_id);
}

#[test]
fn from_str_to_str() {
    assert_eq!(UtxoInput::from_str(OUTPUT_ID).unwrap().to_string(), OUTPUT_ID);
}

#[test]
fn debug() {
    assert_eq!(
        format!("{:?}", UtxoInput::from_str(OUTPUT_ID).unwrap()),
        "UtxoInput(0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c6492a00)"
    );
}

#[test]
fn from_str() {
    assert_eq!(
        *UtxoInput::from_str(OUTPUT_ID).unwrap().output_id(),
        OutputId::from_str(OUTPUT_ID).unwrap()
    );
}

#[test]
fn packed_len() {
    let output_id = OutputId::from_str(OUTPUT_ID).unwrap();

    assert_eq!(
        UtxoInput::new(*output_id.transaction_id(), output_id.index())
            .unwrap()
            .packed_len(),
        32 + 2
    );
    assert_eq!(output_id.pack_to_vec().len(), 32 + 2);
}

#[test]
fn pack_unpack() {
    let output_id = OutputId::from_str(OUTPUT_ID).unwrap();
    let utxo_input = UtxoInput::new(*output_id.transaction_id(), output_id.index()).unwrap();
    let packed_input = utxo_input.pack_to_vec();

    assert_eq!(
        utxo_input,
        UtxoInput::unpack_verified(packed_input.as_slice(), &()).unwrap()
    );

    let input = Input::from(utxo_input);
    let packed_input = input.pack_to_vec();

    assert_eq!(input, Input::unpack_verified(packed_input.as_slice(), &()).unwrap());
}
