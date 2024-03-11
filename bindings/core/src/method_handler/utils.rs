// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    keys::bip39::Mnemonic,
};
use iota_sdk::{
    client::{verify_mnemonic, Client},
    types::{
        block::{
            address::{AccountAddress, Address, ToBech32Ext},
            input::UtxoInput,
            output::{FoundryId, MinimumOutputAmount, Output, OutputId, TokenId},
            payload::{signed_transaction::Transaction, SignedTransactionPayload},
            semantic::SemanticValidationContext,
            signature::SignatureError,
            Block,
        },
        TryFromDto,
    },
};
use packable::PackableExt;

use crate::{method::UtilsMethod, response::Response};

/// Call a utils method.
pub(crate) fn call_utils_method_internal(method: UtilsMethod) -> Result<Response, crate::Error> {
    let response = match method {
        UtilsMethod::AddressToBech32 { address, bech32_hrp } => Response::Bech32Address(address.to_bech32(bech32_hrp)),
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
        UtilsMethod::Blake2b256Hash { bytes } => Response::Hash(prefix_hex::encode(Blake2b256::digest(bytes).to_vec())),
        UtilsMethod::ComputeFoundryId {
            account_id,
            serial_number,
            token_scheme_type,
        } => Response::FoundryId(FoundryId::build(
            &AccountAddress::new(account_id),
            serial_number,
            token_scheme_type,
        )),
        UtilsMethod::ComputeOutputId { id, index } => Response::OutputId(OutputId::new(id, index)),
        UtilsMethod::ComputeTokenId {
            account_id,
            serial_number,
            token_scheme_type,
        } => {
            let foundry_id = FoundryId::build(&AccountAddress::new(account_id), serial_number, token_scheme_type);
            Response::TokenId(TokenId::from(foundry_id))
        }
        UtilsMethod::ProtocolParametersHash { protocol_parameters } => {
            Response::Hash(protocol_parameters.hash().to_string())
        }
        UtilsMethod::TransactionSigningHash { transaction } => {
            Response::Hash(Transaction::try_from_dto(transaction)?.signing_hash().to_string())
        }
        UtilsMethod::ComputeMinimumOutputAmount {
            output,
            storage_score_parameters: storage_params,
        } => Response::Amount(output.minimum_amount(storage_params)),
        UtilsMethod::VerifyMnemonic { mnemonic } => {
            let mnemonic = Mnemonic::from(mnemonic);
            verify_mnemonic(mnemonic)?;
            Response::Ok
        }
        UtilsMethod::VerifyEd25519Signature { signature, message } => {
            let message: Vec<u8> = prefix_hex::decode(message)?;
            Response::Bool(
                signature
                    .try_verify(&message)
                    .map_err(iota_sdk::client::ClientError::from)?,
            )
        }
        UtilsMethod::VerifySecp256k1EcdsaSignature {
            public_key,
            signature,
            message,
        } => {
            use crypto::signatures::secp256k1_ecdsa;
            let public_key = prefix_hex::decode(public_key)?;
            let public_key =
                secp256k1_ecdsa::PublicKey::try_from_bytes(&public_key).map_err(SignatureError::PublicKeyBytes)?;
            let signature = prefix_hex::decode(signature)?;
            let signature =
                secp256k1_ecdsa::Signature::try_from_bytes(&signature).map_err(SignatureError::SignatureBytes)?;
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
            mana_rewards,
            protocol_parameters,
        } => {
            let transaction = Transaction::try_from_dto(transaction)?;
            let inputs = inputs
                .iter()
                .map(|input| (input.output_id(), &input.output))
                .collect::<Vec<(&OutputId, &Output)>>();

            let context = SemanticValidationContext::new(
                &transaction,
                &inputs,
                unlocks.as_deref(),
                mana_rewards.as_ref(),
                &protocol_parameters,
            );
            context.validate()?;

            Response::Ok
        }
        UtilsMethod::ManaWithDecay {
            mana,
            slot_index_created,
            slot_index_target,
            protocol_parameters,
        } => Response::Amount(protocol_parameters.mana_with_decay(mana, slot_index_created, slot_index_target)?),
        UtilsMethod::GenerateManaWithDecay {
            amount,
            slot_index_created,
            slot_index_target,
            protocol_parameters,
        } => Response::Amount(protocol_parameters.generate_mana_with_decay(
            amount,
            slot_index_created,
            slot_index_target,
        )?),
        UtilsMethod::OutputManaWithDecay {
            output,
            slot_index_created,
            slot_index_target,
            protocol_parameters,
        } => Response::DecayedMana(output.decayed_mana(&protocol_parameters, slot_index_created, slot_index_target)?),
        UtilsMethod::VerifyTransactionSyntax {
            transaction,
            protocol_parameters,
        } => {
            Transaction::try_from_dto_with_params(transaction, &protocol_parameters)?;
            Response::Ok
        }
        UtilsMethod::BlockBytes { block } => {
            let block = Block::try_from_dto(block)?;
            Response::Raw(block.pack_to_vec())
        }
        UtilsMethod::IotaMainnetProtocolParameters => {
            Response::ProtocolParameters(iota_sdk::types::block::protocol::iota_mainnet_protocol_parameters().clone())
        }
        UtilsMethod::ShimmerMainnetProtocolParameters => Response::ProtocolParameters(
            iota_sdk::types::block::protocol::shimmer_mainnet_protocol_parameters().clone(),
        ),
    };

    Ok(response)
}
