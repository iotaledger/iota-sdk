// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Implementation of [`LedgerSecretManager`].
//!
//! Ledger status codes: <https://github.com/iotaledger/ledger-iota-app/blob/53c1f96d15f8b014ba8ba31a85f0401bb4d33e18/src/iota_io.h#L54>.

use std::{collections::HashMap, ops::Range};

use async_trait::async_trait;
use crypto::{
    keys::{bip44::Bip44, slip10::Segment},
    signatures::{
        ed25519,
        secp256k1_ecdsa::{self, EvmAddress},
    },
};
use iota_ledger_nano::{
    api::errors::APIError, get_app_config, get_buffer_size, get_ledger, get_opened_app, LedgerBIP32Index,
    Packable as LedgerNanoPackable, TransportTypes,
};
use packable::{error::UnexpectedEOF, unpacker::SliceUnpacker, Packable, PackableExt};
use tokio::sync::Mutex;

use super::{GenerateAddressOptions, SecretManage, SecretManagerConfig};
use crate::{
    client::secret::{
        types::{LedgerApp, LedgerDeviceType},
        LedgerNanoStatus, PreparedTransactionData,
    },
    types::block::{
        address::{AccountAddress, Address, NftAddress},
        output::Output,
        payload::signed_transaction::SignedTransactionPayload,
        protocol::ProtocolParameters,
        signature::{Ed25519Signature, Signature},
        unlock::{AccountUnlock, NftUnlock, ReferenceUnlock, SignatureUnlock, Unlock, Unlocks},
        Error as BlockError,
    },
};

/// Ledger nano errors.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Denied by User
    #[error("denied by user")]
    DeniedByUser,
    /// Dongle Locked
    #[error("ledger locked")]
    DongleLocked,
    /// Ledger Device not found
    #[error("ledger device not found")]
    DeviceNotFound,
    /// Ledger Essence Too Large
    #[error("ledger essence too large")]
    EssenceTooLarge,
    /// Ledger transport error
    #[error("ledger transport error")]
    MiscError,
    /// Unsupported operation
    #[error("unsupported operation")]
    UnsupportedOperation,
    /// Block error
    #[error("{0}")]
    Block(Box<crate::types::block::Error>),
    /// Missing input with ed25519 address
    #[error("missing input with ed25519 address")]
    MissingInputWithEd25519Address,
    /// Missing bip32 chain
    #[error("missing bip32 chain")]
    MissingBip32Chain,
    /// Bip32 chain mismatch
    #[error("Bip32 chain mismatch")]
    Bip32ChainMismatch,
    /// Unpack error
    #[error("{0}")]
    Unpack(#[from] packable::error::UnpackError<crate::types::block::Error, UnexpectedEOF>),
    /// No available inputs provided
    #[error("No available inputs provided")]
    NoAvailableInputsProvided,
    /// Output not unlockable due to deadzone in expiration unlock condition.
    #[error("output not unlockable due to deadzone in expiration unlock condition")]
    ExpirationDeadzone,
}

impl From<crate::types::block::Error> for Error {
    fn from(error: crate::types::block::Error) -> Self {
        Self::Block(Box::new(error))
    }
}

// map most errors to a single error but there are some errors that
// need special care.
// LedgerDongleLocked: Ask the user to unlock the dongle
// LedgerDeniedByUser: The user denied a signing
// LedgerDeviceNotFound: No usable Ledger device was found
// LedgerMiscError: Everything else.
// LedgerEssenceTooLarge: Essence with bip32 input indices need more space then the internal buffer is big
impl From<APIError> for Error {
    fn from(error: APIError) -> Self {
        log::info!("ledger error: {}", error);
        match error {
            APIError::ConditionsOfUseNotSatisfied => Self::DeniedByUser,
            APIError::EssenceTooLarge => Self::EssenceTooLarge,
            APIError::SecurityStatusNotSatisfied => Self::DongleLocked,
            APIError::TransportError => Self::DeviceNotFound,
            _ => Self::MiscError,
        }
    }
}

/// Secret manager that uses a Ledger hardware wallet.
#[derive(Default, Debug)]
pub struct LedgerSecretManager {
    /// Specifies if a real Ledger hardware is used or only a simulator is used.
    pub is_simulator: bool,
    /// Specifies whether the wallet should be in non-interactive mode.
    pub non_interactive: bool,
    /// Mutex to prevent multiple simultaneous requests to a ledger.
    mutex: Mutex<()>,
}

impl TryFrom<u8> for LedgerDeviceType {
    type Error = Error;

