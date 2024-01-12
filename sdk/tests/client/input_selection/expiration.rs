// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_sdk::{
    client::api::input_selection::{Error, InputSelection},
    types::block::{
        address::Address,
        output::{AccountId, NftId},
        protocol::protocol_parameters,
        slot::SlotIndex,
    },
};
use pretty_assertions::assert_eq;

use crate::client::{
    build_inputs, build_outputs, is_remainder_or_return, unsorted_eq,
    Build::{Account, Basic, Nft},
    ACCOUNT_ID_1, BECH32_ADDRESS_ACCOUNT_1, BECH32_ADDRESS_ED25519_0, BECH32_ADDRESS_ED25519_1,
    BECH32_ADDRESS_ED25519_2, NFT_ID_1, SLOT_INDEX,
};

#[test]
fn one_output_expiration_not_expired() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
            None,
            None,
            None,
            None,
            Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(), 200)),
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic(
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        protocol_parameters,
    )
    .select();

    assert!(matches!(selected, Err(Error::NoAvailableInputsProvided)));
}

#[test]
fn expiration_equal_timestamp() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
            None,
            None,
            None,
            None,
            Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(), 200)),
            None,
        )],
        Some(SlotIndex::from(200)),
    );
    let outputs = build_outputs([Basic(
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        200,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn one_output_expiration_expired() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
            None,
            None,
            None,
            None,
            Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(), 50)),
            None,
        )],
        Some(SlotIndex::from(100)),
    );
    let outputs = build_outputs([Basic(
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        100,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn two_outputs_one_expiration_expired() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                2_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
                None,
                None,
                None,
                None,
                Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(), 200)),
                None,
            ),
            Basic(
                2_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
                None,
                None,
                None,
                None,
                Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(), 50)),
                None,
            ),
        ],
        Some(SlotIndex::from(100)),
    );
    let outputs = build_outputs([Basic(
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        100,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[1]);
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn two_outputs_one_unexpired_one_missing() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                2_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
                None,
                None,
                None,
                None,
                Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(), 200)),
                None,
            ),
            Basic(
                2_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                None,
                None,
                None,
                None,
            ),
        ],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic(
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[1]);
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn two_outputs_two_expired() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                2_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
                None,
                None,
                None,
                None,
                Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(), 100)),
                None,
            ),
            Basic(
                2_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
                None,
                None,
                None,
                None,
                Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(), 100)),
                None,
            ),
            Basic(
                2_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                None,
                None,
                None,
                None,
            ),
        ],
        Some(SlotIndex::from(200)),
    );
    let outputs = build_outputs([Basic(
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap()],
        200,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[1]);
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn two_outputs_two_expired_2() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                2_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
                None,
                None,
                None,
                None,
                Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 100)),
                None,
            ),
            Basic(
                2_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
                None,
                None,
                None,
                None,
                Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(), 100)),
                None,
            ),
        ],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Basic(
        4_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap(),
        ],
        SLOT_INDEX,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn expiration_expired_with_sdr() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
            None,
            None,
            Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 1_000_000)),
            None,
            Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(), 50)),
            None,
        )],
        Some(SlotIndex::from(100)),
    );
    let outputs = build_outputs([Basic(
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        100,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn expiration_expired_with_sdr_2() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 1_000_000)),
            None,
            Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(), 50)),
            None,
        )],
        Some(SlotIndex::from(100)),
    );
    let outputs = build_outputs([Basic(
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        100,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn expiration_expired_with_sdr_and_timelock() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
            None,
            None,
            Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(), 1_000_000)),
            Some(50),
            Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(), 50)),
            None,
        )],
        Some(SlotIndex::from(100)),
    );
    let outputs = build_outputs([Basic(
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        100,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn expiration_expired_with_sdr_and_timelock_2() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
            None,
            None,
            Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 1_000_000)),
            Some(50),
            Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(), 50)),
            None,
        )],
        Some(SlotIndex::from(100)),
    );
    let outputs = build_outputs([Basic(
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        100,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn sender_in_expiration() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                None,
                None,
                Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 50)),
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                None,
                None,
                None,
                None,
            ),
            Basic(
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                None,
                None,
                None,
                None,
            ),
        ],
        Some(SlotIndex::from(100)),
    );
    let outputs = build_outputs([Basic(
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        Some(Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()),
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        ],
        100,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert!(selected.inputs.contains(&inputs[2]));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn sender_in_expiration_already_selected() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            1_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            None,
            None,
            None,
            None,
            Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 50)),
            None,
        )],
        Some(SlotIndex::from(100)),
    );
    let outputs = build_outputs([Basic(
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        Some(Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()),
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        ],
        100,
        protocol_parameters,
    )
    .with_required_inputs([*inputs[0].output_id()])
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn remainder_in_expiration() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap(),
            None,
            None,
            None,
            None,
            Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(), 50)),
            None,
        )],
        Some(SlotIndex::from(100)),
    );
    let outputs = build_outputs([Basic(
        1_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        Some(Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()),
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        ],
        100,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                1_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
                None
            ));
        }
    });
}

#[test]
fn expiration_expired_non_ed25519_in_address_unlock_condition() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(
        [Basic(
            2_000_000,
            Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap(),
            None,
            None,
            None,
            None,
            Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(), 50)),
            None,
        )],
        Some(SlotIndex::from(100)),
    );
    let outputs = build_outputs([Basic(
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        100,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn expiration_expired_only_account_addresses() {
    let protocol_parameters = protocol_parameters();
    let account_id_1 = AccountId::from_str(ACCOUNT_ID_1).unwrap();

    let inputs = build_inputs(
        [
            Basic(
                2_000_000,
                Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap(),
                None,
                None,
                None,
                None,
                Some((Address::try_from_bech32(BECH32_ADDRESS_ACCOUNT_1).unwrap(), 50)),
                None,
            ),
            Account(
                1_000_000,
                account_id_1,
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
                None,
                None,
                None,
            ),
        ],
        Some(SLOT_INDEX),
    );

    let outputs = build_outputs([Basic(
        2_000_000,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs,
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        SLOT_INDEX,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
}

#[test]
fn one_nft_output_expiration_unexpired() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(
        [Nft(
            2_000_000,
            nft_id_1,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
            None,
            None,
            None,
            Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(), 150)),
            None,
        )],
        Some(SLOT_INDEX),
    );
    let outputs = build_outputs([Nft(
        2_000_000,
        nft_id_1,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap()],
        SLOT_INDEX,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn one_nft_output_expiration_expired() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(
        [Nft(
            2_000_000,
            nft_id_1,
            Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
            None,
            None,
            None,
            Some((Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap(), 50)),
            None,
        )],
        Some(SlotIndex::from(100)),
    );
    let outputs = build_outputs([Nft(
        2_000_000,
        nft_id_1,
        Address::try_from_bech32(BECH32_ADDRESS_ED25519_1).unwrap(),
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        [Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap()],
        100,
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}
