// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::{
    api::core::{
        CommitteeResponse, CongestionResponse, InfoResponse, IssuanceBlockHeaderResponse, ManaRewardsResponse,
        RoutesResponse, UtxoChangesResponse, ValidatorResponse, ValidatorsResponse,
    },
    block::{output::OutputMetadata, slot::SlotCommitment, BlockDto, BlockId},
};
use packable::{
    error::{UnexpectedEOF, UnpackError},
    Packable, PackableExt,
};
use serde::Deserialize;

fn json_response<T>(path: &str) -> Result<T, serde_json::Error>
where
    for<'a> T: Deserialize<'a>,
{
    let file = std::fs::read_to_string(&format!("./tests/types/api/fixtures/{path}")).unwrap();
    serde_json::from_str::<T>(&file)
}

fn binary_response<T: PackableExt>(
    path: &str,
    visitor: &T::UnpackVisitor,
) -> Result<T, UnpackError<<T as Packable>::UnpackError, UnexpectedEOF>> {
    let file = std::fs::read_to_string(&format!("./tests/types/api/fixtures/{path}")).unwrap();
    let bytes = hex::decode(file).unwrap();
    T::unpack_verified(bytes, visitor)
}

#[test]
fn responses() {
    json_response::<BlockDto>("get-block-by-id-empty-response-example.json").unwrap();
    json_response::<BlockDto>("get-block-by-id-response-example-confirmed-transaction.json").unwrap();
    json_response::<BlockDto>("get-block-by-id-response-example-confirmed.json").unwrap();
    json_response::<BlockDto>("get-block-by-id-response-example-conflicting-transaction.json").unwrap();
    json_response::<BlockDto>("get-block-by-id-response-example-new-transaction.json").unwrap();
    json_response::<BlockDto>("get-block-by-id-response-example-new.json").unwrap();
    json_response::<BlockDto>("get-block-by-id-tagged-data-response-example.json").unwrap();
    json_response::<BlockDto>("get-block-by-id-transaction-response-example.json").unwrap();
    json_response::<BlockDto>("get-block-by-id-validation-response-example.json").unwrap();
    json_response::<IssuanceBlockHeaderResponse>("get-buildingBlock-response-example.json").unwrap();
    json_response::<SlotCommitment>("get-commitment-response-example.json").unwrap();
    json_response::<CommitteeResponse>("get-committee-example.json").unwrap();
    json_response::<CongestionResponse>("get-congestion-estimate-response-example.json").unwrap();
    // json_response("get-full-output-metadata-example.json").unwrap();
    json_response::<InfoResponse>("get-info-response-example.json").unwrap();
    json_response::<ManaRewardsResponse>("get-mana-rewards-example.json").unwrap();
    json_response::<OutputMetadata>("get-output-metadata-by-id-response-spent-example.json").unwrap();
    json_response::<OutputMetadata>("get-output-metadata-by-id-response-unspent-example.json").unwrap();
    // json_response("get-outputs-by-id-response-example.json").unwrap();
    json_response::<RoutesResponse>("get-routes-response-example.json").unwrap();
    json_response::<UtxoChangesResponse>("get-utxo-changes-response-example.json").unwrap();
    json_response::<ValidatorResponse>("get-validator-example.json").unwrap();
    json_response::<ValidatorsResponse>("get-validators-example.json").unwrap();
    json_response::<BlockId>("post-blocks-response-example.json").unwrap();

    binary_response::<SlotCommitment>("get-commitment-response-binary-example", &()).unwrap();
}
