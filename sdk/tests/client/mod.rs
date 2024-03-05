// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod addresses;
mod client_builder;
mod common;
mod error;
mod high_level;
mod input_signing_data;
mod mnemonic;
#[cfg(feature = "mqtt")]
mod mqtt;
mod node_api;
mod secret_manager;
mod signing;
mod transaction_builder;

use std::{collections::HashMap, hash::Hash, str::FromStr};

use crypto::keys::bip44::Bip44;
use iota_sdk::{
    client::secret::types::InputSigningData,
    types::block::{
        address::{AccountAddress, Address},
        output::{
            feature::{IssuerFeature, SenderFeature},
            unlock_condition::{
                AddressUnlockCondition, ExpirationUnlockCondition, ImmutableAccountAddressUnlockCondition,
                StorageDepositReturnUnlockCondition, TimelockUnlockCondition, UnlockCondition,
            },
            AccountId, AccountOutputBuilder, BasicOutputBuilder, DelegationId, DelegationOutputBuilder, Feature,
            FoundryOutputBuilder, NativeToken, NftId, NftOutputBuilder, Output, OutputId, SimpleTokenScheme, TokenId,
            TokenScheme,
        },
        rand::{
            output::rand_output_metadata_with_id,
            transaction::{rand_transaction_id, rand_transaction_id_with_slot_index},
        },
        slot::{SlotCommitmentHash, SlotCommitmentId, SlotIndex},
    },
};

const ACCOUNT_ID_0: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";
const ACCOUNT_ID_1: &str = "0x1111111111111111111111111111111111111111111111111111111111111111";
const ACCOUNT_ID_2: &str = "0x2222222222222222222222222222222222222222222222222222222222222222";
const NFT_ID_0: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";
const NFT_ID_1: &str = "0x1111111111111111111111111111111111111111111111111111111111111111";
const NFT_ID_2: &str = "0x2222222222222222222222222222222222222222222222222222222222222222";
const NFT_ID_3: &str = "0x3333333333333333333333333333333333333333333333333333333333333333";
const NFT_ID_4: &str = "0x4444444444444444444444444444444444444444444444444444444444444444";
const TOKEN_ID_1: &str = "0x1111111111111111111111111111111111111111111111111111111111111111111111111111";
const TOKEN_ID_2: &str = "0x2222222222222222222222222222222222222222222222222222222222222222222222222222";
const BECH32_ADDRESS_REMAINDER: &str = "rms1qrut5ajyfrtgjs325kd9chwfwyyy2z3fewy4vgy0vvdtf2pr8prg5u3zwjn";
const BECH32_ADDRESS_ED25519_0: &str = "rms1qr2xsmt3v3eyp2ja80wd2sq8xx0fslefmxguf7tshzezzr5qsctzc2f5dg6";
const BECH32_ADDRESS_ED25519_1: &str = "rms1qqhvvur9xfj6yhgsxfa4f8xst7vz9zxeu3vcxds8mh4a6jlpteq9xrajhtf";
const BECH32_ADDRESS_ED25519_2: &str = "rms1qr47gz3xxjqpjrwd0yu5glhqrth6w0t08npney8000ust2lcw2r92j5a8rt";
const BECH32_ADDRESS_ACCOUNT_1: &str = "rms1pqg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zws5524"; // Corresponds to ACCOUNT_ID_1
const _BECH32_ADDRESS_ACCOUNT_2: &str = "rms1pq3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zymxrh9z"; // Corresponds to ACCOUNT_ID_2
const BECH32_ADDRESS_NFT_1: &str = "rms1zqg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zxddmy7"; // Corresponds to NFT_ID_1
const _BECH32_ADDRESS_NFT_2: &str = "rms1zq3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zynm6ctf"; // Corresponds to NFT_ID_2
const SLOT_INDEX: SlotIndex = SlotIndex(10);
const SLOT_COMMITMENT_ID: SlotCommitmentId = SlotCommitmentHash::null().const_into_slot_commitment_id(SlotIndex(9));

