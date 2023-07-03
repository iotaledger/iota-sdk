// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use clap::{Parser, Subcommand};
use iota_sdk::{
    client::request_funds_from_faucet,
    types::{
        api::plugins::participation::types::ParticipationEventId,
        block::{
            address::Bech32Address,
            output::{
                unlock_condition::AddressUnlockCondition, AccountId, BasicOutputBuilder, FoundryId, NativeToken, NftId,
                Output, OutputId, TokenId,
            },
            payload::transaction::TransactionId,
            ConvertTo,
        },
    },
    wallet::{
        account::{types::AccountAddress, Account, OutputsToClaim, TransactionOptions},
        CreateNativeTokenParams, MintNftParams, SendAmountParams, SendNativeTokensParams, SendNftParams,
    },
    U256,
};

use crate::{error::Error, helper::to_utc_date_time, println_log_info};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, propagate_version = true)]
pub struct AccountCli {
    #[command(subcommand)]
    pub command: AccountCommand,
}

#[derive(Debug, Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum AccountCommand {
    /// List the account addresses.
    Addresses,
    /// Print the account balance.
    Balance {
        /// Addresses to compute the balance for.
        addresses: Option<Vec<Bech32Address>>,
    },
    /// Burn an amount of native token.
    BurnNativeToken {
        /// Token ID to be burnt, e.g. 0x087d205988b733d97fb145ae340e27a8b19554d1ceee64574d7e5ff66c45f69e7a0100000000.
        token_id: String,
        /// Amount to be burnt, e.g. 100.
        amount: String,
    },
    /// Burn an NFT.
    BurnNft {
        /// NFT ID to be burnt, e.g. 0xecadf10e6545aa82da4df2dfd2a496b457c8850d2cab49b7464cb273d3dffb07.
        nft_id: String,
    },
    /// Claim outputs with storage deposit return, expiration or timelock unlock conditions.
    Claim {
        /// Output ID to be claimed.
        output_id: Option<String>,
    },
    /// Print details about claimable outputs - if there are any.
    ClaimableOutputs,
    /// Consolidate all basic outputs into one address.
    Consolidate,
    /// Create a new account output.
    CreateAccountOutput,
    /// Create a native token.
    CreateNativeToken {
        /// Circulating supply of the native token to be minted, e.g. 100.
        circulating_supply: String,
        /// Maximum supply of the native token to be minted, e.g. 500.
        maximum_supply: String,
        /// Metadata to attach to the associated foundry, e.g. --foundry-metadata-hex 0xdeadbeef.
        #[arg(long, group = "foundry_metadata")]
        foundry_metadata_hex: Option<String>,
        /// Metadata to attach to the associated foundry, e.g. --foundry-metadata-file ./foundry-metadata.json.
        #[arg(long, group = "foundry_metadata")]
        foundry_metadata_file: Option<String>,
    },
    /// Destroy an account output.
    DestroyAccount {
        /// Account ID to be destroyed, e.g. 0xed5a90106ae5d402ebaecb9ba36f32658872df789f7a29b9f6d695b912ec6a1e.
        account_id: String,
    },
    /// Destroy a foundry.
    DestroyFoundry {
        /// Foundry ID to be destroyed, e.g.
        /// 0x08cb54928954c3eb7ece1bf1cc0c68eb179dc1c4634ae5d23df1c70643d0911c3d0200000000.
        foundry_id: String,
    },
    /// Exit the CLI wallet.
    Exit,
    /// Request funds from the faucet.
    Faucet {
        /// Address the faucet sends the funds to, defaults to the latest address.
        address: Option<Bech32Address>,
        /// URL of the faucet, default to <https://faucet.testnet.shimmer.network/api/enqueue>.
        url: Option<String>,
    },
    /// Mint additional native tokens.
    MintNativeToken {
        /// Token ID to be minted, e.g. 0x087d205988b733d97fb145ae340e27a8b19554d1ceee64574d7e5ff66c45f69e7a0100000000.
        token_id: String,
        /// Amount to be minted, e.g. 100.
        amount: String,
    },
    /// Mint an NFT.
    /// IOTA NFT Standard - TIP27: <https://github.com/iotaledger/tips/blob/main/tips/TIP-0027/tip-0027.md>.
    MintNft {
        /// Address to send the NFT to, e.g. rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3.
        address: Option<Bech32Address>,
        #[arg(long, group = "immutable_metadata")]
        /// Immutable metadata to attach to the NFT, e.g. --immutable-metadata-hex 0xdeadbeef.
        immutable_metadata_hex: Option<String>,
        /// Immutable metadata to attach to the NFT, e.g. --immutable-metadata-file ./nft-immutable-metadata.json.
        #[arg(long, group = "immutable_metadata")]
        immutable_metadata_file: Option<String>,
        /// Metadata to attach to the NFT, e.g. --metadata-hex 0xdeadbeef.
        #[arg(long, group = "metadata")]
        metadata_hex: Option<String>,
        /// Metadata to attach to the NFT, e.g. --metadata-file ./nft-metadata.json.
        #[arg(long, group = "metadata")]
        metadata_file: Option<String>,
        #[arg(long)]
        /// Tag feature to attach to the NFT, e.g. 0xdeadbeef.
        tag: Option<String>,
        /// Sender feature to attach to the NFT, e.g. rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3.
        #[arg(long)]
        sender: Option<Bech32Address>,
        /// Issuer feature to attach to the NFT, e.g. rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3.
        #[arg(long)]
        issuer: Option<Bech32Address>,
    },
    /// Melt an amount of native token.
    MeltNativeToken {
        /// Token ID to be melted, e.g. 0x087d205988b733d97fb145ae340e27a8b19554d1ceee64574d7e5ff66c45f69e7a0100000000.
        token_id: String,
        /// Amount to be melted, e.g. 100.
        amount: String,
    },
    /// Generate a new address.
    NewAddress,
    /// Get information about currently set node.
    NodeInfo,
    /// Display an output.
    Output {
        /// Output ID to be displayed.
        output_id: String,
    },
    /// List all outputs.
    Outputs,
    /// Send an amount.
    Send {
        /// Address to send funds to, e.g. rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3.
        address: Bech32Address,
        /// Amount to send, e.g. 1000000.
        amount: u64,
        /// Bech32 encoded return address, to which the storage deposit will be returned if one is necessary
        /// given the provided amount. If a storage deposit is needed and a return address is not provided, it will
        /// default to the first address of the account.
        #[arg(long)]
        return_address: Option<Bech32Address>,
        /// Expiration in seconds, after which the output will be available for the sender again, if not spent by the
        /// receiver already. The expiration will only be used if one is necessary given the provided amount. If an
        /// expiration is needed but not provided, it will default to one day.
        #[arg(long)]
        expiration: Option<humantime::Duration>,
        /// Whether to send micro amounts. This will automatically add Storage Deposit Return and Expiration unlock
        /// conditions if necessary. This flag is implied by the existence of a return address or expiration.
        #[arg(long, default_value_t = false)]
        allow_micro_amount: bool,
    },
    /// Send native tokens.
    /// This will create an output with an expiration and storage deposit return unlock condition.
    SendNativeToken {
        /// Address to send the native tokens to, e.g. rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3.
        address: Bech32Address,
        /// Token ID to be sent, e.g. 0x087d205988b733d97fb145ae340e27a8b19554d1ceee64574d7e5ff66c45f69e7a0100000000.
        token_id: String,
        /// Amount to send, e.g. 1000000.
        amount: String,
        /// Whether to gift the storage deposit for the output or not, e.g. ` true`.
        gift_storage_deposit: Option<bool>,
    },
    /// Send an NFT.
    SendNft {
        /// Address to send the NFT to, e.g. rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3.
        address: Bech32Address,
        /// NFT ID to be sent, e.g. 0xecadf10e6545aa82da4df2dfd2a496b457c8850d2cab49b7464cb273d3dffb07.
        nft_id: String,
    },
    /// Synchronize the account.
    Sync,
    /// Show the details of the transaction.
    #[clap(visible_alias = "tx")]
    Transaction {
        /// Transaction ID to be displayed e.g. 0x84fe6b1796bddc022c9bc40206f0a692f4536b02aa8c13140264e2e01a3b7e4b.
        transaction_id: String,
    },
    /// List the account transactions.
    #[clap(visible_alias = "txs")]
    Transactions {
        /// List account transactions with all details.
        #[arg(long, default_value_t = false)]
        show_details: bool,
    },
    /// List the account unspent outputs.
    UnspentOutputs,
    /// Cast votes for an event.
    Vote {
        /// Event ID for which to cast votes, e.g. 0xdc049a721dc65ec342f836c876ec15631ed915cd55213cee39e8d1c821c751f2.
        event_id: ParticipationEventId,
        /// Answers to the event questions.
        answers: Vec<u8>,
    },
    /// Stop participating to an event.
    StopParticipating {
        /// Event ID for which to stop participation, e.g.
        /// 0xdc049a721dc65ec342f836c876ec15631ed915cd55213cee39e8d1c821c751f2.
        event_id: ParticipationEventId,
    },
    /// Get the participation overview of the account.
    ParticipationOverview {
        /// Event IDs for which to get the participation overview, e.g.
        /// 0xdc049a721dc65ec342f836c876ec15631ed915cd55213cee39e8d1c821c751f2...
        #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
        event_ids: Vec<ParticipationEventId>,
    },
    /// Get the voting power of the account.
    VotingPower,
    /// Increase the voting power of the account.
    IncreaseVotingPower {
        /// Amount to increase the voting power by, e.g. 100.
        amount: u64,
    },
    /// Decrease the voting power of the account.
    DecreaseVotingPower {
        /// Amount to decrease the voting power by, e.g. 100.
        amount: u64,
    },
    /// Get the voting output of the account.
    VotingOutput,
}

