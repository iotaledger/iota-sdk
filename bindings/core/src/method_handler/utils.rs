// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::{hex_public_key_to_bech32_address, hex_to_bech32, request_funds_from_faucet, verify_mnemonic, Client},
    types::block::{
        address::{dto::AddressDto, Address, Ed25519Address},
        output::{AliasId, FoundryId, NftId},
        payload::{transaction::TransactionEssence, TransactionPayload},
        signature::Ed25519Signature,
        Block,
    },
};
use zeroize::Zeroize;

use crate::{method::UtilsMethod, response::Response, Result};

/// Call a utils method.
pub(crate) async fn call_utils_method_internal(method: UtilsMethod) -> Result<Response> {
    match method {
        UtilsMethod::Bech32ToHex { bech32 } => Ok(Response::Bech32ToHex(Client::bech32_to_hex(&bech32)?)),
        UtilsMethod::HexToBech32 { hex, bech32_hrp } => Ok(Response::Bech32Address(hex_to_bech32(&hex, &bech32_hrp)?)),
        UtilsMethod::AliasIdToBech32 { alias_id, bech32_hrp } => {
            Ok(Response::Bech32Address(alias_id.to_bech32(&bech32_hrp)))
        }
        UtilsMethod::NftIdToBech32 { nft_id, bech32_hrp } => Ok(Response::Bech32Address(nft_id.to_bech32(&bech32_hrp))),
        UtilsMethod::HexPublicKeyToBech32Address { hex, bech32_hrp } => Ok(Response::Bech32Address(
            hex_public_key_to_bech32_address(&hex, &bech32_hrp)?,
        )),
        UtilsMethod::ParseBech32Address { address } => Ok(Response::ParsedBech32Address(AddressDto::from(
            &Address::try_from_bech32(address)?,
        ))),
        UtilsMethod::IsAddressValid { address } => Ok(Response::Bool(Address::is_valid_bech32(&address))),
        UtilsMethod::GenerateMnemonic => Ok(Response::GeneratedMnemonic(Client::generate_mnemonic()?)),
        UtilsMethod::MnemonicToHexSeed { mut mnemonic } => {
            let response = Response::MnemonicHexSeed(Client::mnemonic_to_hex_seed(&mnemonic)?);
            mnemonic.zeroize();
            Ok(response)
        }
        UtilsMethod::BlockId { block } => {
            let block = Block::try_from_dto_unverified(&block)?;
            Ok(Response::BlockId(block.id()))
        }
        UtilsMethod::TransactionId { payload } => {
            let payload = TransactionPayload::try_from_dto_unverified(&payload)?;
            Ok(Response::TransactionId(payload.id()))
        }
        UtilsMethod::ComputeAliasId { output_id } => Ok(Response::AliasId(AliasId::from(&output_id))),
        UtilsMethod::ComputeNftId { output_id } => Ok(Response::NftId(NftId::from(&output_id))),
        UtilsMethod::ComputeFoundryId {
            alias_address,
            serial_number,
            token_scheme_kind,
        } => Ok(Response::FoundryId(FoundryId::build(
            &alias_address,
            serial_number,
            token_scheme_kind,
        ))),
        UtilsMethod::Faucet { url, address } => Ok(Response::Faucet(request_funds_from_faucet(&url, &address).await?)),
        UtilsMethod::HashTransactionEssence { essence } => Ok(Response::TransactionEssenceHash(prefix_hex::encode(
            TransactionEssence::try_from_dto_unverified(&essence)?.hash(),
        ))),
        UtilsMethod::VerifyEd25519Signature {
            signature,
            message,
            address,
        } => {
            let signature = Ed25519Signature::try_from(&signature)?;
            let msg: Vec<u8> = prefix_hex::decode(message)?;
            let address = Ed25519Address::try_from(&address)?;
            Ok(Response::Bool(signature.is_valid(&msg, &address).is_ok()))
        }
        UtilsMethod::VerifyMnemonic { mut mnemonic } => {
            verify_mnemonic(&mnemonic)?;
            mnemonic.zeroize();
            Ok(Response::Ok)
        }
    }
}
