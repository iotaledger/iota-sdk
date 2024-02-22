// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    types::block::{output::OutputIdProof, payload::Payload},
    utils::serde::prefix_hex_bytes,
};
use packable::PackableExt;
use pretty_assertions::assert_eq;
use serde::Deserialize;

#[derive(Deserialize)]
struct ProofFixture {
    #[serde(with = "prefix_hex_bytes")]
    transaction_bytes: Vec<u8>,
    proof: OutputIdProof,
    proof_bytes: String,
}

fn proof_fixture(filename: &str) -> Result<ProofFixture, Box<dyn std::error::Error>> {
    Ok(serde_json::from_value(serde_json::from_reader(std::fs::File::open(
        format!("./tests/types/fixtures/{filename}"),
    )?)?)?)
}

#[test]
fn output_proofs() {
    for filename in [
        // https://github.com/iotaledger/tips/blob/tip45/tips/TIP-0045/tip-0045.md#single-output
        "tip45_single_output_proof.json",
        // https://github.com/iotaledger/tips/blob/tip45/tips/TIP-0045/tip-0045.md#five-outputs
        "tip45_five_output_proof.json",
        // https://github.com/iotaledger/tips/blob/tip45/tips/TIP-0045/tip-0045.md#32-outputs
        "tip45_32_output_proof_idx0.json",
        "tip45_32_output_proof_idx28.json",
    ] {
        let fixture = proof_fixture(filename).unwrap_or_else(|e| panic!("failed to deserialize {filename}: {e}"));

        let payload = Payload::unpack_bytes_unverified(&fixture.transaction_bytes).unwrap();
        let transaction = payload.as_signed_transaction().transaction();
        assert_eq!(
            transaction
                .output_id_proof(fixture.proof.output_index)
                .map(|p| serde_json::to_string_pretty(&p).unwrap())
                .unwrap(),
            serde_json::to_string_pretty(&fixture.proof).unwrap(),
            "proof mismatch for {filename}"
        );
        assert_eq!(
            prefix_hex::encode(fixture.proof.pack_to_vec()),
            fixture.proof_bytes,
            "byte mismatch for {filename}"
        );
    }
}
