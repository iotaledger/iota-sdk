// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod completer;

use std::str::FromStr;

use clap::{CommandFactory, Parser, Subcommand};
use colored::Colorize;
use iota_sdk::{
    client::request_funds_from_faucet,
    crypto::signatures::ed25519::PublicKey,
    types::block::{
        address::{Bech32Address, ToBech32Ext},
        mana::ManaAllotment,
        output::{
            feature::MetadataFeature, unlock_condition::AddressUnlockCondition, AccountId, BasicOutputBuilder,
            FoundryId, NativeToken, NativeTokensBuilder, NftId, Output, OutputId, TokenId,
        },
        payload::signed_transaction::TransactionId,
        slot::SlotIndex,
    },
    utils::ConvertTo,
    wallet::{
        types::OutputData, ConsolidationParams, CreateNativeTokenParams, Error as WalletError, MintNftParams,
        OutputsToClaim, SendNativeTokenParams, SendNftParams, SendParams, SyncOptions, TransactionOptions, Wallet,
    },
    U256,
};
use rustyline::{error::ReadlineError, history::MemHistory, Config, Editor};

use self::completer::WalletCommandHelper;
use crate::{
    error::Error,
    helper::{bytes_from_hex_or_file, to_utc_date_time},
    println_log_error, println_log_info,
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, propagate_version = true)]
pub struct WalletCli {
    #[command(subcommand)]
    pub command: WalletCommand,
}

impl WalletCli {
    pub fn print_help() -> Result<(), Error> {
        Self::command().bin_name("Wallet:").print_help()?;
        Ok(())
    }
}

/// Commands
#[derive(Debug, Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum WalletCommand {
    /// Lists the accounts of the wallet.
    Accounts,
    /// Print the wallet address.
    Address,
    /// Allots mana to an account.
    AllotMana { mana: u64, account_id: Option<AccountId> },
    /// Print the wallet balance.
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
    /// Print details about claimable outputs - if there are any.
    ClaimableOutputs,
    /// Checks if an account is ready to issue a block.
    Congestion { account_id: Option<AccountId> },
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
        /// Metadata key, e.g. --foundry-metadata-key data.
        #[arg(long, default_value = "data")]
        foundry_metadata_key: String,
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
        /// Address the faucet sends the funds to, defaults to the wallet address.
        address: Option<Bech32Address>,
        /// URL of the faucet, default to <http://localhost:8088/api/enqueue>.
        url: Option<String>,
    },
    /// Returns the implicit account creation address of the wallet if it is Ed25519 based.
    ImplicitAccountCreationAddress,
    /// Transitions an implicit account to an account.
    ImplicitAccountTransition {
        /// Identifier of the implicit account output.
        output_id: OutputId,
        public_key: Option<String>,
    },
    /// Lists the implicit accounts of the wallet.
    ImplicitAccounts,
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
        /// Immutable metadata key, e.g. --immutable-metadata-key data.
        #[arg(long, default_value = "data")]
        immutable_metadata_key: String,
        #[arg(long, group = "immutable_metadata")]
        /// Immutable metadata to attach to the NFT, e.g. --immutable-metadata-hex 0xdeadbeef.
        immutable_metadata_hex: Option<String>,
        /// Immutable metadata to attach to the NFT, e.g. --immutable-metadata-file ./nft-immutable-metadata.json.
        #[arg(long, group = "immutable_metadata")]
        immutable_metadata_file: Option<String>,
        /// Metadata key, e.g. --metadata-key data.
        #[arg(long, default_value = "data")]
        metadata_key: String,
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
    /// Get information about currently set node.
    NodeInfo,
    /// Display an output.
    Output {
        /// Selector for output.
        /// Either by ID (e.g. 0xbce525324af12eda02bf7927e92cea3a8e8322d0f41966271443e6c3b245a4400000) or index.
        selector: OutputSelector,
        #[arg(short, long)]
        metadata: bool,
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
        /// default to the wallet address.
        #[arg(long)]
        return_address: Option<Bech32Address>,
        /// Expiration in slot indices, after which the output will be available for the sender again, if not spent by
        /// the receiver already. The expiration will only be used if one is necessary given the provided
        /// amount. If an expiration is needed but not provided, it will default to one day.
        #[arg(long)]
        expiration: Option<SlotIndex>,
        /// Whether to send micro amounts. This will automatically add Storage Deposit Return and Expiration unlock
        /// conditions if necessary. This flag is implied by the existence of a return address or expiration.
        #[arg(long, default_value_t = false)]
        allow_micro_amount: bool,
    },
    /// Send a native token.
    /// This will create an output with an expiration and storage deposit return unlock condition.
    SendNativeToken {
        /// Address to send the native tokens to, e.g. rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3.
        address: Bech32Address,
        /// Token ID to be sent, e.g. 0x087d205988b733d97fb145ae340e27a8b19554d1ceee64574d7e5ff66c45f69e7a0100000000.
        token_id: String,
        /// Amount to send, e.g. 1000000.
        amount: String,
        /// Whether to gift the storage deposit for the output or not, e.g. `true`.
        #[arg(value_parser = clap::builder::BoolishValueParser::new())]
        gift_storage_deposit: Option<bool>,
    },
    /// Send an NFT.
    SendNft {
        /// Address to send the NFT to, e.g. rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3.
        address: Bech32Address,
        /// NFT ID to be sent, e.g. 0xecadf10e6545aa82da4df2dfd2a496b457c8850d2cab49b7464cb273d3dffb07.
        nft_id: String,
    },
    /// Synchronize the wallet.
    Sync,
    /// Show the details of a transaction.
    #[clap(visible_alias = "tx")]
    Transaction {
        /// Selector for transaction.
        /// Either by ID (e.g. 0x84fe6b1796bddc022c9bc40206f0a692f4536b02aa8c13140264e2e01a3b7e4b) or index.
        selector: TransactionSelector,
    },
    /// List the wallet transactions.
    #[clap(visible_alias = "txs")]
    Transactions {
        /// List wallet transactions with all details.
        #[arg(long, default_value_t = false)]
        show_details: bool,
    },
    /// List the unspent outputs.
    UnspentOutputs,
    // /// Cast votes for an event.
    // Vote {
    //     /// Event ID for which to cast votes, e.g.
    // 0xdc049a721dc65ec342f836c876ec15631ed915cd55213cee39e8d1c821c751f2.     event_id: ParticipationEventId,
    //     /// Answers to the event questions.
    //     answers: Vec<u8>,
    // },
    // /// Stop participating to an event.
    // StopParticipating {
    //     /// Event ID for which to stop participation, e.g.
    //     /// 0xdc049a721dc65ec342f836c876ec15631ed915cd55213cee39e8d1c821c751f2.
    //     event_id: ParticipationEventId,
    // },
    // /// Get the participation overview of the wallet.
    // ParticipationOverview {
    //     /// Event IDs for which to get the participation overview, e.g.
    //     /// 0xdc049a721dc65ec342f836c876ec15631ed915cd55213cee39e8d1c821c751f2...
    //     #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    //     event_ids: Vec<ParticipationEventId>,
    // },
    // /// Get the voting power of the wallet.
    // VotingPower,
    // /// Increase the voting power of the wallet.
    // IncreaseVotingPower {
    //     /// Amount to increase the voting power by, e.g. 100.
    //     amount: u64,
    // },
    // /// Decrease the voting power of the wallet.
    // DecreaseVotingPower {
    //     /// Amount to decrease the voting power by, e.g. 100.
    //     amount: u64,
    // },
    // /// Get the voting output of the wallet.
    // VotingOutput,
}

