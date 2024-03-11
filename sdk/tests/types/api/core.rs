// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::{
    api::core::{
        BlockMetadataResponse, BlockWithMetadataResponse, CommitteeResponse, CongestionResponse, InfoResponse,
        IssuanceBlockHeaderResponse, ManaRewardsResponse, OutputResponse, OutputWithMetadataResponse, RoutesResponse,
        SubmitBlockResponse, TransactionMetadataResponse, UtxoChangesFullResponse, UtxoChangesResponse,
        ValidatorResponse, ValidatorsResponse,
    },
    block::{output::OutputMetadata, slot::SlotCommitment, BlockDto},
};
use packable::{
    error::{UnexpectedEOF, UnpackError},
    unpacker::SliceUnpacker,
    Packable, PackableExt,
};
// use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};

fn json_response<T>(path: &str) -> Result<T, serde_json::Error>
where
    for<'a> T: Serialize + Deserialize<'a>,
{
    let file = std::fs::read_to_string(format!("./tests/types/api/fixtures/{path}")).unwrap();
    let value_des = serde_json::from_str::<serde_json::Value>(&file)?;
    let t = serde_json::from_value::<T>(value_des.clone())?;
    // let value_ser = serde_json::to_value(&t)?;

    // TODO https://github.com/iotaledger/iota-sdk/issues/1883
    // assert_eq!(value_des, value_ser);

    Ok(t)
}

fn binary_response<T: PackableExt>(
    path: &str,
    visitor: &T::UnpackVisitor,
) -> Result<T, UnpackError<<T as Packable>::UnpackError, UnexpectedEOF>> {
    let file = std::fs::read_to_string(format!("./tests/types/api/fixtures/{path}")).unwrap();
    let bytes = hex::decode(file).unwrap();
    let mut unpacker = SliceUnpacker::new(bytes.as_slice());
    let res = T::unpack_verified(&mut unpacker, visitor);

    assert!(u8::unpack_verified(&mut unpacker, &()).is_err());

    res
}

#[test]
fn responses() {
    // GET /api/routes
    json_response::<RoutesResponse>("get-routes-response-example.json").unwrap();
    // GET /api/core/v3/info
    // TODO reenable when Metrics are split out of Info
    // json_response::<InfoResponse>("get-info-response-example.json").unwrap();
    // GET /api/core/v3/accounts/{bech32Address}/congestion
    json_response::<CongestionResponse>("get-congestion-estimate-response-example.json").unwrap();
    // GET /api/core/v3/rewards/{outputId}
    json_response::<ManaRewardsResponse>("get-mana-rewards-example.json").unwrap();
    // GET /api/core/v3/validators
    json_response::<ValidatorsResponse>("get-validators-example.json").unwrap();
    // GET /api/core/v3/validators/{bech32Address}
    json_response::<ValidatorResponse>("get-validator-example.json").unwrap();
    // GET /api/core/v3/committee
    json_response::<CommitteeResponse>("get-committee-example.json").unwrap();
    // GET /api/core/v3/blocks/issuance
    json_response::<IssuanceBlockHeaderResponse>("get-buildingBlock-response-example.json").unwrap();
    // POST /api/core/v3/blocks
    json_response::<SubmitBlockResponse>("post-blocks-response-example.json").unwrap();
    // GET /api/core/v3/blocks/{blockId}
    json_response::<BlockDto>("get-block-by-id-empty-response-example.json").unwrap();
    json_response::<BlockDto>("tagged-data-block-example.json").unwrap();
    json_response::<BlockDto>("transaction-block-example.json").unwrap();
    json_response::<BlockDto>("get-block-by-id-validation-response-example.json").unwrap();
    // GET /api/core/v3/blocks/{blockId}/metadata
    // TODO reenable when TIP is updated
    // json_response::<BlockMetadataResponse>("get-block-by-id-response-example-new-transaction.json").unwrap();
    // json_response::<BlockMetadataResponse>("get-block-by-id-response-example-new.json").unwrap();
    // json_response::<BlockMetadataResponse>("get-block-by-id-response-example-confirmed-transaction.json").unwrap();
    // json_response::<BlockMetadataResponse>("get-block-by-id-response-example-confirmed.json").unwrap();
    // json_response::<BlockMetadataResponse>("get-block-by-id-response-example-conflicting-transaction.json").unwrap();
    // GET /api/core/v3/blocks/{blockId}/full
    json_response::<BlockWithMetadataResponse>("get-full-block-by-id-tagged-data-response-example.json").unwrap();
    // GET /api/core/v3/outputs/{outputId}
    json_response::<OutputResponse>("get-outputs-by-id-response-example.json").unwrap();
    // GET /api/core/v3/outputs/{outputId}/metadata
    json_response::<OutputMetadata>("get-output-metadata-by-id-response-unspent-example.json").unwrap();
    json_response::<OutputMetadata>("get-output-metadata-by-id-response-spent-example.json").unwrap();
    // GET /api/core/v3/outputs/{outputId}/full
    json_response::<OutputWithMetadataResponse>("get-full-output-metadata-example.json").unwrap();
    // GET /api/core/v3/transactions/{transactionId}/metadata
    // TODO reenable when TIP is updated
    // json_response::<TransactionMetadataResponse>("get-transaction-metadata-by-id-response-example.json").unwrap();
    // GET /api/core/v3/commitments/{commitmentId}
    json_response::<SlotCommitment>("get-commitment-response-example.json").unwrap();
    binary_response::<SlotCommitment>("get-commitment-response-binary-example", &()).unwrap();
    // GET /api/core/v3/commitments/{commitmentId}/utxo-changes
    json_response::<UtxoChangesResponse>("get-utxo-changes-response-example.json").unwrap();
    // GET /api/core/v3/commitments/{commitmentId}/utxo-changes/full
    json_response::<UtxoChangesFullResponse>("get-utxo-changes-full-response-example.json").unwrap();
}
