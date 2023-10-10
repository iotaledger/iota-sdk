// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use crypto::keys::bip44::Bip44;
use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::types::{InputSigningData, InputSigningDataDto},
    },
    types::{
        block::{
            address::Address,
            output::{unlock_condition::AddressUnlockCondition, BasicOutput, OutputId, OutputMetadata},
            payload::transaction::TransactionId,
            protocol::protocol_parameters,
            slot::SlotCommitmentId,
            BlockId,
        },
        TryFromDto,
    },
};

#[test]
fn input_signing_data_conversion() {
    let protocol_parameters = protocol_parameters();

    let bip44_chain = Bip44::new(SHIMMER_COIN_TYPE);

    let output = BasicOutput::build_with_amount(1_000_000)
        .add_unlock_condition(AddressUnlockCondition::new(
            Address::try_from_bech32("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy").unwrap(),
        ))
        .finish_output(protocol_parameters.token_supply())
        .unwrap();

    let input_signing_data = InputSigningData {
        output,
        output_metadata: OutputMetadata::new(
            BlockId::from_str("0xedf5f572c58ddf4b4f9567d82bf96689cc68b730df796d822b4b9fb643f5efda00000000").unwrap(),
            OutputId::from_str("0xbce525324af12eda02bf7927e92cea3a8e8322d0f41966271443e6c3b245a4400000").unwrap(),
            false,
            Some(
                SlotCommitmentId::from_str(
                    "0xedf5f572c58ddf4b4f9567d82bf96689cc68b730df796d822b4b9fb643f5efda4f9567d82bf96689",
                )
                .unwrap(),
            ),
            Some(
                TransactionId::from_str("0x24a1f46bdb6b2bf38f1c59f73cdd4ae5b418804bb231d76d06fbf246498d5883").unwrap(),
            ),
            Some(
                SlotCommitmentId::from_str(
                    "0xedf5f572c58ddf4b4f9567d82bf96689cc68b730df796d822b4b9fb643f5efda4f9567d82bf96689",
                )
                .unwrap(),
            ),
            SlotCommitmentId::from_str(
                "0xedf5f572c58ddf4b4f9567d82bf96689cc68b730df796d822b4b9fb643f5efda4f9567d82bf96689",
            )
            .unwrap(),
        ),
        chain: Some(bip44_chain),
    };

    let input_signing_data_dto = InputSigningDataDto::from(&input_signing_data);
    assert_eq!(input_signing_data_dto.chain.as_ref(), Some(&bip44_chain));

    let restored_input_signing_data =
        InputSigningData::try_from_dto_with_params(input_signing_data_dto.clone(), &protocol_parameters).unwrap();
    assert_eq!(input_signing_data, restored_input_signing_data);

    let input_signing_data_dto_str = r#"{"output":{"type":0,"amount":"1000000","mana":"0","unlockConditions":[{"type":0,"address":{"type":0,"pubKeyHash":"0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3"}}]},"outputMetadata":{"blockId":"0xedf5f572c58ddf4b4f9567d82bf96689cc68b730df796d822b4b9fb643f5efda0000000000000000","transactionId":"0xbce525324af12eda02bf7927e92cea3a8e8322d0f41966271443e6c3b245a440","outputIndex":0,"isSpent":false,"commitmentIdSpent":"0xedf5f572c58ddf4b4f9567d82bf96689cc68b730df796d822b4b9fb643f5efda4f9567d82bf96689","transactionIdSpent":"0x24a1f46bdb6b2bf38f1c59f73cdd4ae5b418804bb231d76d06fbf246498d5883","includedCommitmentId":"0xedf5f572c58ddf4b4f9567d82bf96689cc68b730df796d822b4b9fb643f5efda4f9567d82bf96689","latestCommitmentId":"0xedf5f572c58ddf4b4f9567d82bf96689cc68b730df796d822b4b9fb643f5efda4f9567d82bf96689"},"chain":{"coinType":4219,"account":0,"change":0,"addressIndex":0}}"#;
    assert_eq!(
        serde_json::to_string(&input_signing_data_dto).unwrap(),
        input_signing_data_dto_str
    );

    let restored_input_signing_data_dto =
        serde_json::from_str::<InputSigningDataDto>(input_signing_data_dto_str).unwrap();
    assert_eq!(restored_input_signing_data_dto.chain.as_ref(), Some(&bip44_chain));

    let restored_input_signing_data =
        InputSigningData::try_from_dto_with_params(restored_input_signing_data_dto, &protocol_parameters).unwrap();
    assert!(restored_input_signing_data.output.is_basic());
    assert_eq!(restored_input_signing_data.chain, Some(bip44_chain));
}
