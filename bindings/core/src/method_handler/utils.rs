// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip39::Mnemonic;
use iota_sdk::{
    client::{
        api::verify_semantic, hex_public_key_to_bech32_address, hex_to_bech32, secret::types::InputSigningData,
        verify_mnemonic, Client,
    },
    types::{
        block::{
            address::{dto::AddressDto, Address, AliasAddress, ToBech32Ext},
            input::UtxoInput,
            output::{AliasId, FoundryId, InputsCommitment, NftId, Output, OutputId, Rent, TokenId},
            payload::{transaction::TransactionEssence, MilestonePayload, TransactionPayload},
            signature::Ed25519Signature,
            Block,
        },
        TryFromDto,
    },
};
use packable::PackableExt;

use crate::{method::UtilsMethod, response::Response, Result};

/// Call a utils method.
pub(crate) fn call_utils_method_internal(method: UtilsMethod) -> Result<Response> {
    let response = match method {
        UtilsMethod::Bech32ToHex { bech32 } => Response::Bech32ToHex(Client::bech32_to_hex(bech32)?),
        UtilsMethod::HexToBech32 { hex, bech32_hrp } => Response::Bech32Address(hex_to_bech32(&hex, bech32_hrp)?),
        UtilsMethod::AliasIdToBech32 { alias_id, bech32_hrp } => {
            Response::Bech32Address(alias_id.to_bech32(bech32_hrp))
        }
        UtilsMethod::NftIdToBech32 { nft_id, bech32_hrp } => Response::Bech32Address(nft_id.to_bech32(bech32_hrp)),
        UtilsMethod::HexPublicKeyToBech32Address { hex, bech32_hrp } => {
            Response::Bech32Address(hex_public_key_to_bech32_address(&hex, bech32_hrp)?)
        }
        UtilsMethod::ParseBech32Address { address } => Response::ParsedBech32Address(AddressDto::from(address.inner())),
        UtilsMethod::IsAddressValid { address } => Response::Bool(Address::is_valid_bech32(&address)),
        UtilsMethod::GenerateMnemonic => Response::GeneratedMnemonic(Client::generate_mnemonic()?.to_string()),
        UtilsMethod::MnemonicToHexSeed { mnemonic } => {
            let mnemonic = Mnemonic::from(mnemonic);
            Response::MnemonicHexSeed(Client::mnemonic_to_hex_seed(mnemonic)?)
        }
        UtilsMethod::BlockId { block } => {
            let block = Block::try_from_dto(block)?;
            Response::BlockId(block.id())
        }
        UtilsMethod::MilestoneId { payload } => {
            let payload = MilestonePayload::try_from_dto(payload)?;
            Response::MilestoneId(payload.id())
        }
        UtilsMethod::TransactionId { payload } => {
            let payload = TransactionPayload::try_from_dto(payload)?;
            Response::TransactionId(payload.id())
        }
        UtilsMethod::ComputeAliasId { output_id } => Response::AliasId(AliasId::from(&output_id)),
        UtilsMethod::ComputeFoundryId {
            alias_id,
            serial_number,
            token_scheme_type,
        } => Response::FoundryId(FoundryId::build(
            &AliasAddress::new(alias_id),
            serial_number,
            token_scheme_type,
        )),
        UtilsMethod::ComputeNftId { output_id } => Response::NftId(NftId::from(&output_id)),
        UtilsMethod::ComputeOutputId { id, index } => Response::OutputId(OutputId::new(id, index)?),
        UtilsMethod::ComputeTokenId {
            alias_id,
            serial_number,
            token_scheme_type,
        } => {
            let foundry_id = FoundryId::build(&AliasAddress::new(alias_id), serial_number, token_scheme_type);
            Response::TokenId(TokenId::from(foundry_id))
        }
        UtilsMethod::HashTransactionEssence { essence } => {
            Response::Hash(prefix_hex::encode(TransactionEssence::try_from_dto(essence)?.hash()))
        }
        UtilsMethod::ComputeInputsCommitment { inputs } => {
            let inputs = inputs
                .into_iter()
                .map(|o| Ok(Output::try_from_dto(o)?))
                .collect::<Result<Vec<Output>>>()?;
            Response::Hash(InputsCommitment::new(inputs.iter()).to_string())
        }
        UtilsMethod::ComputeStorageDeposit { output, rent } => {
            let out = Output::try_from_dto(output)?;
            Response::MinimumRequiredStorageDeposit(out.rent_cost(&rent).to_string())
        }
        UtilsMethod::VerifyMnemonic { mnemonic } => {
            let mnemonic = Mnemonic::from(mnemonic);
            verify_mnemonic(mnemonic)?;
            Response::Ok
        }
        UtilsMethod::VerifyEd25519Signature { signature, message } => {
            use iota_sdk::types::block::Error;
            let signature = Ed25519Signature::try_from(signature)?;
            let message: Vec<u8> = prefix_hex::decode(message)?;
            Response::Bool(signature.try_verify(&message).map_err(Error::from)?)
        }
        UtilsMethod::VerifySecp256k1EcdsaSignature {
            public_key,
            signature,
            message,
        } => {
            use crypto::signatures::secp256k1_ecdsa;
            use iota_sdk::types::block::Error;
            let public_key = prefix_hex::decode(public_key)?;
            let public_key = secp256k1_ecdsa::PublicKey::try_from_bytes(&public_key).map_err(Error::from)?;
            let signature = prefix_hex::decode(signature)?;
            let signature = secp256k1_ecdsa::Signature::try_from_bytes(&signature).map_err(Error::from)?;
            let message: Vec<u8> = prefix_hex::decode(message)?;
            Response::Bool(public_key.verify_keccak256(&signature, &message))
        }
        UtilsMethod::OutputIdToUtxoInput { output_id } => Response::Input((&UtxoInput::from(output_id)).into()),
        UtilsMethod::OutputHexBytes { output } => {
            let output = Output::try_from_dto(output)?;
            Response::HexBytes(prefix_hex::encode(output.pack_to_vec()))
        }
        UtilsMethod::VerifyTransactionSemantic {
            inputs,
            transaction,
            time,
        } => {
            let conflict = verify_semantic(
                &inputs
                    .into_iter()
                    .map(InputSigningData::try_from_dto)
                    .collect::<iota_sdk::client::Result<Vec<InputSigningData>>>()?,
                &TransactionPayload::try_from_dto(transaction)?,
                time,
            )?;
            Response::ConflictReason(conflict)
        }
    };
    Ok(response)
}