/// `addresses` command
pub async fn addresses_command(account: &Account) -> Result<(), Error> {
    let addresses = account.addresses().await?;

    if addresses.is_empty() {
        println_log_info!("No addresses found");
    } else {
        for address in addresses {
            print_address(account, &address).await?;
        }
    }

    Ok(())
}

// `balance` command
pub async fn balance_command(account: &Account, addresses: Option<Vec<Bech32Address>>) -> Result<(), Error> {
    let balance = if let Some(addresses) = addresses {
        account.addresses_balance(addresses).await?
    } else {
        account.balance().await?
    };
    println_log_info!("{balance:#?}");

    Ok(())
}

// `burn-native-token` command
pub async fn burn_native_token_command(account: &Account, token_id: String, amount: String) -> Result<(), Error> {
    println_log_info!("Burning native token {token_id} {amount}.");

    let transaction = account
        .burn(
            NativeToken::new(
                TokenId::from_str(&token_id)?,
                U256::from_dec_str(&amount).map_err(|e| Error::Miscellaneous(e.to_string()))?,
            )?,
            None,
        )
        .await?;

    println_log_info!(
        "Burning transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `burn-nft` command
pub async fn burn_nft_command(account: &Account, nft_id: String) -> Result<(), Error> {
    println_log_info!("Burning nft {nft_id}.");

    let transaction = account.burn(NftId::from_str(&nft_id)?, None).await?;

    println_log_info!(
        "Burning transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `claim` command
pub async fn claim_command(account: &Account, output_id: Option<String>) -> Result<(), Error> {
    if let Some(output_id) = output_id {
        println_log_info!("Claiming output {output_id}");

        let transaction = account.claim_outputs([OutputId::from_str(&output_id)?]).await?;

        println_log_info!(
            "Claiming transaction sent:\n{:?}\n{:?}",
            transaction.transaction_id,
            transaction.block_id
        );
    } else {
        println_log_info!("Claiming outputs.");

        let output_ids = account.claimable_outputs(OutputsToClaim::All).await?;

        if output_ids.is_empty() {
            println_log_info!("No outputs available to claim.");
        }

        // Doing chunks of only 60, because we might need to create the double amount of outputs, because of potential
        // storage deposit return unlock conditions and also consider the remainder output.
        for output_ids_chunk in output_ids.chunks(60) {
            let transaction = account.claim_outputs(output_ids_chunk.to_vec()).await?;
            println_log_info!(
                "Claiming transaction sent:\n{:?}\n{:?}",
                transaction.transaction_id,
                transaction.block_id
            );
        }
    };

    Ok(())
}

/// `claimable-outputs` command
pub async fn claimable_outputs_command(account: &Account) -> Result<(), Error> {
    let balance = account.balance().await?;
    for output_id in balance
        .potentially_locked_outputs()
        .iter()
        .filter_map(|(output_id, unlockable)| unlockable.then_some(output_id))
    {
        // Unwrap: for the iterated `OutputId`s this call will always return `Some(...)`.
        let output_data = account.get_output(output_id).await.unwrap();
        let output = output_data.output;
        let kind = match output {
            Output::Nft(_) => "Nft",
            Output::Basic(_) => "Basic",
            _ => unreachable!(),
        };
        println_log_info!("{output_id:?} ({kind})");

        if let Some(native_tokens) = output.native_tokens() {
            if !native_tokens.is_empty() {
                println_log_info!("  - native token amount:");
                native_tokens.iter().for_each(|token| {
                    println_log_info!("    + {} {}", token.amount(), token.token_id());
                });
            }
        }

        if let Some(unlock_conditions) = output.unlock_conditions() {
            let deposit_return = unlock_conditions
                .storage_deposit_return()
                .map(|deposit_return| deposit_return.amount())
                .unwrap_or(0);
            let amount = output.amount() - deposit_return;
            println_log_info!("  - base coin amount: {}", amount);

            if let Some(expiration) = unlock_conditions.expiration() {
                let current_time = iota_sdk::utils::unix_timestamp_now().as_secs() as u32;
                let time_left = expiration.timestamp() - current_time;
                println_log_info!("  - expires in: {} seconds", time_left);
            }
        }
    }

    Ok(())
}

// `consolidate` command
pub async fn consolidate_command(account: &Account) -> Result<(), Error> {
    println_log_info!("Consolidating outputs.");

    let transaction = account.consolidate_outputs(true, None).await?;

    println_log_info!(
        "Consolidation transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `create-account-output` command
pub async fn create_account_output_command(account: &Account) -> Result<(), Error> {
    println_log_info!("Creating account output.");

    let transaction = account.create_account_output(None, None).await?;

    println_log_info!(
        "Account output creation transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `create-native-token` command
pub async fn create_native_token_command(
    account: &Account,
    circulating_supply: String,
    maximum_supply: String,
    foundry_metadata: Option<Vec<u8>>,
) -> Result<(), Error> {
    // If no account output exists, create one first
    if account.balance().await?.accounts().is_empty() {
        let transaction = account.create_account_output(None, None).await?;
        println_log_info!(
            "Account output minting transaction sent:\n{:?}\n{:?}",
            transaction.transaction_id,
            transaction.block_id
        );
        account
            .retry_transaction_until_included(&transaction.transaction_id, None, None)
            .await?;
        // Sync account after the transaction got confirmed, so the account output is available
        account.sync(None).await?;
    }

    let params = CreateNativeTokenParams {
        account_id: None,
        circulating_supply: U256::from_dec_str(&circulating_supply).map_err(|e| Error::Miscellaneous(e.to_string()))?,
        maximum_supply: U256::from_dec_str(&maximum_supply).map_err(|e| Error::Miscellaneous(e.to_string()))?,
        foundry_metadata,
    };

    let create_transaction = account.create_native_token(params, None).await?;

    println_log_info!(
        "Transaction to create native token sent:\n{:?}\n{:?}",
        create_transaction.transaction.transaction_id,
        create_transaction.transaction.block_id
    );

    Ok(())
}

// `destroy-account` command
pub async fn destroy_account_command(account: &Account, account_id: String) -> Result<(), Error> {
    println_log_info!("Destroying account {account_id}.");

    let transaction = account.burn(AccountId::from_str(&account_id)?, None).await?;

    println_log_info!(
        "Destroying account transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `destroy-foundry` command
pub async fn destroy_foundry_command(account: &Account, foundry_id: String) -> Result<(), Error> {
    println_log_info!("Destroying foundry {foundry_id}.");

    let transaction = account.burn(FoundryId::from_str(&foundry_id)?, None).await?;

    println_log_info!(
        "Destroying foundry transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `faucet` command
pub async fn faucet_command(
    account: &Account,
    address: Option<Bech32Address>,
    url: Option<String>,
) -> Result<(), Error> {
    let address = if let Some(address) = address {
        address
    } else {
        match account.addresses().await?.last() {
            Some(address) => *address.address(),
            None => return Err(Error::NoAddressForFaucet),
        }
    };
    let faucet_url = url
        .as_deref()
        .unwrap_or("https://faucet.testnet.shimmer.network/api/enqueue");

    println_log_info!("{}", request_funds_from_faucet(faucet_url, &address).await?);

    Ok(())
}

// `melt-native-token` command
pub async fn melt_native_token_command(account: &Account, token_id: String, amount: String) -> Result<(), Error> {
    let transaction = account
        .melt_native_token(
            TokenId::from_str(&token_id)?,
            U256::from_dec_str(&amount).map_err(|e| Error::Miscellaneous(e.to_string()))?,
            None,
        )
        .await?;

    println_log_info!(
        "Native token melting transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `mint-native-token` command
pub async fn mint_native_token(account: &Account, token_id: String, amount: String) -> Result<(), Error> {
    let mint_transaction = account
        .mint_native_token(
            TokenId::from_str(&token_id)?,
            U256::from_dec_str(&amount).map_err(|e| Error::Miscellaneous(e.to_string()))?,
            None,
        )
        .await?;

    println_log_info!(
        "Transaction minting additional native tokens sent:\n{:?}\n{:?}",
        mint_transaction.transaction_id,
        mint_transaction.block_id
    );

    Ok(())
}

// `mint-nft` command
pub async fn mint_nft_command(
    account: &Account,
    address: Option<Bech32Address>,
    immutable_metadata: Option<Vec<u8>>,
    metadata: Option<Vec<u8>>,
    tag: Option<String>,
    sender: Option<Bech32Address>,
    issuer: Option<Bech32Address>,
) -> Result<(), Error> {
    let tag = if let Some(hex) = tag {
        Some(prefix_hex::decode(hex).map_err(|e| Error::Miscellaneous(e.to_string()))?)
    } else {
        None
    };

    let nft_options = MintNftParams::new()
        .with_address(address)
        .with_immutable_metadata(immutable_metadata)
        .with_metadata(metadata)
        .with_tag(tag)
        .with_sender(sender)
        .with_issuer(issuer);
    let transaction = account.mint_nfts([nft_options], None).await?;

    println_log_info!(
        "NFT minting transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `new-address` command
pub async fn new_address_command(account: &Account) -> Result<(), Error> {
    let address = account.generate_ed25519_addresses(1, None).await?;

    print_address(account, &address[0]).await?;

    Ok(())
}

// `node-info` command
pub async fn node_info_command(account: &Account) -> Result<(), Error> {
    let node_info = account.client().get_info().await?;

    println_log_info!("Current node info: {}", serde_json::to_string_pretty(&node_info)?);

    Ok(())
}

/// `output` command
pub async fn output_command(account: &Account, output_id: String) -> Result<(), Error> {
    let output = account.get_output(&OutputId::from_str(&output_id)?).await;

    if let Some(output) = output {
        println_log_info!("{output:#?}");
    } else {
        println_log_info!("Output not found");
    }

    Ok(())
}

/// `outputs` command
pub async fn outputs_command(account: &Account) -> Result<(), Error> {
    let outputs = account.outputs(None).await?;

    if outputs.is_empty() {
        println_log_info!("No outputs found");
    } else {
        let output_ids: Vec<OutputId> = outputs.iter().map(|o| o.output_id).collect();
        println_log_info!("Outputs: {output_ids:#?}");
    }

    Ok(())
}

// `send` command
pub async fn send_command(
    account: &Account,
    address: impl ConvertTo<Bech32Address>,
    amount: u64,
    return_address: Option<impl ConvertTo<Bech32Address>>,
    expiration: Option<u32>,
    allow_micro_amount: bool,
) -> Result<(), Error> {
    let outputs = [SendAmountParams::new(address, amount)?
        .with_return_address(return_address.map(ConvertTo::convert).transpose()?)
        .with_expiration(expiration)];
    let transaction = account
        .send_amount(
            outputs,
            TransactionOptions {
                allow_micro_amount,
                ..Default::default()
            },
        )
        .await?;

    println_log_info!(
        "Transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `send-native-token` command
pub async fn send_native_token_command(
    account: &Account,
    address: impl ConvertTo<Bech32Address>,
    token_id: String,
    amount: String,
    gift_storage_deposit: Option<bool>,
) -> Result<(), Error> {
    let address = address.convert()?;
    let transaction = if gift_storage_deposit.unwrap_or(false) {
        // Send native tokens together with the required storage deposit
        let rent_structure = account.client().get_rent_structure().await?;
        let token_supply = account.client().get_token_supply().await?;

        account.client().bech32_hrp_matches(address.hrp()).await?;

        let outputs = [BasicOutputBuilder::new_with_minimum_storage_deposit(rent_structure)
            .add_unlock_condition(AddressUnlockCondition::new(address))
            .with_native_tokens([NativeToken::new(
                TokenId::from_str(&token_id)?,
                U256::from_dec_str(&amount).map_err(|e| Error::Miscellaneous(e.to_string()))?,
            )?])
            .finish_output(token_supply)?];

        account.send(outputs, None).await?
    } else {
        // Send native tokens with storage deposit return and expiration
        let outputs = [SendNativeTokensParams::new(
            address,
            [(
                TokenId::from_str(&token_id)?,
                U256::from_dec_str(&amount).map_err(|e| Error::Miscellaneous(e.to_string()))?,
            )],
        )?];
        account.send_native_tokens(outputs, None).await?
    };

    println_log_info!(
        "Native token transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `send-nft` command
pub async fn send_nft_command(
    account: &Account,
    address: impl ConvertTo<Bech32Address>,
    nft_id: String,
) -> Result<(), Error> {
    let outputs = [SendNftParams::new(address.convert()?, &nft_id)?];
    let transaction = account.send_nft(outputs, None).await?;

    println_log_info!(
        "Nft transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `sync` command
pub async fn sync_command(account: &Account) -> Result<(), Error> {
    let balance = account.sync(None).await?;
    println_log_info!("Synced.");
    println_log_info!("{balance:#?}");

    Ok(())
}

/// `transaction` command
pub async fn transaction_command(account: &Account, transaction_id_str: &str) -> Result<(), Error> {
    let transaction_id = TransactionId::from_str(transaction_id_str)?;
    let maybe_transaction = account
        .transactions()
        .await
        .into_iter()
        .find(|tx| tx.transaction_id == transaction_id);

    if let Some(tx) = maybe_transaction {
        println_log_info!("{:#?}", tx);
    } else {
        println_log_info!("No transaction found");
    }

    Ok(())
}

/// `transactions` command
pub async fn transactions_command(account: &Account, show_details: bool) -> Result<(), Error> {
    let mut transactions = account.transactions().await;
    transactions.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    if transactions.is_empty() {
        println_log_info!("No transactions found");
    } else {
        for (i, tx) in transactions.into_iter().rev().enumerate() {
            if show_details {
                println_log_info!("{:#?}", tx);
            } else {
                let transaction_time = to_utc_date_time(tx.timestamp)?;
                let formatted_time = transaction_time.format("%Y-%m-%d %H:%M:%S").to_string();

                println_log_info!("{:<5}{}\t{}", i, tx.transaction_id, formatted_time);
            }
        }
    }

    Ok(())
}

/// `unspent-outputs` command
pub async fn unspent_outputs_command(account: &Account) -> Result<(), Error> {
    let outputs = account.unspent_outputs(None).await?;

    if outputs.is_empty() {
        println_log_info!("No outputs found");
    } else {
        let output_ids: Vec<OutputId> = outputs.iter().map(|o| o.output_id).collect();
        println_log_info!("Unspent outputs: {output_ids:#?}");
    }

    Ok(())
}

pub async fn vote_command(account: &Account, event_id: ParticipationEventId, answers: Vec<u8>) -> Result<(), Error> {
    let transaction = account.vote(Some(event_id), Some(answers)).await?;

    println_log_info!(
        "Voting transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

pub async fn stop_participating_command(account: &Account, event_id: ParticipationEventId) -> Result<(), Error> {
    let transaction = account.stop_participating(event_id).await?;

    println_log_info!(
        "Stop participating transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

pub async fn participation_overview_command(
    account: &Account,
    event_ids: Option<Vec<ParticipationEventId>>,
) -> Result<(), Error> {
    let participation_overview = account.get_participation_overview(event_ids).await?;

    println_log_info!("Participation overview: {participation_overview:?}");

    Ok(())
}

pub async fn voting_power_command(account: &Account) -> Result<(), Error> {
    let voting_power = account.get_voting_power().await?;

    println_log_info!("Voting power: {voting_power}");

    Ok(())
}

pub async fn increase_voting_power_command(account: &Account, amount: u64) -> Result<(), Error> {
    let transaction = account.increase_voting_power(amount).await?;

    println_log_info!(
        "Increase voting power transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

pub async fn decrease_voting_power_command(account: &Account, amount: u64) -> Result<(), Error> {
    let transaction = account.decrease_voting_power(amount).await?;

    println_log_info!(
        "Decrease voting power transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

pub async fn voting_output_command(account: &Account) -> Result<(), Error> {
    let output = account.get_voting_output().await?;

    println_log_info!("Voting output: {output:?}");

    Ok(())
}

async fn print_address(account: &Account, address: &AccountAddress) -> Result<(), Error> {
    let mut log = format!("Address {}: {}", address.key_index(), address.address());

    if *address.internal() {
        log = format!("{log}\nChange address");
    }

    let addresses = account.addresses_with_unspent_outputs().await?;
    let current_time = iota_sdk::utils::unix_timestamp_now().as_secs() as u32;

    if let Ok(index) = addresses.binary_search_by_key(&(address.key_index(), address.internal()), |a| {
        (a.key_index(), a.internal())
    }) {
        let mut address_amount = 0;
        for output_id in addresses[index].output_ids() {
            if let Some(output_data) = account.get_output(output_id).await {
                // Output might be associated with the address, but can't unlocked by it, so we check that here
                let (required_address, _) =
                    output_data
                        .output
                        .required_and_unlocked_address(current_time, output_id, None)?;
                if *address.address().as_ref() == required_address {
                    let unlock_conditions = output_data
                        .output
                        .unlock_conditions()
                        .expect("output must have unlock conditions");

                    if let Some(sdr) = unlock_conditions.storage_deposit_return() {
                        address_amount += output_data.output.amount() - sdr.amount();
                    } else {
                        address_amount += output_data.output.amount();
                    }
                }
            }
        }
        log = format!(
            "{log}\nOutputs: {:#?}\nBase coin amount: {}\n",
            addresses[index].output_ids(),
            address_amount
        );
    } else {
        log = format!("{log}\nOutputs: []\nBase coin amount: 0\n");
    }

    println_log_info!("{log}");

    Ok(())
}
