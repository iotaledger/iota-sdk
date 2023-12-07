// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod addresses;
mod client_builder;
mod common;
mod error;
mod high_level;
mod input_selection;
mod input_signing_data;
mod mnemonic;
#[cfg(feature = "mqtt")]
mod mqtt;
mod node_api;
mod secret_manager;
mod signing;

use std::{collections::HashMap, hash::Hash, str::FromStr};

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
            AccountId, AccountOutputBuilder, BasicOutputBuilder, FoundryOutputBuilder, NativeToken, NftId,
            NftOutputBuilder, Output, OutputId, SimpleTokenScheme, TokenId, TokenScheme,
        },
        rand::{output::rand_output_metadata_with_id, transaction::rand_transaction_id},
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
const BECH32_ADDRESS_ACCOUNT_2: &str = "rms1pq3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zymxrh9z"; // Corresponds to ACCOUNT_ID_2
const BECH32_ADDRESS_NFT_1: &str = "rms1zqg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zxddmy7"; // Corresponds to NFT_ID_1
const _BECH32_ADDRESS_NFT_2: &str = "rms1zq3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zynm6ctf"; // Corresponds to NFT_ID_2

#[derive(Debug, Clone)]
enum Build<'a> {
    Basic(
        u64,
        Address,
        Option<(&'a str, u64)>,
        Option<Address>,
        Option<(Address, u64)>,
        Option<u32>,
        Option<(Address, u32)>,
    ),
    Nft(
        u64,
        NftId,
        Address,
        Option<Address>,
        Option<Address>,
        Option<(Address, u64)>,
        Option<(Address, u32)>,
    ),
    Account(u64, AccountId, Address, Option<Address>, Option<Address>),
    Foundry(u64, AccountId, u32, SimpleTokenScheme, Option<(&'a str, u64)>),
}

fn build_basic_output(
    amount: u64,
    address: Address,
    native_token: Option<(&str, u64)>,
    sender: Option<Address>,
    sdruc: Option<(Address, u64)>,
    timelock: Option<u32>,
    expiration: Option<(Address, u32)>,
) -> Output {
    let mut builder =
        BasicOutputBuilder::new_with_amount(amount).add_unlock_condition(AddressUnlockCondition::new(address.clone()));

    if let Some((id, amount)) = native_token {
        builder = builder.with_native_token(NativeToken::new(TokenId::from_str(id).unwrap(), amount).unwrap());
    }

    if let Some(sender) = sender {
        builder = builder.add_feature(SenderFeature::new(sender.clone()));
    }

    if let Some((address, amount)) = sdruc {
        builder =
            builder.add_unlock_condition(StorageDepositReturnUnlockCondition::new(address.clone(), amount).unwrap());
    }

    if let Some(timelock) = timelock {
        builder = builder.add_unlock_condition(TimelockUnlockCondition::new(timelock).unwrap());
    }

    if let Some((address, timestamp)) = expiration {
        builder = builder.add_unlock_condition(ExpirationUnlockCondition::new(address.clone(), timestamp).unwrap());
    }

    builder.finish_output().unwrap()
}

fn build_nft_output(
    amount: u64,
    nft_id: NftId,
    address: Address,
    sender: Option<Address>,
    issuer: Option<Address>,
    sdruc: Option<(Address, u64)>,
    expiration: Option<(Address, u32)>,
) -> Output {
    let mut builder = NftOutputBuilder::new_with_amount(amount, nft_id)
        .add_unlock_condition(AddressUnlockCondition::new(address.clone()));

    if let Some(sender) = sender {
        builder = builder.add_feature(SenderFeature::new(sender.clone()));
    }

    if let Some(issuer) = issuer {
        builder = builder.add_immutable_feature(IssuerFeature::new(issuer.clone()));
    }

    if let Some((address, amount)) = sdruc {
        builder =
            builder.add_unlock_condition(StorageDepositReturnUnlockCondition::new(address.clone(), amount).unwrap());
    }

    if let Some((address, timestamp)) = expiration {
        builder = builder.add_unlock_condition(ExpirationUnlockCondition::new(address.clone(), timestamp).unwrap());
    }

    builder.finish_output().unwrap()
}

fn build_account_output(
    amount: u64,
    account_id: AccountId,
    address: Address,
    sender: Option<Address>,
    issuer: Option<Address>,
) -> Output {
    let mut builder = AccountOutputBuilder::new_with_amount(amount, account_id)
        .add_unlock_condition(AddressUnlockCondition::new(address.clone()));

    if let Some(sender) = sender {
        builder = builder.add_feature(SenderFeature::new(sender.clone()));
    }

    if let Some(issuer) = issuer {
        builder = builder.add_immutable_feature(IssuerFeature::new(issuer.clone()));
    }

    builder.finish_output().unwrap()
}

fn build_foundry_output(
    amount: u64,
    account_id: AccountId,
    serial_number: u32,
    token_scheme: SimpleTokenScheme,
    native_token: Option<(&str, u64)>,
) -> Output {
    let mut builder = FoundryOutputBuilder::new_with_amount(amount, serial_number, TokenScheme::Simple(token_scheme))
        .add_unlock_condition(ImmutableAccountAddressUnlockCondition::new(AccountAddress::new(
            account_id,
        )));

    if let Some((id, amount)) = native_token {
        builder = builder.with_native_token(NativeToken::new(TokenId::from_str(id).unwrap(), amount).unwrap());
    }

    builder.finish_output().unwrap()
}

fn build_output_inner(build: Build) -> Output {
    match build {
        Build::Basic(amount, address, native_token, sender, sdruc, timelock, expiration) => {
            build_basic_output(amount, address, native_token, sender, sdruc, timelock, expiration)
        }
        Build::Nft(amount, nft_id, address, sender, issuer, sdruc, expiration) => {
            build_nft_output(amount, nft_id, address, sender, issuer, sdruc, expiration)
        }
        Build::Account(amount, account_id, address, sender, issuer) => {
            build_account_output(amount, account_id, address, sender, issuer)
        }
        Build::Foundry(amount, account_id, serial_number, token_scheme, native_token) => {
            build_foundry_output(amount, account_id, serial_number, token_scheme, native_token)
        }
    }
}

fn build_inputs<'a>(outputs: impl IntoIterator<Item = Build<'a>>) -> Vec<InputSigningData> {
    outputs
        .into_iter()
        .map(|build| {
            let output = build_output_inner(build);

            InputSigningData {
                output,
                output_metadata: rand_output_metadata_with_id(OutputId::new(rand_transaction_id(), 0)),
            }
        })
        .collect()
}

fn build_outputs<'a>(outputs: impl IntoIterator<Item = Build<'a>>) -> Vec<Output> {
    outputs.into_iter().map(|build| build_output_inner(build)).collect()
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

fn is_remainder_or_return(output: &Output, amount: u64, address: Address, native_token: Option<(&str, u64)>) -> bool {
    if let Output::Basic(output) = output {
        if output.amount() != amount {
            return false;
        }

        // assert_eq!(output.as_basic().native_tokens().len(), 0);

        if let [UnlockCondition::Address(address_unlock_condition)] = output.unlock_conditions().as_ref() {
            if address_unlock_condition.address() != &address {
                return false;
            }
        } else {
            return false;
        }

        if output.features().len() != 0 {
            return false;
        }

        if let Some((token_id, amount)) = native_token {
            let native_token = NativeToken::new(TokenId::from_str(token_id).unwrap(), amount).unwrap();

            if output.native_token().unwrap() != &native_token {
                return false;
            }
        } else if output.native_token().is_some() {
            return false;
        }

        true
    } else {
        false
    }
}
