// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::output::{Output, StorageScore, StorageScoreParameters};
use packable::PackableExt;
use pretty_assertions::assert_eq;
use serde::Deserialize;

// From https://github.com/iotaledger/tips/blob/tip49/tips/TIP-0049/tip-0049.md#test-vectors
fn storage_score_parameters() -> Result<StorageScoreParameters, Box<dyn std::error::Error>> {
    let json: serde_json::Value =
        serde_json::from_reader(std::fs::File::open("./tests/types/fixtures/protocol_parameters.json")?)?;

    Ok(StorageScoreParameters::deserialize(
        &json["params"]["storageScoreParameters"],
    )?)
}

#[derive(Deserialize)]
struct OutputFixture {
    output: Output,
    bytes: String,
    storage_score: u64,
}

fn output_fixture(filename: &str) -> Result<OutputFixture, Box<dyn std::error::Error>> {
    Ok(serde_json::from_value(serde_json::from_reader(std::fs::File::open(
        format!("./tests/types/fixtures/{filename}"),
    )?)?)?)
}

#[test]
fn output_storage_score() {
    let storage_score_parameters = storage_score_parameters().unwrap();

    for filename in [
        // https://github.com/iotaledger/tips/blob/tip40/tips/TIP-0040/tip-0040.md#storage-score-1
        "tip40_delegation_output.json",
        // https://github.com/iotaledger/tips/blob/tip41/tips/TIP-0041/tip-0041.md#storage-score
        "tip41_basic_output.json",
        // https://github.com/iotaledger/tips/blob/tip42/tips/TIP-0042/tip-0042.md#storage-score-3
        "tip42_account_output.json",
        // https://github.com/iotaledger/tips/blob/tip43/tips/TIP-0043/tip-0043.md#storage-score
        "tip43_nft_output.json",
        // https://github.com/iotaledger/tips/blob/tip44/tips/TIP-0044/tip-0044.md#storage-score
        "tip44_foundry_output.json",
        // https://github.com/iotaledger/tips/blob/tip54/tips/TIP-0054/tip-0054.md#storage-score
        "tip54_anchor_output.json",
    ] {
        let fixture = output_fixture(filename).unwrap_or_else(|e| panic!("failed to deserialize {filename}: {e}"));

        assert_eq!(
            fixture.output.storage_score(storage_score_parameters),
            fixture.storage_score,
            "storage score mismatch for {filename}"
        );
        assert_eq!(
            prefix_hex::encode(fixture.output.pack_to_vec()),
            fixture.bytes,
            "byte mismatch for {filename}"
        );
    }
}