    fn try_from(device: u8) -> Result<Self, Self::Error> {
        match device {
            0 => Ok(Self::LedgerNanoS),
            1 => Ok(Self::LedgerNanoX),
            2 => Ok(Self::LedgerNanoSPlus),
            _ => Err(Error::MiscError),
        }
    }
}

#[async_trait]
impl SecretManage for LedgerSecretManager {
    type Error = crate::client::Error;

    async fn generate_ed25519_public_keys(
        &self,
        // https://github.com/satoshilabs/slips/blob/master/slip-0044.md
        // current ledger app only supports IOTA_COIN_TYPE, SHIMMER_COIN_TYPE and TESTNET_COIN_TYPE
        _coin_type: u32,
        _account_index: u32,
        _address_indexes: Range<u32>,
        _options: impl Into<Option<GenerateAddressOptions>> + Send,
    ) -> Result<Vec<ed25519::PublicKey>, Self::Error> {
        // need an update on the ledger C lib
        todo!();
        // let options = options.into().unwrap_or_default();
        // let bip32_account = account_index.harden().into();

        // let bip32 = LedgerBIP32Index {
        //     bip32_index: address_indexes.start.harden().into(),
        //     bip32_change: u32::from(options.internal).harden().into(),
        // };

        // // lock the mutex to prevent multiple simultaneous requests to a ledger
        // let lock = self.mutex.lock().await;

        // // get ledger
        // let ledger = get_ledger(coin_type, bip32_account, self.is_simulator).map_err(Error::from)?;
        // if ledger.is_debug_app() {
        //     ledger
        //         .set_non_interactive_mode(self.non_interactive)
        //         .map_err(Error::from)?;
        // }

        // let addresses = ledger
        //     .get_addresses(options.ledger_nano_prompt, bip32, address_indexes.len())
        //     .map_err(Error::from)?;

        // drop(lock);

        // Ok(addresses.into_iter().map(Ed25519Address::new).collect())
    }

    async fn generate_evm_addresses(
        &self,
        _coin_type: u32,
        _account_index: u32,
        _address_indexes: Range<u32>,
        _options: impl Into<Option<GenerateAddressOptions>> + Send,
    ) -> Result<Vec<EvmAddress>, Self::Error> {
        Err(Error::UnsupportedOperation.into())
    }

    /// Ledger only allows signing messages of 32 bytes, anything else is unsupported and will result in an error.
    async fn sign_ed25519(&self, msg: &[u8], chain: Bip44) -> Result<Ed25519Signature, Self::Error> {
        if msg.len() != 32 {
            return Err(Error::UnsupportedOperation.into());
        }

        let msg = msg.to_vec();

        let coin_type = chain.coin_type;
        let account_index = chain.account.harden().into();
        let bip32_index = LedgerBIP32Index {
            bip32_change: chain.change.harden().into(),
            bip32_index: chain.address_index.harden().into(),
        };

        // Lock the mutex to prevent multiple simultaneous requests to a ledger.
        let lock = self.mutex.lock().await;

        let ledger = get_ledger(coin_type, account_index, self.is_simulator).map_err(Error::from)?;
        if ledger.is_debug_app() {
            ledger
                .set_non_interactive_mode(self.non_interactive)
                .map_err(Error::from)?;
        }

        log::debug!("[LEDGER] prepare_blind_signing");
        log::debug!("[LEDGER] {:?} {:?}", bip32_index, msg);
        ledger
            .prepare_blind_signing(vec![bip32_index], msg)
            .map_err(Error::from)?;

        // Show transaction to user, if denied by user, it returns with `DeniedByUser` Error.
        log::debug!("[LEDGER] await user confirmation");
        ledger.user_confirm().map_err(Error::from)?;

        // Sign.
        let signature_bytes = ledger.sign(1).map_err(Error::from)?;

        drop(ledger);
        drop(lock);

        let mut unpacker = SliceUnpacker::new(&signature_bytes);

        // Unpack and return signature.
        return match Unlock::unpack::<_, true>(&mut unpacker, &())? {
            Unlock::Signature(s) => match *s {
                SignatureUnlock(Signature::Ed25519(signature)) => Ok(signature),
            },
            _ => Err(Error::UnsupportedOperation.into()),
        };
    }

    async fn sign_secp256k1_ecdsa(
        &self,
        _msg: &[u8],
        _chain: Bip44,
    ) -> Result<(secp256k1_ecdsa::PublicKey, secp256k1_ecdsa::RecoverableSignature), Self::Error> {
        Err(Error::UnsupportedOperation.into())
    }

