// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use clap::{Parser, Subcommand};
use iota_sdk::{
    client::request_funds_from_faucet,
    types::{
        api::plugins::participation::types::ParticipationEventId,
        block::{
            address::Address,
            output::{
                unlock_condition::AddressUnlockCondition, AliasId, BasicOutputBuilder, FoundryId, NativeToken, NftId,
                OutputId, TokenId,
            },
            payload::transaction::TransactionId,
        },
    },
    wallet::{
        account::{types::AccountAddress, AccountHandle, OutputsToClaim, TransactionOptions},
        AddressAndNftId, AddressNativeTokens, AddressWithAmount, NativeTokenOptions, NftOptions, U256,
    },
};

use crate::{error::Error, helper::to_utc_date_time, println_log_info};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, propagate_version = true)]
pub struct AccountCli {
    #[command(subcommand)]
    pub command: AccountCommand,
}

#[derive(Debug, Subcommand)]
pub enum AccountCommand {
    /// List the account addresses.
    Addresses,
    /// Print the account balance.
    Balance,
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
    /// Consolidate all basic outputs into one address.
    Consolidate,
    /// Create a new alias output.
    CreateAliasOutput,
    /// Melt an amount of native token.
    DecreaseNativeTokenSupply {
        /// Token ID to be melted, e.g. 0x087d205988b733d97fb145ae340e27a8b19554d1ceee64574d7e5ff66c45f69e7a0100000000.
        token_id: String,
        /// Amount to be melted, e.g. 100.
        amount: String,
    },
    /// Destroy an alias.
    DestroyAlias {
        /// Alias ID to be destroyed, e.g. 0xed5a90106ae5d402ebaecb9ba36f32658872df789f7a29b9f6d695b912ec6a1e.
        alias_id: String,
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
        address: Option<String>,
        /// URL of the faucet, default to <https://faucet.testnet.shimmer.network/api/enqueue>.
        url: Option<String>,
    },
    /// Mint more of a native token.
    IncreaseNativeTokenSupply {
        /// Token ID to be minted, e.g. 0x087d205988b733d97fb145ae340e27a8b19554d1ceee64574d7e5ff66c45f69e7a0100000000.
        token_id: String,
        /// Amount to be minted, e.g. 100.
        amount: String,
    },
    /// Mint a native token.
    MintNativeToken {
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
    /// Mint an NFT.
    /// IOTA NFT Standard - TIP27: <https://github.com/iotaledger/tips/blob/main/tips/TIP-0027/tip-0027.md>.
    MintNft {
        /// Address to send the NFT to, e.g. rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3.
        address: Option<String>,
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
        sender: Option<String>,
        /// Issuer feature to attach to the NFT, e.g. rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3.
        #[arg(long)]
        issuer: Option<String>,
    },
    /// Generate a new address.
    NewAddress,
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
        address: String,
        /// Amount to send, e.g. 1000000.
        amount: u64,
        /// Bech32 encoded return address, to which the storage deposit will be returned if one is necessary
        /// given the provided amount. If a storage deposit is needed and a return address is not provided, it will
        /// default to the first address of the account.
        #[arg(long)]
        return_address: Option<String>,
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
        address: String,
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
        address: String,
        /// NFT ID to be sent, e.g. 0xecadf10e6545aa82da4df2dfd2a496b457c8850d2cab49b7464cb273d3dffb07.
        nft_id: String,
    },
    /// Synchronize the account.
    Sync,
    /// Show the details of the transaction.
    #[clap(alias = "tx")]
    Transaction {
        /// Transaction ID to be displayed e.g. 0x84fe6b1796bddc022c9bc40206f0a692f4536b02aa8c13140264e2e01a3b7e4b.
        transaction_id: String,
    },
    /// List the account transactions.
    #[clap(alias = "txs")]
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
pub async fn addresses_command(account_handle: &AccountHandle) -> Result<(), Error> {
    let addresses = account_handle.addresses().await?;

    if addresses.is_empty() {
        println_log_info!("No addresses found");
    } else {
        for address in addresses {
            print_address(account_handle, &address).await?;
        }
    }

    Ok(())
}

// `burn-native-token` command
pub async fn burn_native_token_command(
    account_handle: &AccountHandle,
    token_id: String,
    amount: String,
) -> Result<(), Error> {
    println_log_info!("Burning native token {token_id} {amount}.");

    let transaction = account_handle
        .burn_native_token(
            TokenId::from_str(&token_id)?,
            U256::from_dec_str(&amount).map_err(|e| Error::Miscellaneous(e.to_string()))?,
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
pub async fn burn_nft_command(account_handle: &AccountHandle, nft_id: String) -> Result<(), Error> {
    println_log_info!("Burning nft {nft_id}.");

    let transaction = account_handle.burn_nft(NftId::from_str(&nft_id)?, None).await?;

    println_log_info!(
        "Burning transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `balance` command
pub async fn balance_command(account_handle: &AccountHandle) -> Result<(), Error> {
    println_log_info!("{:#?}", account_handle.balance().await?);

    Ok(())
}

// `claim` command
pub async fn claim_command(account_handle: &AccountHandle, output_id: Option<String>) -> Result<(), Error> {
    if let Some(output_id) = output_id {
        println_log_info!("Claiming output {output_id}");

        let transaction = account_handle
            .claim_outputs(vec![OutputId::from_str(&output_id)?])
            .await?;

        println_log_info!(
            "Claiming transaction sent:\n{:?}\n{:?}",
            transaction.transaction_id,
            transaction.block_id
        );
    } else {
        println_log_info!("Claiming outputs.");

        let output_ids = account_handle
            .get_unlockable_outputs_with_additional_unlock_conditions(OutputsToClaim::All)
            .await?;

        if output_ids.is_empty() {
            println_log_info!("No outputs available to claim.");
        }

        // Doing chunks of only 60, because we might need to create the double amount of outputs, because of potential
        // storage deposit return unlock conditions and also consider the remainder output.
        for output_ids_chunk in output_ids.chunks(60) {
            let transaction = account_handle.claim_outputs(output_ids_chunk.to_vec()).await?;
            println_log_info!(
                "Claiming transaction sent:\n{:?}\n{:?}",
                transaction.transaction_id,
                transaction.block_id
            );
        }
    };

    Ok(())
}

// `consolidate` command
pub async fn consolidate_command(account_handle: &AccountHandle) -> Result<(), Error> {
    println_log_info!("Consolidating outputs.");

    let transaction = account_handle.consolidate_outputs(true, None).await?;

    println_log_info!(
        "Consolidation transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `create-alias-output` command
pub async fn create_alias_outputs_command(account_handle: &AccountHandle) -> Result<(), Error> {
    println_log_info!("Creating alias output.");

    let transaction = account_handle.create_alias_output(None, None).await?;

    println_log_info!(
        "Alias output creation transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `decrease-native-token-supply` command
pub async fn decrease_native_token_command(
    account_handle: &AccountHandle,
    token_id: String,
    amount: String,
) -> Result<(), Error> {
    let transaction = account_handle
        .decrease_native_token_supply(
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

// `destroy-alias` command
pub async fn destroy_alias_command(account_handle: &AccountHandle, alias_id: String) -> Result<(), Error> {
    println_log_info!("Destroying alias {alias_id}.");

    let transaction = account_handle
        .destroy_alias(AliasId::from_str(&alias_id)?, None)
        .await?;

    println_log_info!(
        "Destroying alias transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `destroy-foundry` command
pub async fn destroy_foundry_command(account_handle: &AccountHandle, foundry_id: String) -> Result<(), Error> {
    println_log_info!("Destroying foundry {foundry_id}.");

    let transaction = account_handle
        .destroy_foundry(FoundryId::from_str(&foundry_id)?, None)
        .await?;

    println_log_info!(
        "Destroying foundry transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `faucet` command
pub async fn faucet_command(
    account_handle: &AccountHandle,
    address: Option<String>,
    url: Option<String>,
) -> Result<(), Error> {
    let address = if let Some(address) = address {
        address
    } else {
        match account_handle.addresses().await?.last() {
            Some(address) => address.address().to_string(),
            None => return Err(Error::NoAddressForFaucet),
        }
    };
    let faucet_url = url
        .as_deref()
        .unwrap_or("https://faucet.testnet.shimmer.network/api/enqueue");

    println_log_info!("{}", request_funds_from_faucet(faucet_url, &address).await?);

    Ok(())
}

// `increase-native-token-supply` command
pub async fn increase_native_token_command(
    account_handle: &AccountHandle,
    token_id: String,
    amount: String,
) -> Result<(), Error> {
    let mint_transaction = account_handle
        .increase_native_token_supply(
            TokenId::from_str(&token_id)?,
            U256::from_dec_str(&amount).map_err(|e| Error::Miscellaneous(e.to_string()))?,
            None,
            None,
        )
        .await?;

    println_log_info!(
        "Minting more native token transaction sent:\n{:?}\n{:?}",
        mint_transaction.transaction.transaction_id,
        mint_transaction.transaction.block_id
    );

    Ok(())
}

// `mint-native-token` command
pub async fn mint_native_token_command(
    account_handle: &AccountHandle,
    circulating_supply: String,
    maximum_supply: String,
    foundry_metadata: Option<Vec<u8>>,
) -> Result<(), Error> {
    // If no alias output exists, create one first
    if account_handle.balance().await?.aliases().is_empty() {
        let transaction = account_handle.create_alias_output(None, None).await?;
        println_log_info!(
            "Alias output minting transaction sent:\n{:?}\n{:?}",
            transaction.transaction_id,
            transaction.block_id
        );
        account_handle
            .retry_transaction_until_included(&transaction.transaction_id, None, None)
            .await?;
        // Sync account after the transaction got confirmed, so the alias output is available
        account_handle.sync(None).await?;
    }

    let native_token_options = NativeTokenOptions {
        alias_id: None,
        circulating_supply: U256::from_dec_str(&circulating_supply).map_err(|e| Error::Miscellaneous(e.to_string()))?,
        maximum_supply: U256::from_dec_str(&maximum_supply).map_err(|e| Error::Miscellaneous(e.to_string()))?,
        foundry_metadata,
    };

    let mint_transaction = account_handle.mint_native_token(native_token_options, None).await?;

    println_log_info!(
        "Native token minting transaction sent:\n{:?}\n{:?}",
        mint_transaction.transaction.transaction_id,
        mint_transaction.transaction.block_id
    );

    Ok(())
}

// `mint-nft` command
pub async fn mint_nft_command(
    account_handle: &AccountHandle,
    address: Option<String>,
    immutable_metadata: Option<Vec<u8>>,
    metadata: Option<Vec<u8>>,
    tag: Option<String>,
    sender: Option<String>,
    issuer: Option<String>,
) -> Result<(), Error> {
    let tag = if let Some(hex) = tag {
        Some(prefix_hex::decode(hex).map_err(|e| Error::Miscellaneous(e.to_string()))?)
    } else {
        None
    };
    let nft_options = vec![NftOptions {
        issuer,
        sender,
        tag,
        address,
        immutable_metadata,
        metadata,
    }];
    let transaction = account_handle.mint_nfts(nft_options, None).await?;

    println_log_info!(
        "NFT minting transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `new-address` command
pub async fn new_address_command(account_handle: &AccountHandle) -> Result<(), Error> {
    let address = account_handle.generate_addresses(1, None).await?;

    print_address(account_handle, &address[0]).await?;

    Ok(())
}

/// `output` command
pub async fn output_command(account_handle: &AccountHandle, output_id: String) -> Result<(), Error> {
    let output = account_handle.get_output(&OutputId::from_str(&output_id)?).await;

    if let Some(output) = output {
        println_log_info!("{output:#?}");
    } else {
        println_log_info!("Output not found");
    }

    Ok(())
}

/// `outputs` command
pub async fn outputs_command(account_handle: &AccountHandle) -> Result<(), Error> {
    let outputs = account_handle.outputs(None).await?;

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
    account_handle: &AccountHandle,
    address: String,
    amount: u64,
    return_address: Option<String>,
    expiration: Option<u32>,
    allow_micro_amount: bool,
) -> Result<(), Error> {
    let outputs = vec![
        AddressWithAmount::new(address, amount)
            .with_return_address(return_address)
            .with_expiration(expiration),
    ];
    let transaction = account_handle
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
    account_handle: &AccountHandle,
    address: String,
    token_id: String,
    amount: String,
    gift_storage_deposit: Option<bool>,
) -> Result<(), Error> {
    let transaction = if gift_storage_deposit.unwrap_or(false) {
        // Send native tokens together with the required storage deposit
        let rent_structure = account_handle.client().get_rent_structure().await?;
        let token_supply = account_handle.client().get_token_supply().await?;

        let (bech32_hrp, address) = Address::try_from_bech32_with_hrp(address)?;
        account_handle.client().bech32_hrp_matches(&bech32_hrp).await?;

        let outputs = vec![
            BasicOutputBuilder::new_with_minimum_storage_deposit(rent_structure)?
                .add_unlock_condition(AddressUnlockCondition::new(address))
                .with_native_tokens(vec![NativeToken::new(
                    TokenId::from_str(&token_id)?,
                    U256::from_dec_str(&amount).map_err(|e| Error::Miscellaneous(e.to_string()))?,
                )?])
                .finish_output(token_supply)?,
        ];

        account_handle.send(outputs, None).await?
    } else {
        // Send native tokens with storage deposit return and expiration
        let outputs = vec![AddressNativeTokens {
            address,
            native_tokens: vec![(
                TokenId::from_str(&token_id)?,
                U256::from_dec_str(&amount).map_err(|e| Error::Miscellaneous(e.to_string()))?,
            )],
            ..Default::default()
        }];
        account_handle.send_native_tokens(outputs, None).await?
    };

    println_log_info!(
        "Native token transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `send-nft` command
pub async fn send_nft_command(account_handle: &AccountHandle, address: String, nft_id: String) -> Result<(), Error> {
    let outputs = vec![AddressAndNftId {
        address,
        nft_id: NftId::from_str(&nft_id)?,
    }];
    let transaction = account_handle.send_nft(outputs, None).await?;

    println_log_info!(
        "Nft transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `sync` command
pub async fn sync_command(account_handle: &AccountHandle) -> Result<(), Error> {
    println_log_info!("Synced: {:#?}", account_handle.sync(None).await?);

    Ok(())
}

/// `transactions` command
pub async fn transactions_command(account_handle: &AccountHandle, show_details: bool) -> Result<(), Error> {
    let mut transactions = account_handle.transactions().await?;
    transactions.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    if transactions.is_empty() {
        println_log_info!("No transactions found");
    } else {
        for (i, tx) in transactions.into_iter().enumerate() {
            if show_details {
                println_log_info!("{:#?}", tx);
            } else {
                let transaction_time = to_utc_date_time(tx.timestamp)?;
                let formatted_time = transaction_time.format("%Y-%m-%d %H:%M:%S").to_string();

                println_log_info!("{:<5}{}\t{}", i, tx.transaction_id.to_string(), formatted_time);
            }
        }
    }

    Ok(())
}

/// `transaction` command
pub async fn transaction_command(account_handle: &AccountHandle, transaction_id_str: &str) -> Result<(), Error> {
    let transaction_id = TransactionId::from_str(transaction_id_str)?;
    let maybe_transaction = account_handle
        .transactions()
        .await?
        .into_iter()
        .find(|tx| tx.transaction_id == transaction_id);

    if let Some(tx) = maybe_transaction {
        println_log_info!("{:#?}", tx);
    } else {
        println_log_info!("No transaction found");
    }

    Ok(())
}

/// `unspent-outputs` command
pub async fn unspent_outputs_command(account_handle: &AccountHandle) -> Result<(), Error> {
    let outputs = account_handle.unspent_outputs(None).await?;

    if outputs.is_empty() {
        println_log_info!("No outputs found");
    } else {
        let output_ids: Vec<OutputId> = outputs.iter().map(|o| o.output_id).collect();
        println_log_info!("Unspent outputs: {output_ids:#?}");
    }

    Ok(())
}

pub async fn vote_command(
    account_handle: &AccountHandle,
    event_id: ParticipationEventId,
    answers: Vec<u8>,
) -> Result<(), Error> {
    let transaction = account_handle.vote(Some(event_id), Some(answers)).await?;

    println_log_info!(
        "Voting transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

pub async fn stop_participating_command(
    account_handle: &AccountHandle,
    event_id: ParticipationEventId,
) -> Result<(), Error> {
    let transaction = account_handle.stop_participating(event_id).await?;

    println_log_info!(
        "Stop participating transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

pub async fn participation_overview_command(
    account_handle: &AccountHandle,
    event_ids: Option<Vec<ParticipationEventId>>,
) -> Result<(), Error> {
    let participation_overview = account_handle.get_participation_overview(event_ids).await?;

    println_log_info!("Participation overview: {participation_overview:?}");

    Ok(())
}

pub async fn voting_power_command(account_handle: &AccountHandle) -> Result<(), Error> {
    let voting_power = account_handle.get_voting_power().await?;

    println_log_info!("Voting power: {voting_power}");

    Ok(())
}

pub async fn increase_voting_power_command(account_handle: &AccountHandle, amount: u64) -> Result<(), Error> {
    let transaction = account_handle.increase_voting_power(amount).await?;

    println_log_info!(
        "Increase voting power transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

pub async fn decrease_voting_power_command(account_handle: &AccountHandle, amount: u64) -> Result<(), Error> {
    let transaction = account_handle.decrease_voting_power(amount).await?;

    println_log_info!(
        "Decrease voting power transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

pub async fn voting_output_command(account_handle: &AccountHandle) -> Result<(), Error> {
    let output = account_handle.get_voting_output().await?;

    println_log_info!("Voting output: {output:?}");

    Ok(())
}

async fn print_address(account_handle: &AccountHandle, address: &AccountAddress) -> Result<(), Error> {
    let mut log = format!("Address {}: {}", address.key_index(), address.address());

    if *address.internal() {
        log = format!("{log}\nChange address");
    }

    let addresses = account_handle.addresses_with_unspent_outputs().await?;
    let current_time = iota_sdk::utils::unix_timestamp_now().as_secs() as u32;

    if let Ok(index) = addresses.binary_search_by_key(&(address.key_index(), address.internal()), |a| {
        (a.key_index(), a.internal())
    }) {
        let mut address_amount = 0;
        for output_id in addresses[index].output_ids() {
            if let Some(output_data) = account_handle.get_output(output_id).await {
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
