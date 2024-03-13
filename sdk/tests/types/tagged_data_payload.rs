// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::{
    payload::{tagged_data::TaggedDataPayload, PayloadError},
    rand::bytes::{rand_bytes, rand_bytes_array},
};
use packable::{
    bounded::{TryIntoBoundedU32Error, TryIntoBoundedU8Error},
    error::UnpackError,
    PackableExt,
};
use pretty_assertions::assert_eq;

#[test]
fn kind() {
    assert_eq!(TaggedDataPayload::KIND, 0);
}

#[test]
fn new_valid() {
    let tag = rand_bytes_array::<64>();
    let data = [0x42, 0xff, 0x84, 0xa2, 0x42, 0xff, 0x84, 0xa2];
    let tagged_data = TaggedDataPayload::new(tag, data).unwrap();

    assert_eq!(tagged_data.tag(), &tag);
    assert_eq!(tagged_data.data(), &data);
}

#[test]
fn new_valid_empty_data() {
    let tag = rand_bytes_array::<64>();
    let data = [];
    let tagged_data = TaggedDataPayload::new(tag, data).unwrap();

    assert_eq!(tagged_data.tag(), &tag);
    assert_eq!(tagged_data.data(), &data);
}

#[test]
fn new_valid_padded() {
    let tag = rand_bytes_array::<32>();
    let data = [];
    let tagged_data = TaggedDataPayload::new(tag, data).unwrap();

    assert_eq!(tagged_data.tag(), &tag);
    assert_eq!(tagged_data.data(), &data);
}

#[test]
fn new_valid_tag_length_min() {
    let payload = TaggedDataPayload::new(Vec::new(), [0x42, 0xff, 0x84, 0xa2, 0x42, 0xff, 0x84, 0xa2]).unwrap();

    assert!(payload.tag().is_empty());
}

#[test]
fn new_invalid_tag_length_more_than_max() {
    assert_eq!(
        TaggedDataPayload::new(
            [0u8; *TaggedDataPayload::TAG_LENGTH_RANGE.end() as usize + 1],
            [0x42, 0xff, 0x84, 0xa2, 0x42, 0xff, 0x84, 0xa2],
        ),
        Err(PayloadError::TagLength(TryIntoBoundedU8Error::Invalid(
            TaggedDataPayload::TAG_LENGTH_RANGE.end() + 1
        )))
    );
}

#[test]
fn new_invalid_data_length_more_than_max() {
    assert_eq!(
        TaggedDataPayload::new(
            rand_bytes(32),
            [0u8; *TaggedDataPayload::DATA_LENGTH_RANGE.end() as usize + 1]
        ),
        Err(PayloadError::TaggedDataLength(TryIntoBoundedU32Error::Invalid(
            TaggedDataPayload::DATA_LENGTH_RANGE.end() + 1
        )))
    );
}

#[test]
fn packed_len() {
    let tagged_data = TaggedDataPayload::new(rand_bytes(10), [0x42, 0xff, 0x84, 0xa2, 0x42, 0xff, 0x84, 0xa2]).unwrap();

    assert_eq!(tagged_data.packed_len(), 1 + 10 + 4 + 8);
    assert_eq!(tagged_data.pack_to_vec().len(), 1 + 10 + 4 + 8);
}

#[test]
fn pack_unpack_valid() {
    let tagged_data_1 =
        TaggedDataPayload::new(rand_bytes(32), [0x42, 0xff, 0x84, 0xa2, 0x42, 0xff, 0x84, 0xa2]).unwrap();
    let tagged_data_2 = TaggedDataPayload::unpack_bytes_verified(tagged_data_1.pack_to_vec().as_slice(), &()).unwrap();

    assert_eq!(tagged_data_1.tag(), tagged_data_2.tag());
    assert_eq!(tagged_data_1.data(), tagged_data_2.data());
}

#[test]
fn unpack_valid_tag_length_min() {
    let payload =
        TaggedDataPayload::unpack_bytes_verified([0x00, 0x00, 0x00, 0x00, 0x00, 0x00].as_slice(), &()).unwrap();

    assert!(payload.tag().is_empty());
}

#[test]
fn unpack_invalid_tag_length_more_than_max() {
    assert!(matches!(
        TaggedDataPayload::unpack_bytes_verified(
            [
                0x41, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00
            ],
            &()
        ),
        Err(UnpackError::Packable(PayloadError::TagLength(
            TryIntoBoundedU8Error::Invalid(65)
        )))
    ));
}

#[test]
fn unpack_invalid_data_length_more_than_max() {
    assert!(matches!(
        TaggedDataPayload::unpack_bytes_verified([0x02, 0x00, 0x00, 0x35, 0x82, 0x00, 0x00], &()),
        Err(UnpackError::Packable(PayloadError::TaggedDataLength(
            TryIntoBoundedU32Error::Invalid(33333)
        )))
    ));
}