    async fn transaction_unlocks(
        &self,
        prepared_transaction: &PreparedTransactionData,
        protocol_parameters: &ProtocolParameters,
    ) -> Result<Unlocks, <Self as SecretManage>::Error> {
        let mut input_bip32_indices = Vec::new();
        let mut coin_type = None;
        let mut account_index = None;

        let input_len = prepared_transaction.inputs_data.len();

        for input in &prepared_transaction.inputs_data {
            let chain = input.chain.ok_or(Error::MissingBip32Chain)?;

            // coin_type and account_index should be the same in each output
            if (coin_type.is_some() && coin_type != Some(chain.coin_type))
                || (account_index.is_some() && account_index != Some(chain.account))
            {
                return Err(Error::Bip32ChainMismatch.into());
            }

            coin_type = Some(chain.coin_type);
            account_index = Some(chain.account);
            input_bip32_indices.push(LedgerBIP32Index {
                bip32_change: chain.change.harden().into(),
                bip32_index: chain.address_index.harden().into(),
            });
        }

        let (coin_type, account_index) = coin_type.zip(account_index).ok_or(Error::NoAvailableInputsProvided)?;

        let bip32_account = account_index.harden().into();

        let transaction_bytes = prepared_transaction.transaction.pack_to_vec();
        let transaction_signing_hash = prepared_transaction.transaction.signing_hash().to_vec();

        // lock the mutex to prevent multiple simultaneous requests to a ledger
        let lock = self.mutex.lock().await;

        let ledger = get_ledger(coin_type, bip32_account, self.is_simulator).map_err(Error::from)?;
        if ledger.is_debug_app() {
            ledger
                .set_non_interactive_mode(self.non_interactive)
                .map_err(Error::from)?;
        }
        let blind_signing = needs_blind_signing(prepared_transaction, ledger.get_buffer_size());

        // if transaction + bip32 input indices are larger than the buffer size or the transaction contains
        // features / types that are not supported blind signing will be needed
        if blind_signing {
            // prepare signing
            log::debug!("[LEDGER] prepare_blind_signing");
            log::debug!("[LEDGER] {:?} {:?}", input_bip32_indices, transaction_signing_hash);
            ledger
                .prepare_blind_signing(input_bip32_indices, transaction_signing_hash)
                .map_err(Error::from)?;
        } else {
            // figure out the remainder output and bip32 index (if there is one)
            #[allow(clippy::option_if_let_else)]
            let (remainder_output, remainder_bip32) = match &prepared_transaction.remainder {
                Some(remainder) => {
                    if let Some(chain) = remainder.chain {
                        (
                            Some(&remainder.output),
                            LedgerBIP32Index {
                                bip32_change: chain.change.harden().into(),
                                bip32_index: chain.address_index.harden().into(),
                            },
                        )
                    } else {
                        (None, LedgerBIP32Index::default())
                    }
                }
                None => (None, LedgerBIP32Index::default()),
            };

            let mut remainder_index = 0u16;
            if let Some(remainder_output) = remainder_output {
                // Find the index of the remainder in the transaction because it is not always the last output.
                // The index within the transaction and the bip32 index will be validated by the hardware
                // wallet.
                for output in prepared_transaction.transaction.outputs().iter() {
                    if !output.is_basic() {
                        log::debug!("[LEDGER] unsupported output");
                        return Err(Error::MiscError.into());
                    }

                    if remainder_output == output {
                        break;
                    }

                    remainder_index += 1;
                }

                // Was index found?
                if remainder_index as usize == prepared_transaction.transaction.outputs().len() {
                    log::debug!("[LEDGER] remainder_index not found");
                    return Err(Error::MiscError.into());
                }

                // was index found?
                if remainder_index as usize == prepared_transaction.transaction.outputs().len() {
                    log::debug!("[LEDGER] remainder_index not found");
                    return Err(Error::MiscError.into());
                }
            }

            // prepare signing
            log::debug!("[LEDGER] prepare signing");
            log::debug!(
                "[LEDGER] {:?} {:02x?} {} {} {:?}",
                input_bip32_indices,
                transaction_bytes,
                remainder_output.is_some(),
                remainder_index,
                remainder_bip32
            );
            ledger
                .prepare_signing(
                    input_bip32_indices,
                    transaction_bytes,
                    remainder_output.is_some(),
                    remainder_index,
                    remainder_bip32,
                )
                .map_err(Error::from)?;
        }

        // show transaction to user
        // if denied by user, it returns with `DeniedByUser` Error
        log::debug!("[LEDGER] await user confirmation");
        ledger.user_confirm().map_err(Error::from)?;

        // sign
        let signature_bytes = ledger.sign(input_len as u16).map_err(Error::from)?;
        drop(ledger);
        drop(lock);
        let mut unpacker = SliceUnpacker::new(&signature_bytes);

        // unpack signature to unlocks
        let mut unlocks = Vec::new();
        for _ in 0..input_len {
            let unlock = Unlock::unpack::<_, true>(&mut unpacker, &())?;
            // The ledger nano can return the same SignatureUnlocks multiple times, so only insert it once
            match unlock {
                Unlock::Signature(_) => {
                    if !unlocks.contains(&unlock) {
                        unlocks.push(unlock);
                    }
                }
                // Multiple reference unlocks with the same index are allowed
                _ => unlocks.push(unlock),
            }
        }

        // With blind signing the ledger only returns SignatureUnlocks, so we might have to merge them with
        // Account/Nft/Reference unlocks
        if blind_signing {
            unlocks = merge_unlocks(prepared_transaction, unlocks.into_iter(), protocol_parameters)?;
        }

        Ok(Unlocks::new(unlocks)?)
    }

