// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::{hex_public_key_to_bech32_address, hex_to_bech32, request_funds_from_faucet, Client},
    types::block::{
        address::{dto::AddressDto, Address, Ed25519Address},
        output::{AliasId, FoundryId, NftId},
        payload::{transaction::TransactionEssence, TransactionPayload},
        signature::Ed25519Signature,
        Block,
    },
};
use zeroize::Zeroize;

use crate::{method::UtilityMethod, method_handler::Result, response::Response};

/// Call a utility method.
pub(crate) async fn call_utility_method_internal(method: UtilityMethod) -> Result<Response> {
    match method {
        UtilityMethod::Bech32ToHex { bech32 } => Ok(Response::Bech32ToHex(Client::bech32_to_hex(&bech32)?)),
        UtilityMethod::HexToBech32 { hex, bech32_hrp } => {
            Ok(Response::Bech32Address(hex_to_bech32(&hex, &bech32_hrp)?))
        }
        UtilityMethod::AliasIdToBech32 { alias_id, bech32_hrp } => {
            Ok(Response::Bech32Address(alias_id.to_bech32(&bech32_hrp)))
        }
        UtilityMethod::NftIdToBech32 { nft_id, bech32_hrp } => {
            Ok(Response::Bech32Address(nft_id.to_bech32(&bech32_hrp)))
        }
        UtilityMethod::HexPublicKeyToBech32Address { hex, bech32_hrp } => Ok(Response::Bech32Address(
            hex_public_key_to_bech32_address(&hex, &bech32_hrp)?,
        )),
        UtilityMethod::ParseBech32Address { address } => Ok(Response::ParsedBech32Address(AddressDto::from(
            &Address::try_from_bech32(address)?,
        ))),
        UtilityMethod::IsAddressValid { address } => Ok(Response::Bool(Address::is_valid_bech32(&address))),
        UtilityMethod::GenerateMnemonic => Ok(Response::GeneratedMnemonic(Client::generate_mnemonic()?)),
        UtilityMethod::MnemonicToHexSeed { mut mnemonic } => {
            let response = Response::MnemonicHexSeed(Client::mnemonic_to_hex_seed(&mnemonic)?);

            mnemonic.zeroize();

            Ok(response)
        }
        UtilityMethod::BlockId { block } => {
            let block = Block::try_from_dto_unverified(&block)?;
            Ok(Response::BlockId(block.id()))
        }
        UtilityMethod::TransactionId { payload } => {
            let payload = TransactionPayload::try_from_dto_unverified(&payload)?;
            Ok(Response::TransactionId(payload.id()))
        }
        UtilityMethod::ComputeAliasId { output_id } => Ok(Response::AliasId(AliasId::from(&output_id))),
        UtilityMethod::ComputeNftId { output_id } => Ok(Response::NftId(NftId::from(&output_id))),
        UtilityMethod::ComputeFoundryId {
            alias_address,
            serial_number,
            token_scheme_kind,
        } => Ok(Response::FoundryId(FoundryId::build(
            &alias_address,
            serial_number,
            token_scheme_kind,
        ))),
        UtilityMethod::Faucet { url, address } => {
            Ok(Response::Faucet(request_funds_from_faucet(&url, &address).await?))
        }
        UtilityMethod::HashTransactionEssence { essence } => Ok(Response::TransactionEssenceHash(prefix_hex::encode(
            TransactionEssence::try_from_dto_unverified(&essence)?.hash(),
        ))),
        UtilityMethod::VerifyEd25519Signature {
            signature,
            message,
            address,
        } => {
            let signature = Ed25519Signature::try_from(&signature)?;
            let msg: Vec<u8> = prefix_hex::decode(message)?;
            let address = Ed25519Address::try_from(&address)?;
            Ok(Response::Bool(signature.is_valid(&msg, &address).is_ok()))
        }
    }
}
