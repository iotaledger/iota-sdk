// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::{
    payload::Payload,
    protocol::protocol_parameters,
    rand::{parents::rand_strong_parents, payload::rand_tagged_data_payload},
    Block, BlockBuilder,
};
use packable::PackableExt;

// TODO reenable tests
// #[test]
// fn invalid_length() {
//     let res = BlockBuilder::new(Parents::from_vec(rand_block_ids(2)).unwrap())
//         .with_payload(TaggedDataPayload::new(vec![42], vec![0u8; Block::LENGTH_MAX - Block::LENGTH_MIN -
// 9]).unwrap())         .finish();

//     assert!(matches!(res, Err(Error::InvalidBlockLength(len)) if len == Block::LENGTH_MAX + 33));
// }

// #[test]
// fn unpack_valid_no_remaining_bytes() {
//     assert!(
//         Block::unpack_strict(
//             vec![
//                 2, 2, 140, 28, 186, 52, 147, 145, 96, 9, 105, 89, 78, 139, 3, 71, 249, 97, 149, 190, 63, 238, 168,
// 202,                 82, 140, 227, 66, 173, 19, 110, 93, 117, 34, 225, 202, 251, 10, 156, 58, 144, 225, 54, 79, 62,
// 38, 20,                 121, 95, 90, 112, 109, 6, 166, 126, 145, 13, 62, 52, 68, 248, 135, 223, 119, 137, 13, 0, 0,
// 0, 0, 21,                 205, 91, 7, 0, 0, 0, 0,
//             ]
//             .as_slice(),
//             &protocol_parameters()
//         )
//         .is_ok()
//     )
// }

// #[test]
// fn unpack_invalid_remaining_bytes() {
//     assert!(matches!(
//         Block::unpack_strict(
//             vec![
//                 2, 2, 140, 28, 186, 52, 147, 145, 96, 9, 105, 89, 78, 139, 3, 71, 249, 97, 149, 190, 63, 238, 168,
// 202,                 82, 140, 227, 66, 173, 19, 110, 93, 117, 34, 225, 202, 251, 10, 156, 58, 144, 225, 54, 79, 62,
// 38, 20,                 121, 95, 90, 112, 109, 6, 166, 126, 145, 13, 62, 52, 68, 248, 135, 223, 119, 137, 13, 0, 0,
// 0, 0, 21,                 205, 91, 7, 0, 0, 0, 0, 42
//             ]
//             .as_slice(),
//             &protocol_parameters()
//         ),
//         Err(UnpackError::Packable(Error::RemainingBytesAfterBlock))
//     ))
// }

// Validate that a `unpack` ∘ `pack` round-trip results in the original block.
#[test]
fn pack_unpack_valid() {
    let protocol_parameters = protocol_parameters();
    let block = BlockBuilder::new(rand_strong_parents()).finish().unwrap();
    let packed_block = block.pack_to_vec();

    assert_eq!(packed_block.len(), block.packed_len());
    assert_eq!(
        block,
        PackableExt::unpack_verified(packed_block.as_slice(), &protocol_parameters).unwrap()
    );
}

#[test]
fn getters() {
    let protocol_parameters = protocol_parameters();
    let parents = rand_strong_parents();
    let payload = Payload::from(rand_tagged_data_payload());

    let block = BlockBuilder::new(parents.clone())
        .with_payload(payload.clone())
        .finish()
        .unwrap();

    assert_eq!(block.protocol_version(), protocol_parameters.protocol_version());
    assert_eq!(*block.strong_parents(), parents);
    assert_eq!(*block.payload().as_ref().unwrap(), &payload);
}

#[test]
fn build_into_parents() {
    let parents = rand_strong_parents();
    let block = Block::build(parents.clone()).finish().unwrap();

    assert_eq!(block.into_strong_parents(), parents);
}