/// Select by transaction ID or list index
#[derive(Debug, Copy, Clone)]
pub enum TransactionSelector {
    Id(TransactionId),
    Index(usize),
}

impl FromStr for TransactionSelector {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Ok(index) = s.parse() {
            Self::Index(index)
        } else {
            Self::Id(s.parse()?)
        })
    }
}

/// Select by output ID or list index
#[derive(Debug, Copy, Clone)]
pub enum OutputSelector {
    Id(OutputId),
    Index(usize),
}

impl FromStr for OutputSelector {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Ok(index) = s.parse() {
            Self::Index(index)
        } else {
            Self::Id(s.parse()?)
        })
    }
}

// `accounts` command
pub async fn accounts_command(wallet: &Wallet) -> Result<(), Error> {
    let wallet_data = wallet.data().await;
    let accounts = wallet_data.accounts();
    let hrp = wallet.client().get_bech32_hrp().await?;

    println_log_info!("Accounts:\n");

    for account in accounts {
        let output_id = account.output_id;
        let account_id = account.output.as_account().account_id_non_null(&output_id);
        let account_address = account_id.to_bech32(hrp);
        let bic = wallet
            .client()
            .get_account_congestion(&account_id)
            .await
            .map(|r| r.block_issuance_credits)
            .ok();

        println_log_info!(
            "{:<16} {output_id}\n{:<16} {account_id}\n{:<16} {account_address}\n{:<16} {bic:?}\n",
            "Output ID:",
            "Account ID:",
            "Account Address:",
            "BIC:"
        );
    }

    Ok(())
}

// `address` command
pub async fn address_command(wallet: &Wallet) -> Result<(), Error> {
    print_wallet_address(wallet).await?;

    Ok(())
}

