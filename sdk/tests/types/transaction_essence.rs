// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::{
    address::{Address, Ed25519Address},
    input::{Input, UtxoInput},
    output::{unlock_condition::AddressUnlockCondition, BasicOutput, Output},
    payload::transaction::{RegularTransactionEssence, TransactionEssence, TransactionId},
    protocol::protocol_parameters,
    rand::mana::rand_mana_allotment,
    Error,
};
use packable::{error::UnpackError, PackableExt};
use pretty_assertions::assert_eq;

const TRANSACTION_ID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649";
const ED25519_ADDRESS: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649";

#[test]
fn essence_kind() {
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
            .finish_with_params(protocol_parameters.token_supply())
            .unwrap(),
    );
    let essence = TransactionEssence::Regular(
        RegularTransactionEssence::builder(protocol_parameters.network_id())
            .with_inputs([input1, input2])
            .add_output(output)
            .add_mana_allotment(rand_mana_allotment(&protocol_parameters))
            .finish_with_params(&protocol_parameters)
            .unwrap(),
    );

    assert_eq!(essence.kind(), RegularTransactionEssence::KIND);
}

#[test]
fn essence_unpack_invalid_kind() {
    assert!(matches!(
        TransactionEssence::unpack_verified([3u8; 32], &protocol_parameters()),
        Err(UnpackError::Packable(Error::InvalidEssenceKind(3)))
    ));
}
