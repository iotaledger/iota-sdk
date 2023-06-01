// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create an NFT output.
//!
//! `cargo run --example nft --release`

use iota_sdk::{
    client::{
        api::GetAddressesOptions, node_api::indexer::query_parameters::QueryParameter, request_funds_from_faucet,
        secret::SecretManager, Client, Result,
    },
    types::block::{
        address::{Bech32Address, NftAddress},
        output::{
            unlock_condition::AddressUnlockCondition, BasicOutputBuilder, NftId, NftOutputBuilder, Output, OutputId,
        },
        payload::{transaction::TransactionEssence, Payload},
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    // Configure your own mnemonic in the ".env" file. Since the output amount cannot be zero, the seed must contain
    // non-zero balance.
    dotenvy::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();
    let explorer_url = std::env::var("EXPLORER_URL").unwrap();
    let faucet_url = std::env::var("FAUCET_URL").unwrap();

    // Create a client instance.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    let secret_manager =
        SecretManager::try_from_mnemonic(std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    let token_supply = client.get_token_supply().await?;

    let address = secret_manager
        .generate_ed25519_addresses(GetAddressesOptions::from_client(&client).await?.with_range(0..1))
        .await?[0];
    request_funds_from_faucet(&faucet_url, &address).await?;
    tokio::time::sleep(std::time::Duration::from_secs(20)).await;

    //////////////////////////////////
    // create new nft output
    //////////////////////////////////

    let outputs = vec![
        // address of the owner of the NFT
        NftOutputBuilder::new_with_amount(1_000_000, NftId::null())
            .add_unlock_condition(AddressUnlockCondition::new(address))
            // address of the minter of the NFT
            // .add_feature(IssuerFeature::new(address))
            .finish_output(token_supply)?,
    ];

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_outputs(outputs)?
        .finish()
        .await?;

    println!("Block with new NFT output sent: {explorer_url}/block/{}", block.id());
    let _ = client.retry_until_included(&block.id(), None, None).await?;

    //////////////////////////////////
    // move funds from an NFT address
    //////////////////////////////////

    let nft_output_id = get_nft_output_id(block.payload().unwrap())?;
    let nft_id = NftId::from(&nft_output_id);

    let nft_address = NftAddress::new(nft_id);
    let bech32_nft_address = Bech32Address::new(client.get_bech32_hrp().await?, nft_address);
    println!("bech32_nft_address {bech32_nft_address}");
    println!(
        "Faucet request {:?}",
        request_funds_from_faucet(&faucet_url, &bech32_nft_address).await?
    );
    tokio::time::sleep(std::time::Duration::from_secs(20)).await;

    let output_ids_response = client
        .basic_output_ids(vec![QueryParameter::Address(bech32_nft_address)])
        .await?;
    let output_with_meta = client.get_output(&output_ids_response.items[0]).await?;

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_input(nft_output_id.into())?
        .with_input(output_ids_response.items[0].into())?
        .with_outputs(vec![
            NftOutputBuilder::new_with_amount(1_000_000 + output_with_meta.output().amount(), nft_id)
                .add_unlock_condition(AddressUnlockCondition::new(bech32_nft_address))
                .finish_output(token_supply)?,
        ])?
        .finish()
        .await?;

    println!(
        "Block with input (basic output) to NFT output sent: {explorer_url}/block/{}",
        block.id()
    );

    let _ = client.retry_until_included(&block.id(), None, None).await?;

    //////////////////////////////////
    // burn NFT
    //////////////////////////////////

    let nft_output_id = get_nft_output_id(block.payload().unwrap())?;
    let output_with_meta = client.get_output(&nft_output_id).await?;
    let outputs = vec![
        BasicOutputBuilder::new_with_amount(output_with_meta.output().amount())
            .add_unlock_condition(AddressUnlockCondition::new(bech32_nft_address))
            .finish_output(token_supply)?,
    ];

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_input(nft_output_id.into())?
        .with_outputs(outputs)?
        .finish()
        .await?;
    println!("Block with burn transaction sent: {explorer_url}/block/{}", block.id());
    let _ = client.retry_until_included(&block.id(), None, None).await?;
    Ok(())
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