// `allot-mana` command
pub async fn allot_mana_command(wallet: &Wallet, mana: u64, account_id: Option<AccountId>) -> Result<(), Error> {
    let wallet_data = wallet.data().await;
    let account_id = account_id
        .or_else(|| wallet_data.first_account_id())
        .ok_or(WalletError::AccountNotFound)?;
    drop(wallet_data);

    let transaction = wallet.allot_mana([ManaAllotment::new(account_id, mana)?], None).await?;

    println_log_info!(
        "Mana allotment transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `balance` command
pub async fn balance_command(wallet: &Wallet) -> Result<(), Error> {
    let balance = wallet.balance().await?;
    println_log_info!("{balance:#?}");

    Ok(())
}

// `burn-native-token` command
pub async fn burn_native_token_command(wallet: &Wallet, token_id: String, amount: String) -> Result<(), Error> {
    println_log_info!("Burning native token {token_id} {amount}.");

    let transaction = wallet
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
pub async fn burn_nft_command(wallet: &Wallet, nft_id: String) -> Result<(), Error> {
    println_log_info!("Burning nft {nft_id}.");

    let transaction = wallet.burn(NftId::from_str(&nft_id)?, None).await?;

    println_log_info!(
        "Burning transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `claim` command
pub async fn claim_command(wallet: &Wallet, output_id: Option<String>) -> Result<(), Error> {
    if let Some(output_id) = output_id {
        println_log_info!("Claiming output {output_id}");

        let transaction = wallet.claim_outputs([OutputId::from_str(&output_id)?]).await?;

        println_log_info!(
            "Claiming transaction sent:\n{:?}\n{:?}",
            transaction.transaction_id,
            transaction.block_id
        );
    } else {
        println_log_info!("Claiming outputs.");

        let output_ids = wallet.claimable_outputs(OutputsToClaim::All).await?;

        if output_ids.is_empty() {
            println_log_info!("No outputs available to claim.");
        }

        // Doing chunks of only 60, because we might need to create the double amount of outputs, because of potential
        // storage deposit return unlock conditions and also consider the remainder output.
        for output_ids_chunk in output_ids.chunks(60) {
            let transaction = wallet.claim_outputs(output_ids_chunk.to_vec()).await?;
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
pub async fn claimable_outputs_command(wallet: &Wallet) -> Result<(), Error> {
    let balance = wallet.balance().await?;
    for output_id in balance
        .potentially_locked_outputs()
        .iter()
        .filter_map(|(output_id, unlockable)| unlockable.then_some(output_id))
    {
        let wallet_data = wallet.data().await;
        // Unwrap: for the iterated `OutputId`s this call will always return `Some(...)`.
        let output = &wallet_data.get_output(output_id).unwrap().output;
        let kind = match output {
            Output::Nft(_) => "Nft",
            Output::Basic(_) => "Basic",
            _ => unreachable!(),
        };
        println_log_info!("{output_id:?} ({kind})");

        // TODO https://github.com/iotaledger/iota-sdk/issues/1633
        // if let Some(native_tokens) = output.native_tokens() {
        //     if !native_tokens.is_empty() {
        //         println_log_info!("  - native token amount:");
        //         native_tokens.iter().for_each(|token| {
        //             println_log_info!("    + {} {}", token.amount(), token.token_id());
        //         });
        //     }
        // }

        if let Some(unlock_conditions) = output.unlock_conditions() {
            let deposit_return = unlock_conditions
                .storage_deposit_return()
                .map(|deposit_return| deposit_return.amount())
                .unwrap_or(0);
            let amount = output.amount() - deposit_return;
            println_log_info!("  - base coin amount: {}", amount);

            if let Some(expiration) = unlock_conditions.expiration() {
                let slot_index = wallet.client().get_slot_index().await?;

                if *expiration.slot_index() > *slot_index {
                    println_log_info!("  - expires in {} slot indices", *expiration.slot_index() - *slot_index);
                } else {
                    println_log_info!(
                        "  - expired {} slot indices ago",
                        *slot_index - *expiration.slot_index()
                    );
                }
            }
        }
    }

    Ok(())
}

// `congestion` command
pub async fn congestion_command(wallet: &Wallet, account_id: Option<AccountId>) -> Result<(), Error> {
    let account_id = {
        let wallet_data = wallet.data().await;
        account_id
            .or_else(|| {
                wallet_data
                    .accounts()
                    .next()
                    .map(|o| o.output.as_account().account_id_non_null(&o.output_id))
            })
            .or_else(|| {
                wallet_data
                    .implicit_accounts()
                    .next()
                    .map(|o| AccountId::from(&o.output_id))
            })
            .ok_or(WalletError::AccountNotFound)?
    };

    let congestion = wallet.client().get_account_congestion(&account_id).await?;

    println_log_info!("{congestion:#?}");

    Ok(())
}

// `consolidate` command
pub async fn consolidate_command(wallet: &Wallet) -> Result<(), Error> {
    println_log_info!("Consolidating outputs.");

    let transaction = wallet
        .consolidate_outputs(ConsolidationParams::new().with_force(true))
        .await?;

    println_log_info!(
        "Consolidation transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `create-account-output` command
pub async fn create_account_output_command(wallet: &Wallet) -> Result<(), Error> {
    println_log_info!("Creating account output.");

    let transaction = wallet.create_account_output(None, None).await?;

    println_log_info!(
        "Account output creation transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `create-native-token` command
pub async fn create_native_token_command(
    wallet: &Wallet,
    circulating_supply: String,
    maximum_supply: String,
    foundry_metadata: Option<MetadataFeature>,
) -> Result<(), Error> {
    // If no account output exists, create one first
    if wallet.balance().await?.accounts().is_empty() {
        let transaction = wallet.create_account_output(None, None).await?;
        println_log_info!(
            "Account output minting transaction sent:\n{:?}\n{:?}",
            transaction.transaction_id,
            transaction.block_id
        );
        wallet
            .reissue_transaction_until_included(&transaction.transaction_id, None, None)
            .await?;
        // Sync wallet after the transaction got confirmed, so the account output is available
        wallet.sync(None).await?;
    }

    let params = CreateNativeTokenParams {
        account_id: None,
        circulating_supply: U256::from_dec_str(&circulating_supply).map_err(|e| Error::Miscellaneous(e.to_string()))?,
        maximum_supply: U256::from_dec_str(&maximum_supply).map_err(|e| Error::Miscellaneous(e.to_string()))?,
        foundry_metadata,
    };

    let create_transaction = wallet.create_native_token(params, None).await?;

    println_log_info!(
        "Transaction to create native token sent:\n{:?}\n{:?}",
        create_transaction.transaction.transaction_id,
        create_transaction.transaction.block_id
    );

    Ok(())
}

// `destroy-account` command
pub async fn destroy_account_command(wallet: &Wallet, account_id: String) -> Result<(), Error> {
    println_log_info!("Destroying account {account_id}.");

    let transaction = wallet.burn(AccountId::from_str(&account_id)?, None).await?;

    println_log_info!(
        "Destroying account transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `destroy-foundry` command
pub async fn destroy_foundry_command(wallet: &Wallet, foundry_id: String) -> Result<(), Error> {
    println_log_info!("Destroying foundry {foundry_id}.");

    let transaction = wallet.burn(FoundryId::from_str(&foundry_id)?, None).await?;

    println_log_info!(
        "Destroying foundry transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `faucet` command
pub async fn faucet_command(wallet: &Wallet, address: Option<Bech32Address>, url: Option<String>) -> Result<(), Error> {
    let address = if let Some(address) = address {
        address
    } else {
        wallet.address().await
    };

    let faucet_url = url.as_deref().unwrap_or("http://localhost:8088/api/enqueue");
    let response = request_funds_from_faucet(faucet_url, &address).await?;

    println_log_info!("{response}");

    Ok(())
}

// `implicit-account-creation-address` command
pub async fn implicit_account_creation_address_command(wallet: &Wallet) -> Result<(), Error> {
    let address = wallet.implicit_account_creation_address().await?;

    println_log_info!("{address}");

    Ok(())
}

// `implicit-account-transition` command
pub async fn implicit_account_transition_command(
    wallet: &Wallet,
    output_id: OutputId,
    public_key: Option<String>,
) -> Result<(), Error> {
    let public_key = public_key
        .map(|s| {
            PublicKey::try_from_bytes(prefix_hex::decode(s).map_err(|e| Error::Miscellaneous(e.to_string()))?)
                .map_err(|e| Error::Miscellaneous(e.to_string()))
        })
        .transpose()?;
    let transaction = wallet.implicit_account_transition(&output_id, public_key).await?;

    println_log_info!(
        "Implicit account transition transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `implicit-accounts` command
pub async fn implicit_accounts_command(wallet: &Wallet) -> Result<(), Error> {
    let wallet_data = wallet.data().await;
    let implicit_accounts = wallet_data.implicit_accounts();
    let hrp = wallet.client().get_bech32_hrp().await?;

    println_log_info!("Implicit accounts:\n");

    for implicit_account in implicit_accounts {
        let output_id = implicit_account.output_id;
        let account_id = AccountId::from(&output_id);
        let account_address = account_id.to_bech32(hrp);
        let bic = wallet
            .client()
            .get_account_congestion(&account_id)
            .await
            .map(|r| r.block_issuance_credits)
            .ok();

        println_log_info!(
            "{:<16} {output_id}\n{:<16} {account_id}\n{:<16} {account_address}\n{:<16} {bic:?}\n",
            "Output ID:",
            "Account ID:",
            "Account Address:",
            "BIC:"
        );
    }

    Ok(())
}

// `melt-native-token` command
pub async fn melt_native_token_command(wallet: &Wallet, token_id: String, amount: String) -> Result<(), Error> {
    let transaction = wallet
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
pub async fn mint_native_token_command(wallet: &Wallet, token_id: String, amount: String) -> Result<(), Error> {
    let mint_transaction = wallet
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
    wallet: &Wallet,
    address: Option<Bech32Address>,
    immutable_metadata_key: String,
    immutable_metadata: Option<Vec<u8>>,
    metadata_key: String,
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

    let mut nft_options = MintNftParams::new()
        .with_address(address)
        .with_tag(tag)
        .with_sender(sender)
        .with_issuer(issuer);

    if let Some(metadata) = metadata {
        nft_options = nft_options.with_metadata(MetadataFeature::new([(metadata_key, metadata)])?);
    }
    if let Some(immutable_metadata) = immutable_metadata {
        nft_options =
            nft_options.with_immutable_metadata(MetadataFeature::new([(immutable_metadata_key, immutable_metadata)])?);
    }

    let transaction = wallet.mint_nfts([nft_options], None).await?;

    println_log_info!(
        "NFT minting transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `node-info` command
pub async fn node_info_command(wallet: &Wallet) -> Result<(), Error> {
    let node_info = serde_json::to_string_pretty(&wallet.client().get_info().await?)?;

    println_log_info!("Current node info: {node_info}");

    Ok(())
}

/// `output` command
pub async fn output_command(wallet: &Wallet, selector: OutputSelector, metadata: bool) -> Result<(), Error> {
    let wallet_data = wallet.data().await;
    let output = match selector {
        OutputSelector::Id(id) => wallet_data.get_output(&id),
        OutputSelector::Index(index) => {
            let mut outputs = wallet_data.outputs().values().collect::<Vec<_>>();
            outputs.sort_unstable_by_key(|o| o.output_id);
            outputs.into_iter().nth(index)
        }
    };

    if let Some(output) = output {
        if metadata {
            println_log_info!("{output:#?}");
        } else {
            println_log_info!("{:#?}", output.output);
        }
    } else {
        println_log_info!("Output not found");
    }

    Ok(())
}

/// `outputs` command
pub async fn outputs_command(wallet: &Wallet) -> Result<(), Error> {
    print_outputs(wallet.data().await.outputs().values().cloned().collect(), "Outputs:")
}

// `send` command
pub async fn send_command(
    wallet: &Wallet,
    address: impl ConvertTo<Bech32Address>,
    amount: u64,
    return_address: Option<impl ConvertTo<Bech32Address>>,
    expiration: Option<SlotIndex>,
    allow_micro_amount: bool,
) -> Result<(), Error> {
    let params = [SendParams::new(amount, address)?
        .with_return_address(return_address.map(ConvertTo::convert).transpose()?)
        .with_expiration(expiration)];
    let transaction = wallet
        .send_with_params(
            params,
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
    wallet: &Wallet,
    address: impl ConvertTo<Bech32Address>,
    token_id: String,
    amount: String,
    gift_storage_deposit: Option<bool>,
) -> Result<(), Error> {
    let address = address.convert()?;
    let transaction = if gift_storage_deposit.unwrap_or(false) {
        // Send native tokens together with the required storage deposit
        let storage_params = wallet.client().get_storage_score_parameters().await?;

        wallet.client().bech32_hrp_matches(address.hrp()).await?;

        let outputs = [BasicOutputBuilder::new_with_minimum_amount(storage_params)
            .add_unlock_condition(AddressUnlockCondition::new(address))
            .with_native_token(NativeToken::new(
                TokenId::from_str(&token_id)?,
                U256::from_dec_str(&amount).map_err(|e| Error::Miscellaneous(e.to_string()))?,
            )?)
            .finish_output()?];

        wallet.send_outputs(outputs, None).await?
    } else {
        // Send native tokens with storage deposit return and expiration
        let outputs = [SendNativeTokenParams::new(
            address,
            (
                TokenId::from_str(&token_id)?,
                U256::from_dec_str(&amount).map_err(|e| Error::Miscellaneous(e.to_string()))?,
            ),
        )?];
        wallet.send_native_tokens(outputs, None).await?
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
    wallet: &Wallet,
    address: impl ConvertTo<Bech32Address>,
    nft_id: String,
) -> Result<(), Error> {
    let outputs = [SendNftParams::new(address.convert()?, &nft_id)?];
    let transaction = wallet.send_nft(outputs, None).await?;

    println_log_info!(
        "Nft transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `sync` command
pub async fn sync_command(wallet: &Wallet) -> Result<(), Error> {
    let balance = wallet
        .sync(Some(SyncOptions {
            sync_native_token_foundries: true,
            sync_implicit_accounts: true,
            ..Default::default()
        }))
        .await?;
    println_log_info!("Synced.");
    println_log_info!("{balance:#?}");

    Ok(())
}

/// `transaction` command
pub async fn transaction_command(wallet: &Wallet, selector: TransactionSelector) -> Result<(), Error> {
    let wallet_data = wallet.data().await;
    let transaction = match selector {
        TransactionSelector::Id(id) => wallet_data.get_transaction(&id),
        TransactionSelector::Index(index) => {
            let mut transactions = wallet_data.transactions().values().collect::<Vec<_>>();
            transactions.sort_unstable_by(|a, b| b.timestamp.cmp(&a.timestamp));
            transactions.into_iter().nth(index)
        }
    };

    if let Some(tx) = transaction {
        println_log_info!("{:#?}", tx);
    } else {
        println_log_info!("No transaction found");
    }

    Ok(())
}

/// `transactions` command
pub async fn transactions_command(wallet: &Wallet, show_details: bool) -> Result<(), Error> {
    let wallet_data = wallet.data().await;
    let mut transactions = wallet_data.transactions().values().collect::<Vec<_>>();
    transactions.sort_unstable_by(|a, b| b.timestamp.cmp(&a.timestamp));

    if transactions.is_empty() {
        println_log_info!("No transactions found");
    } else {
        for (i, tx) in transactions.into_iter().enumerate() {
            if show_details {
                println_log_info!("{:#?}", tx);
            } else {
                let transaction_time = to_utc_date_time(tx.timestamp)?;
                let formatted_time = transaction_time.format("%Y-%m-%d %H:%M:%S UTC").to_string();

                println_log_info!("{:<5}{}\t{}", i, tx.transaction_id, formatted_time);
            }
        }
    }

    Ok(())
}

/// `unspent-outputs` command
pub async fn unspent_outputs_command(wallet: &Wallet) -> Result<(), Error> {
    print_outputs(
        wallet.data().await.unspent_outputs().values().cloned().collect(),
        "Unspent outputs:",
    )
}

// pub async fn vote_command(wallet: &Wallet, event_id: ParticipationEventId, answers: Vec<u8>) -> Result<(), Error> {
//     let transaction = wallet.vote(Some(event_id), Some(answers)).await?;

//     println_log_info!(
//         "Voting transaction sent:\n{:?}\n{:?}",
//         transaction.transaction_id,
//         transaction.block_id
//     );

//     Ok(())
// }

// pub async fn stop_participating_command(wallet: &Wallet, event_id: ParticipationEventId) -> Result<(), Error> {
//     let transaction = wallet.stop_participating(event_id).await?;

//     println_log_info!(
//         "Stop participating transaction sent:\n{:?}\n{:?}",
//         transaction.transaction_id,
//         transaction.block_id
//     );

//     Ok(())
// }

// pub async fn participation_overview_command(
//     wallet: &Wallet,
//     event_ids: Option<Vec<ParticipationEventId>>,
// ) -> Result<(), Error> {
//     let participation_overview = wallet.get_participation_overview(event_ids).await?;

//     println_log_info!("Participation overview: {participation_overview:?}");

//     Ok(())
// }

// pub async fn voting_power_command(wallet: &Wallet) -> Result<(), Error> {
//     let voting_power = wallet.get_voting_power().await?;

//     println_log_info!("Voting power: {voting_power}");

//     Ok(())
// }

// pub async fn increase_voting_power_command(wallet: &Wallet, amount: u64) -> Result<(), Error> {
//     let transaction = wallet.increase_voting_power(amount).await?;

//     println_log_info!(
//         "Increase voting power transaction sent:\n{:?}\n{:?}",
//         transaction.transaction_id,
//         transaction.block_id
//     );

//     Ok(())
// }

// pub async fn decrease_voting_power_command(wallet: &Wallet, amount: u64) -> Result<(), Error> {
//     let transaction = wallet.decrease_voting_power(amount).await?;

//     println_log_info!(
//         "Decrease voting power transaction sent:\n{:?}\n{:?}",
//         transaction.transaction_id,
//         transaction.block_id
//     );

//     Ok(())
// }

// pub async fn voting_output_command(wallet: &Wallet) -> Result<(), Error> {
//     let output = wallet.get_voting_output().await?;

//     println_log_info!("Voting output: {output:?}");

//     Ok(())
// }

async fn print_wallet_address(wallet: &Wallet) -> Result<(), Error> {
    let address = wallet.address().await;

    let mut log = format!(
        "Address:\n{:<9}{}\n{:<9}{:?}",
        "Bech32:",
        address,
        "Hex:",
        address.inner()
    );

    let slot_index = wallet.client().get_slot_index().await?;
    let protocol_parameters = wallet.client().get_protocol_parameters().await?;

    let mut output_ids = Vec::new();
    let mut amount = 0;
    let mut native_tokens = NativeTokensBuilder::new();
    let mut accounts = Vec::new();
    let mut foundries = Vec::new();
    let mut nfts = Vec::new();
    let mut delegations = Vec::new();
    let mut anchors = Vec::new();

    for output_data in wallet.data().await.unspent_outputs().values() {
        let output_id = output_data.output_id;
        output_ids.push(output_id);

        // Output might be associated with the address, but can't be unlocked by it, so we check that here.
        let required_address = &output_data
            .output
            .required_address(slot_index, protocol_parameters.committable_age_range())?;

        if required_address
            .as_ref()
            .is_some_and(|required_address| required_address == address.inner())
        {
            if let Some(nt) = output_data.output.native_token() {
                native_tokens.add_native_token(*nt)?;
            }
            match &output_data.output {
                Output::Basic(_) => {}
                Output::Account(account) => accounts.push(account.account_id_non_null(&output_id)),
                Output::Foundry(foundry) => foundries.push(foundry.id()),
                Output::Nft(nft) => nfts.push(nft.nft_id_non_null(&output_id)),
                Output::Delegation(delegation) => delegations.push(delegation.delegation_id_non_null(&output_id)),
                Output::Anchor(anchor) => anchors.push(anchor.anchor_id_non_null(&output_id)),
            }
            let unlock_conditions = output_data
                .output
                .unlock_conditions()
                .expect("output must have unlock conditions");
            let sdr_amount = unlock_conditions
                .storage_deposit_return()
                .map(|sdr| sdr.amount())
                .unwrap_or(0);

            amount += output_data.output.amount() - sdr_amount;
        }
    }

    let bip_path = wallet.bip_path().await;
    log = format!("{log}\nBIP path: {bip_path:?}");

    log = format!(
        "{log}\nOutputs: {:#?}\nBase coin amount: {}\nNative Tokens: {:?}\nAccounts: {:?}\nFoundries: {:?}\nNFTs: {:?}\nDelegations: {:?}\nAnchors: {:?}\n",
        output_ids,
        amount,
        native_tokens.finish_vec()?,
        accounts,
        foundries,
        nfts,
        delegations,
        anchors
    );

    println_log_info!("{log}");

    Ok(())
}

// loop on the wallet prompt
pub async fn prompt(wallet: &Wallet) -> Result<(), Error> {
    let config = Config::builder()
        .auto_add_history(true)
        .history_ignore_space(true)
        .completion_type(rustyline::CompletionType::List)
        .edit_mode(rustyline::EditMode::Emacs)
        .build();

    let mut rl = Editor::with_history(config, MemHistory::with_config(config))?;
    rl.set_helper(Some(WalletCommandHelper::default()));

    loop {
        match prompt_internal(wallet, &mut rl).await {
            Ok(res) => match res {
                PromptResponse::Reprompt => (),
                PromptResponse::Done => {
                    return Ok(());
                }
            },
            Err(e) => {
                println_log_error!("{e}");
            }
        }
    }
}

pub enum PromptResponse {
    Reprompt,
    Done,
}

pub async fn prompt_internal(
    wallet: &Wallet,
    rl: &mut Editor<WalletCommandHelper, MemHistory>,
) -> Result<PromptResponse, Error> {
    let prompt = if let Some(alias) = wallet.alias().await {
        format!("Wallet \"{alias}\": ")
    } else {
        String::from("Wallet: ")
    };

    if let Some(helper) = rl.helper_mut() {
        helper.set_prompt(prompt.green().to_string());
    }

    let input = rl.readline(&prompt);
    match input {
        Ok(command) => {
            match command.trim() {
                "" => {}
                "h" | "help" => WalletCli::print_help()?,
                "c" | "clear" => {
                    // Clear console
                    let _ = std::process::Command::new("clear").status();
                }
                _ => {
                    // Prepend `Wallet: ` so the parsing will be correct
                    let command = format!("Wallet: {command}");
                    let protocol_cli = match WalletCli::try_parse_from(command.split_whitespace()) {
                        Ok(protocol_cli) => protocol_cli,
                        Err(err) => {
                            println!("{err}");
                            return Ok(PromptResponse::Reprompt);
                        }
                    };
                    match protocol_cli.command {
                        WalletCommand::Accounts => accounts_command(wallet).await,
                        WalletCommand::Address => address_command(wallet).await,
                        WalletCommand::AllotMana { mana, account_id } => {
                            allot_mana_command(wallet, mana, account_id).await
                        }
                        WalletCommand::Balance => balance_command(wallet).await,
                        WalletCommand::BurnNativeToken { token_id, amount } => {
                            burn_native_token_command(wallet, token_id, amount).await
                        }
                        WalletCommand::BurnNft { nft_id } => burn_nft_command(wallet, nft_id).await,
                        WalletCommand::Claim { output_id } => claim_command(wallet, output_id).await,
                        WalletCommand::ClaimableOutputs => claimable_outputs_command(wallet).await,
                        WalletCommand::Congestion { account_id } => congestion_command(wallet, account_id).await,
                        WalletCommand::Consolidate => consolidate_command(wallet).await,
                        WalletCommand::CreateAccountOutput => create_account_output_command(wallet).await,
                        WalletCommand::CreateNativeToken {
                            circulating_supply,
                            maximum_supply,
                            foundry_metadata_key,
                            foundry_metadata_hex,
                            foundry_metadata_file,
                        } => {
                            create_native_token_command(
                                wallet,
                                circulating_supply,
                                maximum_supply,
                                bytes_from_hex_or_file(foundry_metadata_hex, foundry_metadata_file)
                                    .await?
                                    .map(|d| MetadataFeature::new([(foundry_metadata_key, d)]))
                                    .transpose()?,
                            )
                            .await
                        }
                        WalletCommand::DestroyAccount { account_id } => {
                            destroy_account_command(wallet, account_id).await
                        }
                        WalletCommand::DestroyFoundry { foundry_id } => {
                            destroy_foundry_command(wallet, foundry_id).await
                        }
                        WalletCommand::Exit => {
                            return Ok(PromptResponse::Done);
                        }
                        WalletCommand::Faucet { address, url } => faucet_command(wallet, address, url).await,
                        WalletCommand::ImplicitAccountCreationAddress => {
                            implicit_account_creation_address_command(wallet).await
                        }
                        WalletCommand::ImplicitAccountTransition { output_id, public_key } => {
                            implicit_account_transition_command(wallet, output_id, public_key).await
                        }
                        WalletCommand::ImplicitAccounts => implicit_accounts_command(wallet).await,
                        WalletCommand::MeltNativeToken { token_id, amount } => {
                            melt_native_token_command(wallet, token_id, amount).await
                        }
                        WalletCommand::MintNativeToken { token_id, amount } => {
                            mint_native_token_command(wallet, token_id, amount).await
                        }
                        WalletCommand::MintNft {
                            address,
                            immutable_metadata_key,
                            immutable_metadata_hex,
                            immutable_metadata_file,
                            metadata_key,
                            metadata_hex,
                            metadata_file,
                            tag,
                            sender,
                            issuer,
                        } => {
                            mint_nft_command(
                                wallet,
                                address,
                                immutable_metadata_key,
                                bytes_from_hex_or_file(immutable_metadata_hex, immutable_metadata_file).await?,
                                metadata_key,
                                bytes_from_hex_or_file(metadata_hex, metadata_file).await?,
                                tag,
                                sender,
                                issuer,
                            )
                            .await
                        }
                        WalletCommand::NodeInfo => node_info_command(wallet).await,
                        WalletCommand::Output { selector, metadata } => {
                            output_command(wallet, selector, metadata).await
                        }
                        WalletCommand::Outputs => outputs_command(wallet).await,
                        WalletCommand::Send {
                            address,
                            amount,
                            return_address,
                            expiration,
                            allow_micro_amount,
                        } => {
                            let allow_micro_amount = if return_address.is_some() || expiration.is_some() {
                                true
                            } else {
                                allow_micro_amount
                            };
                            send_command(wallet, address, amount, return_address, expiration, allow_micro_amount).await
                        }
                        WalletCommand::SendNativeToken {
                            address,
                            token_id,
                            amount,
                            gift_storage_deposit,
                        } => send_native_token_command(wallet, address, token_id, amount, gift_storage_deposit).await,
                        WalletCommand::SendNft { address, nft_id } => send_nft_command(wallet, address, nft_id).await,
                        WalletCommand::Sync => sync_command(wallet).await,
                        WalletCommand::Transaction { selector } => transaction_command(wallet, selector).await,
                        WalletCommand::Transactions { show_details } => {
                            transactions_command(wallet, show_details).await
                        }
                        WalletCommand::UnspentOutputs => unspent_outputs_command(wallet).await,
                        // WalletCommand::Vote { event_id, answers } => vote_command(wallet, event_id, answers).await,
                        // WalletCommand::StopParticipating { event_id } => {
                        //     stop_participating_command(wallet, event_id).await
                        // }
                        // WalletCommand::ParticipationOverview { event_ids } => {
                        //     let event_ids = (!event_ids.is_empty()).then_some(event_ids);
                        //     participation_overview_command(wallet, event_ids).await
                        // }
                        // WalletCommand::VotingPower => voting_power_command(wallet).await,
                        // WalletCommand::IncreaseVotingPower { amount } => {
                        //     increase_voting_power_command(wallet, amount).await
                        // }
                        // WalletCommand::DecreaseVotingPower { amount } => {
                        //     decrease_voting_power_command(wallet, amount).await
                        // }
                        // WalletCommand::VotingOutput => voting_output_command(wallet).await,
                    }
                    .unwrap_or_else(|err| {
                        println_log_error!("{err}");
                    });
                }
            }
        }
        Err(ReadlineError::Interrupted) => {
            return Ok(PromptResponse::Done);
        }
        Err(err) => {
            println_log_error!("{err}");
        }
    }

    Ok(PromptResponse::Reprompt)
}

fn print_outputs(mut outputs: Vec<OutputData>, title: &str) -> Result<(), Error> {
    if outputs.is_empty() {
        println_log_info!("No outputs found");
    } else {
        println_log_info!("{title}");
        outputs.sort_unstable_by_key(|o| o.output_id);

        for (i, output_data) in outputs.into_iter().enumerate() {
            let kind_str = if output_data.output.is_implicit_account() {
                "ImplicitAccount"
            } else {
                output_data.output.kind_str()
            };

            println_log_info!(
                "{:<5}{} {:<16}{}",
                i,
                &output_data.output_id,
                kind_str,
                if output_data.is_spent { "Spent" } else { "Unspent" },
            );
        }
    }

    Ok(())
}