#[derive(Debug, Clone)]
enum Build<'a> {
    Basic {
        amount: u64,
        address: Address,
        native_token: Option<(&'a str, u64)>,
        sender: Option<Address>,
        sdruc: Option<(Address, u64)>,
        timelock: Option<u32>,
        expiration: Option<(Address, u32)>,
    },
    Nft {
        amount: u64,
        nft_id: NftId,
        address: Address,
        sender: Option<Address>,
        issuer: Option<Address>,
        sdruc: Option<(Address, u64)>,
        expiration: Option<(Address, u32)>,
    },
    Account {
        amount: u64,
        account_id: AccountId,
        address: Address,
        sender: Option<Address>,
        issuer: Option<Address>,
    },
    Foundry {
        amount: u64,
        account_id: AccountId,
        serial_number: u32,
        token_scheme: SimpleTokenScheme,
        native_token: Option<(&'a str, u64)>,
    },
    Delegation {
        amount: u64,
        delegation_amount: u64,
        delegation_id: DelegationId,
        address: Address,
        validator_address: AccountAddress,
        start_epoch: u32,
        end_epoch: u32,
    },
}

impl<'a> Build<'a> {
    fn build(self) -> Output {
        match self {
            Build::Basic {
                amount,
                address,
                native_token,
                sender,
                sdruc,
                timelock,
                expiration,
            } => {
                let mut builder = BasicOutputBuilder::new_with_amount(amount)
                    .add_unlock_condition(AddressUnlockCondition::new(address.clone()));

                if let Some((id, amount)) = native_token {
                    builder =
                        builder.with_native_token(NativeToken::new(TokenId::from_str(id).unwrap(), amount).unwrap());
                }

                if let Some(sender) = sender {
                    builder = builder.add_feature(SenderFeature::new(sender.clone()));
                }

                if let Some((address, amount)) = sdruc {
                    builder = builder.add_unlock_condition(
                        StorageDepositReturnUnlockCondition::new(address.clone(), amount).unwrap(),
                    );
                }

                if let Some(timelock) = timelock {
                    builder = builder.add_unlock_condition(TimelockUnlockCondition::new(timelock).unwrap());
                }

                if let Some((address, timestamp)) = expiration {
                    builder = builder
                        .add_unlock_condition(ExpirationUnlockCondition::new(address.clone(), timestamp).unwrap());
                }

                builder.finish_output().unwrap()
            }
            Build::Nft {
                amount,
                nft_id,
                address,
                sender,
                issuer,
                sdruc,
                expiration,
            } => {
                let mut builder = NftOutputBuilder::new_with_amount(amount, nft_id)
                    .add_unlock_condition(AddressUnlockCondition::new(address));

                if let Some(sender) = sender {
                    builder = builder.add_feature(SenderFeature::new(sender));
                }

                if let Some(issuer) = issuer {
                    builder = builder.add_immutable_feature(IssuerFeature::new(issuer));
                }

                if let Some((address, amount)) = sdruc {
                    builder = builder
                        .add_unlock_condition(StorageDepositReturnUnlockCondition::new(address, amount).unwrap());
                }

                if let Some((address, timestamp)) = expiration {
                    builder = builder.add_unlock_condition(ExpirationUnlockCondition::new(address, timestamp).unwrap());
                }

                builder.finish_output().unwrap()
            }
            Build::Account {
                amount,
                account_id,
                address,
                sender,
                issuer,
            } => {
                let mut builder = AccountOutputBuilder::new_with_amount(amount, account_id)
                    .add_unlock_condition(AddressUnlockCondition::new(address));

                if let Some(sender) = sender {
                    builder = builder.add_feature(SenderFeature::new(sender));
                }

                if let Some(issuer) = issuer {
                    builder = builder.add_immutable_feature(IssuerFeature::new(issuer));
                }

                builder.finish_output().unwrap()
            }
            Build::Foundry {
                amount,
                account_id,
                serial_number,
                token_scheme,
                native_token,
            } => {
                let mut builder =
                    FoundryOutputBuilder::new_with_amount(amount, serial_number, TokenScheme::Simple(token_scheme))
                        .add_unlock_condition(ImmutableAccountAddressUnlockCondition::new(AccountAddress::new(
                            account_id,
                        )));

                if let Some((id, amount)) = native_token {
                    builder =
                        builder.with_native_token(NativeToken::new(TokenId::from_str(id).unwrap(), amount).unwrap());
                }

                builder.finish_output().unwrap()
            }
            Build::Delegation {
                amount,
                delegation_amount,
                delegation_id,
                address,
                validator_address,
                start_epoch,
                end_epoch,
            } => DelegationOutputBuilder::new_with_amount(delegation_amount, delegation_id, validator_address)
                .with_amount(amount)
                .add_unlock_condition(AddressUnlockCondition::new(address))
                .with_start_epoch(start_epoch)
                .with_end_epoch(end_epoch)
                .finish_output()
                .unwrap(),
        }
    }
}

fn build_inputs<'a>(
    outputs: impl IntoIterator<Item = (Build<'a>, Option<Bip44>)>,
    slot_index: Option<SlotIndex>,
) -> Vec<InputSigningData> {
    outputs
        .into_iter()
        .map(|(build, chain)| {
            let output = build.build();
            let transaction_id = slot_index.map_or_else(rand_transaction_id, rand_transaction_id_with_slot_index);

            InputSigningData {
                output,
                output_metadata: rand_output_metadata_with_id(OutputId::new(transaction_id, 0)),
                chain,
            }
        })
        .collect()
}

fn build_outputs<'a>(outputs: impl IntoIterator<Item = Build<'a>>) -> Vec<Output> {
    outputs.into_iter().map(|build| build.build()).collect()
}

fn unsorted_eq<T>(a: &[T], b: &[T]) -> bool
where
    T: Eq + Hash,
{
    fn count<T>(items: &[T]) -> HashMap<&T, usize>
    where
        T: Eq + Hash,
    {
        let mut cnt = HashMap::new();
        for i in items {
            *cnt.entry(i).or_insert(0) += 1
        }
        cnt
    }

    count(a) == count(b)
}

fn assert_remainder_or_return(output: &Output, amount: u64, address: Address, native_token: Option<(&str, u64)>) {
    let output = output.as_basic();
    assert_eq!(amount, output.amount());

    if let [UnlockCondition::Address(address_unlock_condition)] = output.unlock_conditions().as_ref() {
        assert_eq!(&address, address_unlock_condition.address());
    } else {
        panic!("no address unlock condition");
    }

    match output.features().as_ref() {
        [] | [Feature::NativeToken(_)] => {}
        _ => panic!("incorrect features"),
    }

    if let Some((token_id, amount)) = native_token {
        let native_token = NativeToken::new(TokenId::from_str(token_id).unwrap(), amount).unwrap();

        assert_eq!(&native_token, output.native_token().unwrap());
    } else if output.native_token().is_some() {
        panic!("no native token provided but native token exists on output");
    }
}
