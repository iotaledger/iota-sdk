// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod addresses;
mod client_builder;
mod common;
mod consolidation;
mod error;
mod input_selection;
mod input_signing_data;
mod mnemonic;
#[cfg(feature = "mqtt")]
mod mqtt;
mod node_api;
mod secret_manager;
mod signing;
mod transactions;

use std::{
    collections::{BTreeSet, HashMap},
    hash::Hash,
    str::FromStr,
};

use crypto::keys::bip44::Bip44;
use iota_sdk::{
    client::secret::types::InputSigningData,
    types::block::{
        address::{Address, AliasAddress, Bech32Address},
        output::{
            feature::{IssuerFeature, SenderFeature},
            unlock_condition::{
                AddressUnlockCondition, ExpirationUnlockCondition, GovernorAddressUnlockCondition,
                ImmutableAliasAddressUnlockCondition, StateControllerAddressUnlockCondition,
                StorageDepositReturnUnlockCondition, TimelockUnlockCondition, UnlockCondition,
            },
            AliasId, AliasOutputBuilder, BasicOutputBuilder, FoundryOutputBuilder, NativeToken, NativeTokens, NftId,
            NftOutputBuilder, Output, OutputId, OutputMetadata, SimpleTokenScheme, TokenId, TokenScheme,
        },
        rand::{block::rand_block_id, transaction::rand_transaction_id},
    },
};

const TOKEN_SUPPLY: u64 = 1_813_620_509_061_365;
const ALIAS_ID_0: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";
const ALIAS_ID_1: &str = "0x1111111111111111111111111111111111111111111111111111111111111111";
const ALIAS_ID_2: &str = "0x2222222222222222222222222222222222222222222222222222222222222222";
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
const BECH32_ADDRESS_ALIAS_1: &str = "rms1pqg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zws5524"; // Corresponds to ALIAS_ID_1
const BECH32_ADDRESS_ALIAS_2: &str = "rms1pq3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zymxrh9z"; // Corresponds to ALIAS_ID_2
const BECH32_ADDRESS_NFT_1: &str = "rms1zqg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zxddmy7"; // Corresponds to NFT_ID_1
const _BECH32_ADDRESS_NFT_2: &str = "rms1zq3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zynm6ctf"; // Corresponds to NFT_ID_2

