// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip44::Bip44;
use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::types::{InputSigningData, InputSigningDataDto},
    },
    types::block::{
        address::Address,
        output::{unlock_condition::AddressUnlockCondition, BasicOutput},
        rand::output::rand_output_metadata,
    },
};
use pretty_assertions::assert_eq;

#[test]
fn input_signing_data_conversion() {
    let bip44_chain = Bip44::new(SHIMMER_COIN_TYPE);

    let output = BasicOutput::build_with_amount(1_000_000)
        .add_unlock_condition(AddressUnlockCondition::new(
            Address::try_from_bech32("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy").unwrap(),
        ))
        .finish_output()
        .unwrap();

    let input_signing_data = InputSigningData {
        output,
        output_metadata: rand_output_metadata(),
        chain: Some(bip44_chain),
    };

    let input_signing_data_dto = InputSigningDataDto::from(&input_signing_data);
    assert_eq!(input_signing_data_dto.chain.as_ref(), Some(&bip44_chain));

    let restored_input_signing_data = InputSigningData::try_from(input_signing_data_dto.clone()).unwrap();
    assert_eq!(input_signing_data, restored_input_signing_data);

    let input_signing_data_dto_json = serde_json::to_string(&input_signing_data_dto).unwrap();

    let restored_input_signing_data_dto =
        serde_json::from_str::<InputSigningDataDto>(&input_signing_data_dto_json).unwrap();
    assert_eq!(restored_input_signing_data_dto.chain.as_ref(), Some(&bip44_chain));

    let restored_input_signing_data = InputSigningData::try_from(restored_input_signing_data_dto).unwrap();
    assert!(restored_input_signing_data.output.is_basic());
    assert_eq!(restored_input_signing_data.chain, Some(bip44_chain));
}
