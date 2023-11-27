// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::{
    api::core::{
        CommitteeResponse, CongestionResponse, InfoResponse, IssuanceBlockHeaderResponse, ManaRewardsResponse,
        RoutesResponse, UtxoChangesResponse, ValidatorResponse, ValidatorsResponse,
    },
    block::{output::OutputMetadata, slot::SlotCommitment, BlockDto, BlockId},
};
use packable::PackableExt;
use serde::Deserialize;

fn assert_json_response<T>(path: &str)
where
    for<'a> T: Deserialize<'a>,
{
    let file = std::fs::read_to_string(&format!("./tests/types/api/fixtures/{path}")).unwrap();
    serde_json::from_str::<T>(&file).unwrap();
}

fn assert_binary_response<T: PackableExt>(path: &str, visitor: &T::UnpackVisitor) {
    let file = std::fs::read_to_string(&format!("./tests/types/api/fixtures/{path}")).unwrap();
    let bytes = hex::decode(file).unwrap();
    T::unpack_verified(bytes, visitor).unwrap();
}

#[test]
fn responses() {
    assert_json_response::<BlockDto>("get-block-by-id-empty-response-example.json");
    assert_json_response::<BlockDto>("get-block-by-id-response-example-confirmed-transaction.json");
    assert_json_response::<BlockDto>("get-block-by-id-response-example-confirmed.json");
    assert_json_response::<BlockDto>("get-block-by-id-response-example-conflicting-transaction.json");
    assert_json_response::<BlockDto>("get-block-by-id-response-example-new-transaction.json");
    assert_json_response::<BlockDto>("get-block-by-id-response-example-new.json");
    assert_json_response::<BlockDto>("get-block-by-id-tagged-data-response-example.json");
    assert_json_response::<BlockDto>("get-block-by-id-transaction-response-example.json");
    assert_json_response::<BlockDto>("get-block-by-id-validation-response-example.json");
    assert_json_response::<IssuanceBlockHeaderResponse>("get-buildingBlock-response-example.json");
    assert_json_response::<SlotCommitment>("get-commitment-response-example.json");
    assert_json_response::<CommitteeResponse>("get-committee-example.json");
    assert_json_response::<CongestionResponse>("get-congestion-estimate-response-example.json");
    // assert_json_response("get-full-output-metadata-example.json");
    assert_json_response::<InfoResponse>("get-info-response-example.json");
    assert_json_response::<ManaRewardsResponse>("get-mana-rewards-example.json");
    assert_json_response::<OutputMetadata>("get-output-metadata-by-id-response-spent-example.json");
    assert_json_response::<OutputMetadata>("get-output-metadata-by-id-response-unspent-example.json");
    // assert_json_response("get-outputs-by-id-response-example.json");
    assert_json_response::<RoutesResponse>("get-routes-response-example.json");
    assert_json_response::<UtxoChangesResponse>("get-utxo-changes-response-example.json");
    assert_json_response::<ValidatorResponse>("get-validator-example.json");
    assert_json_response::<ValidatorsResponse>("get-validators-example.json");
    assert_json_response::<BlockId>("post-blocks-response-example.json");

    assert_binary_response::<SlotCommitment>("get-commitment-response-binary-example", &());
}
