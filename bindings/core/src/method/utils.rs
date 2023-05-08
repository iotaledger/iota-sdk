// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derivative::Derivative;
use iota_sdk::types::block::{
    address::{dto::Ed25519AddressDto, AliasAddress},
    output::{AliasId, NftId, OutputId},
    payload::transaction::dto::{TransactionEssenceDto, TransactionPayloadDto},
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
        bech32: String,
    },
    /// Transforms a hex encoded address to a bech32 encoded address
    #[serde(rename_all = "camelCase")]
    HexToBech32 {
        /// Hex encoded bech32 address
        hex: String,
        /// Human readable part
        bech32_hrp: String,
    },
    /// Transforms an alias id to a bech32 encoded address
    #[serde(rename_all = "camelCase")]
    AliasIdToBech32 {
        /// Alias ID
        alias_id: AliasId,
        /// Human readable part
        bech32_hrp: String,
    },
    /// Transforms an nft id to a bech32 encoded address
    #[serde(rename_all = "camelCase")]
    NftIdToBech32 {
        /// Nft ID
        nft_id: NftId,
        /// Human readable part
        bech32_hrp: String,
    },
    /// Transforms a hex encoded public key to a bech32 encoded address
    #[serde(rename_all = "camelCase")]
    HexPublicKeyToBech32Address {
        /// Hex encoded public key
        hex: String,
        /// Human readable part
        bech32_hrp: String,
    },
    /// Returns a valid Address parsed from a String.
    ParseBech32Address {
        /// Address
        address: String,
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
    /// Computes the NFT ID
    #[serde(rename_all = "camelCase")]
    ComputeNftId {
        /// Output ID
        output_id: OutputId,
    },
    /// Computes the Foundry ID
    #[serde(rename_all = "camelCase")]
    ComputeFoundryId {
        /// Alias address
        alias_address: AliasAddress,
        /// Serial number
        serial_number: u32,
        /// Token scheme kind
        token_scheme_kind: u8,
    },
    /// Compute the hash of a transaction essence.
    HashTransactionEssence {
        /// The transaction essence
        essence: TransactionEssenceDto,
    },
    /// Verifies the Ed25519Signature for a message against an Ed25519Address.
    VerifyEd25519Signature {
        /// The Ed25519 Signature
        signature: Ed25519SignatureDto,
        /// The signed message, hex encoded String
        message: String,
        /// The hex encoded Ed25519 address
        address: Ed25519AddressDto,
    },
    /// Checks if the given mnemonic is valid.
    /// Expected response: [`Ok`](crate::Response::Ok)
    VerifyMnemonic {
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        mnemonic: String,
    },
}
