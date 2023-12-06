// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip44::Bip44;
use iota_sdk::{
    client::{constants::SHIMMER_COIN_TYPE, secret::types::InputSigningData},
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
        signing_options: Some(bip44_chain),
    };

    assert_eq!(input_signing_data.signing_options.as_ref(), Some(&bip44_chain));

    let input_signing_data_json = serde_json::to_value(&input_signing_data).unwrap();

    let restored_input_signing_data =
        serde_json::from_value::<InputSigningData<Bip44>>(input_signing_data_json).unwrap();
    assert!(restored_input_signing_data.output.is_basic());
    assert_eq!(restored_input_signing_data.signing_options.as_ref(), Some(&bip44_chain));
}