#[derive(Debug, Clone)]
enum Build<'a> {
    Basic(
        u64,
        &'a str,
        Option<Vec<(&'a str, u64)>>,
        Option<&'a str>,
        Option<(&'a str, u64)>,
        Option<u32>,
        Option<(&'a str, u32)>,
        Option<Bip44>,
    ),
    Nft(
        u64,
        NftId,
        &'a str,
        Option<Vec<(&'a str, u64)>>,
        Option<&'a str>,
        Option<&'a str>,
        Option<(&'a str, u64)>,
        Option<(&'a str, u32)>,
        Option<Bip44>,
    ),
    Alias(
        u64,
        AliasId,
        u32,
        &'a str,
        &'a str,
        Option<Vec<(&'a str, u64)>>,
        Option<&'a str>,
        Option<&'a str>,
        Option<Bip44>,
    ),
    Foundry(u64, AliasId, u32, SimpleTokenScheme, Option<Vec<(&'a str, u64)>>),
}

fn build_basic_output(
    amount: u64,
    bech32_address: Bech32Address,
    native_tokens: Option<Vec<(&str, u64)>>,
    bech32_sender: Option<Bech32Address>,
    sdruc: Option<(Bech32Address, u64)>,
    timelock: Option<u32>,
    expiration: Option<(Bech32Address, u32)>,
) -> Output {
    let mut builder =
        BasicOutputBuilder::new_with_amount(amount).add_unlock_condition(AddressUnlockCondition::new(bech32_address));

    if let Some(native_tokens) = native_tokens {
        builder = builder.with_native_tokens(
            native_tokens
                .into_iter()
                .map(|(id, amount)| NativeToken::new(TokenId::from_str(id).unwrap(), amount).unwrap()),
        );
    }

    if let Some(bech32_sender) = bech32_sender {
        builder = builder.add_feature(SenderFeature::new(bech32_sender));
    }

    if let Some((address, amount)) = sdruc {
        builder = builder
            .add_unlock_condition(StorageDepositReturnUnlockCondition::new(address, amount, TOKEN_SUPPLY).unwrap());
    }

    if let Some(timelock) = timelock {
        builder = builder.add_unlock_condition(TimelockUnlockCondition::new(timelock).unwrap());
    }

    if let Some((address, timestamp)) = expiration {
        builder = builder.add_unlock_condition(ExpirationUnlockCondition::new(address, timestamp).unwrap());
    }

    builder.finish_output(TOKEN_SUPPLY).unwrap()
}

#[allow(clippy::too_many_arguments)]
fn build_nft_output(
    amount: u64,
    nft_id: NftId,
    bech32_address: Bech32Address,
    native_tokens: Option<Vec<(&str, u64)>>,
    bech32_sender: Option<Bech32Address>,
    bech32_issuer: Option<Bech32Address>,
    sdruc: Option<(Bech32Address, u64)>,
    expiration: Option<(Bech32Address, u32)>,
) -> Output {
    let mut builder = NftOutputBuilder::new_with_amount(amount, nft_id)
        .add_unlock_condition(AddressUnlockCondition::new(bech32_address));

    if let Some(native_tokens) = native_tokens {
        builder = builder.with_native_tokens(
            native_tokens
                .into_iter()
                .map(|(id, amount)| NativeToken::new(TokenId::from_str(id).unwrap(), amount).unwrap()),
        );
    }

    if let Some(bech32_sender) = bech32_sender {
        builder = builder.add_feature(SenderFeature::new(bech32_sender));
    }

    if let Some(bech32_issuer) = bech32_issuer {
        builder = builder.add_immutable_feature(IssuerFeature::new(bech32_issuer));
    }

    if let Some((address, amount)) = sdruc {
        builder = builder
            .add_unlock_condition(StorageDepositReturnUnlockCondition::new(address, amount, TOKEN_SUPPLY).unwrap());
    }

    if let Some((address, timestamp)) = expiration {
        builder = builder.add_unlock_condition(ExpirationUnlockCondition::new(address, timestamp).unwrap());
    }

    builder.finish_output(TOKEN_SUPPLY).unwrap()
}

#[allow(clippy::too_many_arguments)]
fn build_alias_output(
    amount: u64,
    alias_id: AliasId,
    state_index: u32,
    state_address: Bech32Address,
    governor_address: Bech32Address,
    native_tokens: Option<Vec<(&str, u64)>>,
    bech32_sender: Option<Bech32Address>,
    bech32_issuer: Option<Bech32Address>,
) -> Output {
    let mut builder = AliasOutputBuilder::new_with_amount(amount, alias_id)
        .with_state_index(state_index)
        .add_unlock_condition(StateControllerAddressUnlockCondition::new(state_address))
        .add_unlock_condition(GovernorAddressUnlockCondition::new(governor_address));

    if let Some(native_tokens) = native_tokens {
        builder = builder.with_native_tokens(
            native_tokens
                .into_iter()
                .map(|(id, amount)| NativeToken::new(TokenId::from_str(id).unwrap(), amount).unwrap()),
        );
    }

    if let Some(bech32_sender) = bech32_sender {
        builder = builder.add_feature(SenderFeature::new(bech32_sender));
    }

    if let Some(bech32_issuer) = bech32_issuer {
        builder = builder.add_immutable_feature(IssuerFeature::new(bech32_issuer));
    }

    builder.finish_output(TOKEN_SUPPLY).unwrap()
}

fn build_foundry_output(
    amount: u64,
    alias_id: AliasId,
    serial_number: u32,
    token_scheme: SimpleTokenScheme,
    native_tokens: Option<Vec<(&str, u64)>>,
) -> Output {
    let mut builder = FoundryOutputBuilder::new_with_amount(amount, serial_number, TokenScheme::Simple(token_scheme))
        .add_unlock_condition(ImmutableAliasAddressUnlockCondition::new(AliasAddress::new(alias_id)));

    if let Some(native_tokens) = native_tokens {
        builder = builder.with_native_tokens(
            native_tokens
                .into_iter()
                .map(|(id, amount)| NativeToken::new(TokenId::from_str(id).unwrap(), amount).unwrap()),
        );
    }

    builder.finish_output(TOKEN_SUPPLY).unwrap()
}

fn build_output_inner(build: Build) -> (Output, Option<Bip44>) {
    match build {
        Build::Basic(amount, bech32_address, native_tokens, bech32_sender, sdruc, timelock, expiration, chain) => (
            build_basic_output(
                amount,
                Bech32Address::try_from_str(bech32_address).unwrap(),
                native_tokens,
                bech32_sender.map(|address| Bech32Address::try_from_str(address).unwrap()),
                sdruc.map(|(address, exp)| (Bech32Address::try_from_str(address).unwrap(), exp)),
                timelock,
                expiration.map(|(address, exp)| (Bech32Address::try_from_str(address).unwrap(), exp)),
            ),
            chain,
        ),
        Build::Nft(
            amount,
            nft_id,
            bech32_address,
            native_tokens,
            bech32_sender,
            bech32_issuer,
            sdruc,
            expiration,
            chain,
        ) => (
            build_nft_output(
                amount,
                nft_id,
                Bech32Address::try_from_str(bech32_address).unwrap(),
                native_tokens,
                bech32_sender.map(|address| Bech32Address::try_from_str(address).unwrap()),
                bech32_issuer.map(|address| Bech32Address::try_from_str(address).unwrap()),
                sdruc.map(|(address, exp)| (Bech32Address::try_from_str(address).unwrap(), exp)),
                expiration.map(|(address, exp)| (Bech32Address::try_from_str(address).unwrap(), exp)),
            ),
            chain,
        ),
        Build::Alias(
            amount,
            alias_id,
            state_index,
            state_address,
            governor_address,
            native_tokens,
            bech32_sender,
            bech32_issuer,
            chain,
        ) => (
            build_alias_output(
                amount,
                alias_id,
                state_index,
                Bech32Address::try_from_str(state_address).unwrap(),
                Bech32Address::try_from_str(governor_address).unwrap(),
                native_tokens,
                bech32_sender.map(|address| Bech32Address::try_from_str(address).unwrap()),
                bech32_issuer.map(|address| Bech32Address::try_from_str(address).unwrap()),
            ),
            chain,
        ),
        Build::Foundry(amount, alias_id, serial_number, token_scheme, native_tokens) => (
            build_foundry_output(amount, alias_id, serial_number, token_scheme, native_tokens),
            None,
        ),
    }
}

fn build_inputs<'a>(outputs: impl IntoIterator<Item = Build<'a>>) -> Vec<InputSigningData> {
    outputs
        .into_iter()
        .map(|build| {
            let (output, chain) = build_output_inner(build);

            InputSigningData {
                output,
                output_metadata: OutputMetadata::new(
                    rand_block_id(),
                    OutputId::new(rand_transaction_id(), 0).unwrap(),
                    false,
                    None,
                    None,
                    None,
                    0,
                    0,
                    0,
                ),
                chain,
            }
        })
        .collect()
}

fn build_outputs<'a>(outputs: impl IntoIterator<Item = Build<'a>>) -> Vec<Output> {
    outputs.into_iter().map(|build| build_output_inner(build).0).collect()
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

fn is_remainder_or_return(
    output: &Output,
    amount: u64,
    address: &str,
    native_tokens: Option<Vec<(&str, u64)>>,
) -> bool {
    if let Output::Basic(output) = output {
        if output.amount() != amount {
            return false;
        }

        // assert_eq!(output.as_basic().native_tokens().len(), 0);

        if let [UnlockCondition::Address(address_unlock_condition)] = output.unlock_conditions().as_ref() {
            if address_unlock_condition.address() != Bech32Address::try_from_str(address).unwrap().inner() {
                return false;
            }
        } else {
            return false;
        }

        if output.features().len() != 0 {
            return false;
        }

        if let Some(native_tokens) = native_tokens {
            let native_tokens = NativeTokens::from_set(
                native_tokens
                    .into_iter()
                    .map(|(token_id, amount)| NativeToken::new(TokenId::from_str(token_id).unwrap(), amount).unwrap())
                    .collect::<BTreeSet<_>>(),
            )
            .unwrap();

            if output.native_tokens() != &native_tokens {
                return false;
            }
        } else if output.native_tokens().len() != 0 {
            return false;
        }

        true
    } else {
        false
    }
}

fn addresses<'a>(addresses: impl IntoIterator<Item = &'a str>) -> Vec<Address> {
    addresses
        .into_iter()
        .map(|address| Address::try_from_bech32(address).unwrap())
        .collect()
}
