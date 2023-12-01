// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derivative::Derivative;
use iota_sdk::{
    client::secret::types::InputSigningData,
    types::block::{
        address::{Bech32Address, Hrp},
        output::{AccountId, NftId, Output, OutputId, StorageScoreParameters},
        payload::signed_transaction::{
            dto::{SignedTransactionPayloadDto, TransactionDto},
            TransactionId,
        },
        protocol::ProtocolParameters,
        signature::Ed25519Signature,
        slot::SlotCommitment,
        unlock::Unlock,
        BlockDto,
    },
};
use serde::{Deserialize, Serialize};

use crate::OmittedDebug;

/// Each public utils method.
#[derive(Clone, Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
#[serde(tag = "name", content = "data", rename_all = "camelCase")]
#[non_exhaustive]
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
    /// Transforms an account id to a bech32 encoded address
    #[serde(rename_all = "camelCase")]
    AccountIdToBech32 {
        /// Account ID
        account_id: AccountId,
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
    /// Returns a block ID from a block and slot protocol parameters
    #[serde(rename_all = "camelCase")]
    BlockId {
        /// Block
        block: BlockDto,
        /// Network Protocol Parameters
        protocol_parameters: ProtocolParameters,
    },
    /// Returns the transaction ID (Blake2b256 hash of the provided transaction payload)
    TransactionId {
        /// Transaction Payload
        payload: SignedTransactionPayloadDto,
    },
    /// Computes the account ID
    #[serde(rename_all = "camelCase")]
    ComputeAccountId {
        /// Output ID
        output_id: OutputId,
    },
    /// Computes the Foundry ID
    #[serde(rename_all = "camelCase")]
    ComputeFoundryId {
        /// Alias id
        account_id: AccountId,
        /// Serial number
        serial_number: u32,
        /// Token scheme kind
        token_scheme_type: u8,
    },
    /// Computes the NFT ID
    #[serde(rename_all = "camelCase")]
    ComputeNftId {
        /// Output ID
        output_id: OutputId,
    },
    /// Computes the output ID from transaction id and output index
    ComputeOutputId { id: TransactionId, index: u16 },
    /// Computes a tokenId from the aliasId, serial number and token scheme type.
    #[serde(rename_all = "camelCase")]
    ComputeTokenId {
        account_id: AccountId,
        serial_number: u32,
        token_scheme_type: u8,
    },
    /// Computes the hash of the given protocol parameters.
    #[serde(rename_all = "camelCase")]
    ProtocolParametersHash {
        /// Network Protocol Parameters
        protocol_parameters: ProtocolParameters,
    },
    /// Computes the signing hash of a transaction.
    TransactionSigningHash {
        /// The transaction.
        transaction: TransactionDto,
    },
    /// Computes the minimum required amount of an output.
    #[serde(rename_all = "camelCase")]
    ComputeMinimumOutputAmount {
        output: Output,
        storage_score_parameters: StorageScoreParameters,
    },
    /// Checks if the given mnemonic is valid.
    /// Expected response: [`Ok`](crate::Response::Ok)
    VerifyMnemonic {
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        mnemonic: String,
    },
    /// Verify an ed25519 signature against a message.
    VerifyEd25519Signature {
        signature: Ed25519Signature,
        message: String,
    },
    /// Verify a Secp256k1Ecdsa signature against a message.
    #[serde(rename_all = "camelCase")]
    VerifySecp256k1EcdsaSignature {
        public_key: String,
        signature: String,
        message: String,
    },
    /// Creates a UTXOInput from outputId.
    #[serde(rename_all = "camelCase")]
    OutputIdToUtxoInput { output_id: OutputId },
    /// Computes the slot commitment id from a slot commitment.
    #[serde(rename_all = "camelCase")]
    ComputeSlotCommitmentId { slot_commitment: SlotCommitment },
    /// Returns the hex representation of the serialized output bytes.
    #[serde(rename_all = "camelCase")]
    OutputHexBytes { output: Output },
    /// Verifies the semantic of a transaction.
    VerifyTransactionSemantic {
        transaction: TransactionDto,
        inputs: Vec<InputSigningData>,
        unlocks: Option<Vec<Unlock>>,
    },
}