    async fn sign_transaction(
        &self,
        prepared_transaction_data: PreparedTransactionData,
        protocol_parameters: &ProtocolParameters,
    ) -> Result<SignedTransactionPayload, Self::Error> {
        super::default_sign_transaction(self, prepared_transaction_data, protocol_parameters).await
    }
}

impl SecretManagerConfig for LedgerSecretManager {
    type Config = bool;

    fn to_config(&self) -> Option<Self::Config> {
        Some(self.is_simulator)
    }

    fn from_config(config: &Self::Config) -> Result<Self, Self::Error> {
        Ok(Self::new(*config))
    }
}

/// the Ledger Nano S(+)/X app can present the user a detailed view of the transaction before it
/// is signed but only with BasicOutputs, without extra-features and if the transaction is not too large.
/// If criteria are not met, blind signing is needed.
/// This method finds out if we have to switch to blind signing mode.
pub fn needs_blind_signing(prepared_transaction: &PreparedTransactionData, buffer_size: usize) -> bool {
    if !prepared_transaction.transaction.outputs().iter().all(
        |output| matches!(output, Output::Basic(o) if o.simple_deposit_address().is_some() && o.address().is_ed25519()),
    ) {
        return true;
    }

    // check if transaction + bip32 indices fit into the buffer of the device
    let total_size = LedgerBIP32Index::default().packed_len() * prepared_transaction.inputs_data.len()
        + prepared_transaction.transaction.packed_len();

    // return true if too large
    total_size > buffer_size
}

impl LedgerSecretManager {
    /// Creates a [`LedgerSecretManager`].
    ///
    /// To use a Ledger Speculos simulator, pass `true` to `is_simulator`; `false` otherwise.
    pub fn new(is_simulator: bool) -> Self {
        Self {
            is_simulator,
            non_interactive: false,
            mutex: Mutex::new(()),
        }
    }

    /// Get Ledger hardware status.
    pub async fn get_ledger_nano_status(&self) -> LedgerNanoStatus {
        log::debug!("get_ledger_nano_status");
        // lock the mutex
        let _lock = self.mutex.lock().await;
        let transport_type = if self.is_simulator {
            TransportTypes::TCP
        } else {
            TransportTypes::NativeHID
        };

        log::debug!("get_opened_app");
        let app = match get_opened_app(&transport_type) {
            Ok((name, version)) => Some(LedgerApp { name, version }),
            _ => None,
        };

        log::debug!("get_app_config");
        // if IOTA or Shimmer app is opened, the call will always succeed, returning information like
        // device, debug-flag, version number, lock-state but here we only are interested in a
        // successful call and the locked-flag
        let (connected_, locked, blind_signing_enabled, device) =
            get_app_config(&transport_type).map_or((false, None, false, None), |config| {
                (
                    true,
                    // locked flag
                    Some(config.flags & (1 << 0) != 0),
                    // blind signing enabled flag
                    config.flags & (1 << 1) != 0,
                    LedgerDeviceType::try_from(config.device).ok(),
                )
            });

        log::debug!("get_buffer_size");
        // get buffer size of connected device
        let buffer_size = get_buffer_size(&transport_type).ok();

        // We get the app info also if not the iota app is open, but another one
        // connected_ is in this case false, even tough the ledger is connected, that's why we always return true if we
        // got the app
        let connected = if app.is_some() { true } else { connected_ };
        LedgerNanoStatus {
            connected,
            locked,
            blind_signing_enabled,
            app,
            device,
            buffer_size,
        }
    }
}

