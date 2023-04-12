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
    HexToBech32 {
        /// Hex encoded bech32 address
        hex: String,
        /// Human readable part
        #[serde(rename = "bech32Hrp")]
        bech32_hrp: String,
    },
    /// Transforms an alias id to a bech32 encoded address
    AliasIdToBech32 {
        /// Alias ID
        #[serde(rename = "aliasId")]
        alias_id: AliasId,
        /// Human readable part
        #[serde(rename = "bech32Hrp")]
        bech32_hrp: String,
    },
    /// Transforms an nft id to a bech32 encoded address
    NftIdToBech32 {
        /// Nft ID
        #[serde(rename = "nftId")]
        nft_id: NftId,
        /// Human readable part
        #[serde(rename = "bech32Hrp")]
        bech32_hrp: String,
    },
    /// Transforms a hex encoded public key to a bech32 encoded address
    HexPublicKeyToBech32Address {
        /// Hex encoded public key
        hex: String,
        /// Human readable part
        #[serde(rename = "bech32Hrp")]
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
    ComputeAliasId {
        /// Output ID
        #[serde(rename = "outputId")]
        output_id: OutputId,
    },
    /// Computes the NFT ID
    ComputeNftId {
        /// Output ID
        #[serde(rename = "outputId")]
        output_id: OutputId,
    },
    /// Computes the Foundry ID
    ComputeFoundryId {
        /// Alias address
        #[serde(rename = "aliasAddress")]
        alias_address: AliasAddress,
        /// Serial number
        #[serde(rename = "serialNumber")]
        serial_number: u32,
        /// Token scheme kind
        #[serde(rename = "tokenSchemeKind")]
        token_scheme_kind: u8,
    },
    /// Requests funds for a given address from the faucet.
    Faucet {
        /// Faucet URL
        url: String,
        /// The address for request funds
        address: String,
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
}
