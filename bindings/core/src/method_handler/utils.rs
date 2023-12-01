// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip39::Mnemonic;
use iota_sdk::{
    client::{hex_public_key_to_bech32_address, hex_to_bech32, verify_mnemonic, Client},
    types::{
        block::{
            address::{AccountAddress, Address, ToBech32Ext},
            input::UtxoInput,
            output::{AccountId, FoundryId, MinimumOutputAmount, NftId, Output, OutputId, TokenId},
            payload::{signed_transaction::Transaction, SignedTransactionPayload},
            semantic::SemanticValidationContext,
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
        UtilsMethod::AccountIdToBech32 { account_id, bech32_hrp } => {
            Response::Bech32Address(account_id.to_bech32(bech32_hrp))
        }
        UtilsMethod::NftIdToBech32 { nft_id, bech32_hrp } => Response::Bech32Address(nft_id.to_bech32(bech32_hrp)),
        UtilsMethod::HexPublicKeyToBech32Address { hex, bech32_hrp } => {
            Response::Bech32Address(hex_public_key_to_bech32_address(&hex, bech32_hrp)?)
        }
        UtilsMethod::ParseBech32Address { address } => Response::ParsedBech32Address(address.into_inner()),
        UtilsMethod::IsAddressValid { address } => Response::Bool(Address::is_valid_bech32(&address)),
        UtilsMethod::GenerateMnemonic => Response::GeneratedMnemonic(Client::generate_mnemonic()?.to_string()),
        UtilsMethod::MnemonicToHexSeed { mnemonic } => {
            let mnemonic = Mnemonic::from(mnemonic);
            Response::MnemonicHexSeed(Client::mnemonic_to_hex_seed(mnemonic)?)
        }
        UtilsMethod::BlockId {
            block,
            protocol_parameters,
        } => {
            let block = Block::try_from_dto_with_params(block, &protocol_parameters)?;
            Response::BlockId(block.id(&protocol_parameters))
        }
        UtilsMethod::TransactionId { payload } => {
            let payload = SignedTransactionPayload::try_from_dto(payload)?;
            Response::TransactionId(payload.transaction().id())
        }
        UtilsMethod::ComputeAccountId { output_id } => Response::AccountId(AccountId::from(&output_id)),
        UtilsMethod::ComputeFoundryId {
            account_id,
            serial_number,
            token_scheme_type,
        } => Response::FoundryId(FoundryId::build(
            &AccountAddress::new(account_id),
            serial_number,
            token_scheme_type,
        )),
        UtilsMethod::ComputeNftId { output_id } => Response::NftId(NftId::from(&output_id)),
        UtilsMethod::ComputeOutputId { id, index } => Response::OutputId(OutputId::new(id, index)),
        UtilsMethod::ComputeTokenId {
            account_id,
            serial_number,
            token_scheme_type,
        } => {
            let foundry_id = FoundryId::build(&AccountAddress::new(account_id), serial_number, token_scheme_type);
            Response::TokenId(TokenId::from(foundry_id))
        }
        UtilsMethod::TransactionSigningHash { transaction } => {
            Response::Hash(Transaction::try_from_dto(transaction)?.signing_hash().to_string())
        }
        UtilsMethod::ComputeMinimumOutputAmount {
            output,
            storage_score_parameters: storage_params,
        } => Response::OutputAmount(output.minimum_amount(storage_params)),
        UtilsMethod::VerifyMnemonic { mnemonic } => {
            let mnemonic = Mnemonic::from(mnemonic);
            verify_mnemonic(mnemonic)?;
            Response::Ok
        }
        UtilsMethod::VerifyEd25519Signature { signature, message } => {
            let message: Vec<u8> = prefix_hex::decode(message)?;
            Response::Bool(signature.verify(&message))
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
        UtilsMethod::OutputIdToUtxoInput { output_id } => Response::Input(UtxoInput::from(output_id)),
        UtilsMethod::ComputeSlotCommitmentId { slot_commitment } => Response::SlotCommitmentId(slot_commitment.id()),
        UtilsMethod::OutputHexBytes { output } => Response::HexBytes(prefix_hex::encode(output.pack_to_vec())),
        UtilsMethod::VerifyTransactionSemantic {
            transaction,
            inputs,
            unlocks,
        } => {
            let transaction = Transaction::try_from_dto(transaction)?;
            let inputs = inputs
                .iter()
                .map(|input| (input.output_id(), &input.output))
                .collect::<Vec<(&OutputId, &Output)>>();

            let context = SemanticValidationContext::new(&transaction, &inputs, unlocks.as_deref());

            Response::TransactionFailureReason(context.validate()?)
        }
    };

    Ok(response)
}