// Merge signature unlocks with Account/Nft/Reference unlocks
fn merge_unlocks(
    prepared_transaction_data: &PreparedTransactionData,
    mut unlocks: impl Iterator<Item = Unlock>,
    protocol_parameters: &ProtocolParameters,
) -> Result<Vec<Unlock>, Error> {
    let slot_index = prepared_transaction_data
        .transaction
        .context_inputs()
        .iter()
        .find_map(|c| c.as_commitment_opt().map(|c| c.slot_index()));
    let transaction_signing_hash = prepared_transaction_data.transaction.signing_hash();

    let mut merged_unlocks = Vec::new();
    let mut block_indexes = HashMap::<Address, usize>::new();

    // Assuming inputs_data is ordered by address type
    for (current_block_index, input) in prepared_transaction_data.inputs_data.iter().enumerate() {
        // Get the address that is required to unlock the input
        let required_address = input
            .output
            .required_address(slot_index, protocol_parameters.committable_age_range())?
            // Time in which no address can unlock the output because of an expiration unlock condition
            .ok_or(Error::ExpirationDeadzone)?;

        let required_address = if let Address::Restricted(restricted) = &required_address {
            restricted.address()
        } else {
            &required_address
        };

        // Check if we already added an [Unlock] for this address
        match block_indexes.get(required_address) {
            // If we already have an [Unlock] for this address, add a [Unlock] based on the address type
            Some(block_index) => match required_address {
                Address::Ed25519(_) | Address::ImplicitAccountCreation(_) => {
                    merged_unlocks.push(Unlock::Reference(ReferenceUnlock::new(*block_index as u16)?));
                }
                Address::Account(_) => merged_unlocks.push(Unlock::Account(AccountUnlock::new(*block_index as u16)?)),
                Address::Nft(_) => merged_unlocks.push(Unlock::Nft(NftUnlock::new(*block_index as u16)?)),
                _ => Err(BlockError::UnsupportedAddressKind(required_address.kind()))?,
            },
            None => {
                // We can only sign ed25519 addresses and block_indexes needs to contain the account or nft
                // address already at this point, because the reference index needs to be lower
                // than the current block index
                match &required_address {
                    Address::Ed25519(_) | Address::ImplicitAccountCreation(_) => {}
                    _ => Err(Error::MissingInputWithEd25519Address)?,
                }

                let unlock = unlocks.next().ok_or(Error::MissingInputWithEd25519Address)?;

                if let Unlock::Signature(signature_unlock) = &unlock {
                    let Signature::Ed25519(ed25519_signature) = signature_unlock.signature();
                    let ed25519_address = match required_address {
                        Address::Ed25519(ed25519_address) => ed25519_address,
                        _ => return Err(Error::MissingInputWithEd25519Address),
                    };
                    ed25519_signature.is_valid(transaction_signing_hash.as_ref(), ed25519_address)?;
                }

                merged_unlocks.push(unlock);

                // Add the ed25519 address to the block_indexes, so it gets referenced if further inputs have
                // the same address in their unlock condition
                block_indexes.insert(required_address.clone(), current_block_index);
            }
        }

        // When we have an account or Nft output, we will add their account or nft address to block_indexes,
        // because they can be used to unlock outputs via [Unlock::Account] or [Unlock::Nft],
        // that have the corresponding account or nft address in their unlock condition
        match &input.output {
            Output::Account(account_output) => block_indexes.insert(
                Address::Account(AccountAddress::new(
                    account_output.account_id_non_null(input.output_id()),
                )),
                current_block_index,
            ),
            Output::Nft(nft_output) => block_indexes.insert(
                Address::Nft(NftAddress::new(nft_output.nft_id_non_null(input.output_id()))),
                current_block_index,
            ),
            _ => None,
        };
    }
    Ok(merged_unlocks)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{
        client::{api::GetAddressesOptions, constants::IOTA_COIN_TYPE, secret::SecretManager},
        types::block::address::ToBech32Ext,
    };

    #[tokio::test]
    #[ignore = "requires ledger nano instance"]
    async fn ed25519_address() {
        let mut secret_manager = LedgerSecretManager::new(true);
        secret_manager.non_interactive = true;

        let addresses = SecretManager::LedgerNano(secret_manager)
            .generate_ed25519_addresses(
                GetAddressesOptions::default()
                    .with_coin_type(IOTA_COIN_TYPE)
                    .with_account_index(0)
                    .with_range(0..1),
            )
            .await
            .unwrap();

        assert_eq!(
            addresses[0].clone().to_bech32_unchecked("atoi").to_string(),
            "atoi1qqdnv60ryxynaeyu8paq3lp9rkll7d7d92vpumz88fdj4l0pn5mru50gvd8"
        );
    }
}
