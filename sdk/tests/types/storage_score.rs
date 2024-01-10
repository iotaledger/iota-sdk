// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    types::block::output::{Output, StorageScore, StorageScoreParameters},
    utils::serde::prefix_hex_bytes,
};
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
    #[serde(with = "prefix_hex_bytes")]
    bytes: Vec<u8>,
    storage_score: u64,
}

fn output_fixture(filename: &str) -> Result<OutputFixture, Box<dyn std::error::Error>> {
    Ok(serde_json::from_value(serde_json::from_reader(std::fs::File::open(
        format!("./tests/types/fixtures/{filename}"),
    )?)?)?)
}

// From https://github.com/iotaledger/tips/blob/tip41/tips/TIP-0041/tip-0041.md#storage-score
#[test]
fn output_storage_score() {
    for fixture in [
        output_fixture("tip41_basic_output.json").unwrap(),
        output_fixture("tip42_account_output.json").unwrap(),
        output_fixture("tip43_nft_output.json").unwrap(),
        output_fixture("tip44_foundry_output.json").unwrap(),
        output_fixture("tip54_anchor_output.json").unwrap(),
        output_fixture("tip40_delegation_output.json").unwrap(),
    ] {
        assert_eq!(
            fixture.output.storage_score(storage_score_parameters().unwrap()),
            fixture.storage_score
        );
    }
}
