// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create all output types in a single transaction.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example all_automatic_input_selection
//! ```

use iota_sdk::{
    client::{api::GetAddressesOptions, request_funds_from_faucet, secret::SecretManager, Client, Result},
    types::block::{
        address::AliasAddress,
        output::{
            feature::{IssuerFeature, MetadataFeature, SenderFeature},
            unlock_condition::{
                AddressUnlockCondition, ExpirationUnlockCondition, GovernorAddressUnlockCondition,
                ImmutableAliasAddressUnlockCondition, StateControllerAddressUnlockCondition,
                StorageDepositReturnUnlockCondition, TimelockUnlockCondition,
            },
            AliasId, AliasOutputBuilder, BasicOutputBuilder, FoundryId, FoundryOutputBuilder, NativeToken, NftId,
            NftOutputBuilder, Output, OutputId, SimpleTokenScheme, TokenId, TokenScheme,
        },
        payload::{transaction::TransactionEssence, Payload},
    },
};
use primitive_types::U256;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    // Configure your own mnemonic in the ".env" file. Since the output amount cannot be zero, the seed must contain
    // non-zero balance.
    dotenvy::dotenv().ok();

    // Create a node client.
    let client = Client::builder()
        .with_node(&std::env::var("NODE_URL").unwrap())?
        .finish()
        .await?;

    let secret_manager =
        SecretManager::try_from_mnemonic(std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    let token_supply = client.get_token_supply().await?;

    let address = secret_manager
        .generate_ed25519_addresses(GetAddressesOptions::from_client(&client).await?.with_range(0..1))
        .await?[0];

    println!(
        "Requesting funds (waiting 15s): {}",
        request_funds_from_faucet(&std::env::var("FAUCET_URL").unwrap(), &address).await?,
    );
    tokio::time::sleep(std::time::Duration::from_secs(15)).await;

    //////////////////////////////////
    // create new alias and nft output
    //////////////////////////////////
    let alias_output_builder = AliasOutputBuilder::new_with_amount(2_000_000, AliasId::null())
        .add_feature(SenderFeature::new(address))
        .add_feature(MetadataFeature::new([1, 2, 3])?)
        .add_immutable_feature(IssuerFeature::new(address))
        .add_unlock_condition(StateControllerAddressUnlockCondition::new(address))
        .add_unlock_condition(GovernorAddressUnlockCondition::new(address));

    // address of the owner of the NFT
    let nft_output_builder = NftOutputBuilder::new_with_amount(1_000_000, NftId::null())
        .add_unlock_condition(AddressUnlockCondition::new(address));

    let outputs = [
        alias_output_builder.clone().finish_output(token_supply)?,
        nft_output_builder
            .clone()
            // address of the minter of the NFT
            // .add_feature(IssuerFeature::new(address))
            .finish_output(token_supply)?,
    ];

    let block = client
        .build_block()
        .with_secret_manager(&secret_manager)
        .with_outputs(outputs)?
        .finish()
        .await?;

    println!(
        "Block with new nft and alias output sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id()
    );
    let _ = client.retry_until_included(&block.id(), None, None).await?;

    //////////////////////////////////
    // create foundry output, native tokens and nft
    //////////////////////////////////
    let alias_output_id_1 = get_alias_output_id(block.payload().unwrap())?;
    let alias_id = AliasId::from(&alias_output_id_1);
    let nft_output_id = get_nft_output_id(block.payload().unwrap())?;
    let nft_id = NftId::from(&nft_output_id);
    let token_scheme = TokenScheme::Simple(SimpleTokenScheme::new(U256::from(50), U256::from(0), U256::from(100))?);
    let foundry_id = FoundryId::build(
        &AliasAddress::from(AliasId::from(&alias_output_id_1)),
        1,
        token_scheme.kind(),
    );
    let token_id = TokenId::from(foundry_id);

    let foundry_output_builder = FoundryOutputBuilder::new_with_amount(1_000_000, 1, token_scheme)
        .add_unlock_condition(ImmutableAliasAddressUnlockCondition::new(AliasAddress::from(alias_id)));

    let outputs = [
        alias_output_builder
            .clone()
            .with_amount(1_000_000)
            .with_alias_id(alias_id)
            .with_state_index(1)
            .with_foundry_counter(1)
            .finish_output(token_supply)?,
        foundry_output_builder
            .clone()
            // Mint native tokens
            .add_native_token(NativeToken::new(token_id, U256::from(50))?)
            .finish_output(token_supply)?,
        nft_output_builder
            .clone()
            .with_nft_id(nft_id)
            .finish_output(token_supply)?,
    ];

    let block = client
        .build_block()
        .with_secret_manager(&secret_manager)
        .with_outputs(outputs)?
        .finish()
        .await?;
    println!(
        "Block with alias id, foundry output with minted native tokens, and nfts sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id(),
    );
    let _ = client.retry_until_included(&block.id(), None, None).await?;

    //////////////////////////////////
    // create all outputs
    //////////////////////////////////

    let basic_output_builder =
        BasicOutputBuilder::new_with_amount(1_000_000).add_unlock_condition(AddressUnlockCondition::new(address));

    let outputs = [
        alias_output_builder
            .with_amount(1_000_000)
            .with_alias_id(alias_id)
            .with_state_index(2)
            .with_foundry_counter(1)
            .finish_output(token_supply)?,
        foundry_output_builder.finish_output(token_supply)?,
        nft_output_builder.with_nft_id(nft_id).finish_output(token_supply)?,
        // with native token
        basic_output_builder
            .clone()
            .add_native_token(NativeToken::new(token_id, U256::from(50))?)
            .finish_output(token_supply)?,
        // with most simple output
        basic_output_builder.clone().finish_output(token_supply)?,
        // with metadata feature block
        basic_output_builder
            .clone()
            .add_feature(MetadataFeature::new([13, 37])?)
            .finish_output(token_supply)?,
        // with storage deposit return
        basic_output_builder
            .clone()
            .with_amount(234_100)
            .add_unlock_condition(StorageDepositReturnUnlockCondition::new(
                address,
                234_000,
                token_supply,
            )?)
            .finish_output(token_supply)?,
        // with expiration
        basic_output_builder
            .clone()
            .add_unlock_condition(ExpirationUnlockCondition::new(address, 1)?)
            .finish_output(token_supply)?,
        // with timelock
        basic_output_builder
            .clone()
            .add_unlock_condition(TimelockUnlockCondition::new(1)?)
            .finish_output(token_supply)?,
    ];

    let block = client
        .build_block()
        .with_secret_manager(&secret_manager)
        .with_outputs(outputs)?
        .finish()
        .await?;
    println!(
        "Block with all outputs sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id()
    );
    let _ = client.retry_until_included(&block.id(), None, None).await?;

    Ok(())
}

// helper function to get the output id for the first alias output
fn get_alias_output_id(payload: &Payload) -> Result<OutputId> {
    match payload {
        Payload::Transaction(tx_payload) => {
            let TransactionEssence::Regular(regular) = tx_payload.essence();
            for (index, output) in regular.outputs().iter().enumerate() {
                if let Output::Alias(_alias_output) = output {
                    return Ok(OutputId::new(tx_payload.id(), index.try_into().unwrap())?);
                }
            }
            panic!("No alias output in transaction essence")
        }
        _ => panic!("No tx payload"),
    }
}

// helper function to get the output id for the first NFT output
fn get_nft_output_id(payload: &Payload) -> Result<OutputId> {
    match payload {
        Payload::Transaction(tx_payload) => {
            let TransactionEssence::Regular(regular) = tx_payload.essence();
            for (index, output) in regular.outputs().iter().enumerate() {
                if let Output::Nft(_nft_output) = output {
                    return Ok(OutputId::new(tx_payload.id(), index.try_into().unwrap())?);
                }
            }
            panic!("No nft output in transaction essence")
        }
        _ => panic!("No tx payload"),
    }
}
