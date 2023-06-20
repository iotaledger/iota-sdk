// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derivative::Derivative;
use iota_sdk::types::block::{
    address::{Bech32Address, Hrp},
    output::{dto::OutputDto, AliasId, NftId, OutputId, RentStructure, TokenScheme},
    payload::{
        dto::MilestonePayloadDto,
        transaction::{
            dto::{TransactionEssenceDto, TransactionPayloadDto},
            TransactionId,
        },
    },
    signature::dto::Ed25519SignatureDto,
    BlockDto,
};
use serde::{Deserialize, Serialize};

use crate::OmittedDebug;

/// Each public utils method.
#[derive(Clone, Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
#[serde(tag = "name", content = "data", rename_all = "camelCase")]
pub enum UtilsMethod {
    /// Transforms bech32 to hex
    Bech32ToHex {
        /// Bech32 encoded address
        bech32: Bech32Address,
    },
    /// Transforms a hex encoded address to a bech32 encoded address
    #[serde(rename_all = "camelCase")]
    HexToBech32 {
        /// Hex encoded bech32 address
        hex: String,
        /// Human readable part
        bech32_hrp: Hrp,
    },
    /// Transforms an alias id to a bech32 encoded address
    #[serde(rename_all = "camelCase")]
    AliasIdToBech32 {
        /// Alias ID
        alias_id: AliasId,
        /// Human readable part
        bech32_hrp: Hrp,
    },
    /// Transforms an nft id to a bech32 encoded address
    #[serde(rename_all = "camelCase")]
    NftIdToBech32 {
        /// Nft ID
        nft_id: NftId,
        /// Human readable part
        bech32_hrp: Hrp,
    },
    /// Transforms a hex encoded public key to a bech32 encoded address
    #[serde(rename_all = "camelCase")]
    HexPublicKeyToBech32Address {
        /// Hex encoded public key
        hex: String,
        /// Human readable part
        bech32_hrp: Hrp,
    },
    /// Returns a valid Address parsed from a String.
    ParseBech32Address {
        /// Address
        address: Bech32Address,
    },
    /// Checks if a String is a valid bech32 encoded address.
    IsAddressValid {
        /// Address
        address: String,
    },
    /// Generates a new mnemonic.
    GenerateMnemonic,
    /// Returns a hex encoded seed for a mnemonic.
    MnemonicToHexSeed {
        /// Mnemonic
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        mnemonic: String,
    },
    /// Returns a block ID (Blake2b256 hash of block bytes) from a block
    BlockId {
        /// Block
        block: BlockDto,
    },
    /// Returns a milestone ID (Blake2b256 hash of milestone essence)
    MilestoneId {
        /// Block
        payload: MilestonePayloadDto,
    },
    /// Returns the transaction ID (Blake2b256 hash of the provided transaction payload)
    TransactionId {
        /// Transaction Payload
        payload: TransactionPayloadDto,
    },
    /// Computes the alias ID
    #[serde(rename_all = "camelCase")]
    ComputeAliasId {
        /// Output ID
        output_id: OutputId,
    },
    /// Computes the Foundry ID
    #[serde(rename_all = "camelCase")]
    ComputeFoundryId {
        /// Alias address
        alias_id: AliasId,
        /// Serial number
        serial_number: u32,
        /// Token scheme kind
        token_scheme_kind: u8,
    },
    /// Computes the NFT ID
    #[serde(rename_all = "camelCase")]
    ComputeNftId {
        /// Output ID
        output_id: OutputId,
    },
    /// Returns the output ID from transaction id and output index
    ComputeOutputId { id: TransactionId, index: u16 },
    /// Constructs a tokenId from the aliasId, serial number and token scheme type.
    #[serde(rename_all = "camelCase")]
    ComputeTokenId {
        alias_id: AliasId,
        serial_number: u32,
        token_scheme_type: TokenScheme,
    },
    /// Compute the hash of a transaction essence.
    HashTransactionEssence {
        /// The transaction essence
        essence: TransactionEssenceDto,
    },
    /// Calculate the input commitment from the output objects that are used as inputs to fund the transaction.
    ComputeInputsCommitment { inputs: Vec<OutputDto> },
    /// Calculates the required storage deposit of an output.
    #[serde(rename_all = "camelCase")]
    ComputeStorageDeposit { output: OutputDto, rent: RentStructure },
    /// Checks if the given mnemonic is valid.
    /// Expected response: [`Ok`](crate::Response::Ok)
    VerifyMnemonic {
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        mnemonic: String,
    },
    /// Verify an ed25519 signature against a message.
    VerifyEd25519Signature {
        signature: Ed25519SignatureDto,
        message: String,
    },
    /// Verify a Secp256k1Ecdsa signature against a message.
    #[serde(rename_all = "camelCase")]
    VerifySecp256k1EcdsaSignature {
        public_key: String,
        signature: String,
        message: String,
    },
}
