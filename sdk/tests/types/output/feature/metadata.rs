// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::output::feature::{FeatureError, MetadataFeature};
use packable::{error::UnpackError, PackableExt};

#[test]
fn invalid() {
    // Invalid key
    assert!(
        serde_json::from_str::<MetadataFeature>(
            r#"{"type": 2, "entries": { "space is a non graphical ASCII value": "0x42" } }"#
        )
        .is_err()
    );

    // Invalid value
    assert!(serde_json::from_str::<MetadataFeature>(r#"{"type": 2, "entries": { "nothing": "" } }"#).is_err());
}

#[test]
fn serde_roundtrip() {
    // Single entry
    let metadata_feature: MetadataFeature =
        serde_json::from_str(r#"{"type": 2, "entries": { "some_key": "0x42" } }"#).unwrap();
    let metadata_feature_ser = serde_json::to_string(&metadata_feature).unwrap();

    assert_eq!(
        serde_json::from_str::<MetadataFeature>(&metadata_feature_ser).unwrap(),
        metadata_feature
    );

    // Multiple entries, order doesn't matter
    let metadata_feature: MetadataFeature =
        serde_json::from_str(r#"{"type": 2, "entries": { "b": "0x42", "a": "0x1337", "c": "0x" } }"#).unwrap();
    let metadata_feature_ser = serde_json::to_string(&metadata_feature).unwrap();

    assert_eq!(
        serde_json::from_str::<MetadataFeature>(&metadata_feature_ser).unwrap(),
        metadata_feature
    );
    // Unordered keys are not removed
    assert_eq!(metadata_feature.len(), 3);
}

#[test]
fn unpack_invalid_order() {
    assert!(matches!(
        MetadataFeature::unpack_bytes_verified([3, 1, 99, 0, 0, 1, 98, 0, 0, 1, 97, 0, 0], &()),
        Err(UnpackError::Packable(FeatureError::MetadataFeature(error_msg))) if &error_msg == "unordered map"
    ));
}

#[test]
fn unpack_invalid_length() {
    assert!(matches!(
        MetadataFeature::unpack_bytes_verified([vec![1, 1, 33, 0, 32], vec![0u8; 8192]].concat(), &()),
        Err(UnpackError::Packable(FeatureError::MetadataFeature(len))) if &len == "Out of bounds byte length: 8197"
    ));
}
