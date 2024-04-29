// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod completer;

use std::str::FromStr;

use clap::{CommandFactory, Parser, Subcommand};
use colored::Colorize;
use eyre::Error;
use iota_sdk::{
    client::{api::options::TransactionOptions, request_funds_from_faucet, secret::SecretManager},
    types::block::{
        address::{AccountAddress, Bech32Address, ToBech32Ext},
        mana::ManaAllotment,
        output::{
            feature::{BlockIssuerKeySource, Ed25519PublicKeyHashBlockIssuerKey, MetadataFeature},
            unlock_condition::AddressUnlockCondition,
            AccountId, BasicOutputBuilder, DelegationId, FoundryId, NativeToken, NativeTokensBuilder, NftId, Output,
            OutputId, TokenId,
        },
        payload::signed_transaction::TransactionId,
        slot::{EpochIndex, SlotIndex},
        IdentifierError,
    },
    utils::ConvertTo,
    wallet::{
        types::OutputData, BeginStakingParams, ConsolidationParams, CreateDelegationParams, CreateNativeTokenParams,
        MintNftParams, ModifyAccountBlockIssuerKey, OutputsToClaim, ReturnStrategy, SendManaParams,
        SendNativeTokenParams, SendNftParams, SendParams, SyncOptions, Wallet, WalletError,
    },
    U256,
};
use rustyline::{error::ReadlineError, history::MemHistory, Config, Editor};

use self::completer::WalletCommandHelper;
use crate::{
    helper::{bytes_from_hex_or_file, enter_password, to_utc_date_time},
    println_log_error, println_log_info,
};

const DEFAULT_FAUCET_URL: &str = "http://localhost:8088/api/enqueue";

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
#[derive(Debug, Subcommand, strum::VariantNames)]
#[strum(serialize_all = "kebab-case")]
#[allow(clippy::large_enum_variant)]
pub enum WalletCommand {
    /// Lists the accounts of the wallet.
    Accounts,
    /// Print the wallet address.
    Address,
    /// Allots mana to an account.
    AllotMana { mana: u64, account_id: Option<AccountId> },
    /// Announces that a staking account wants to be a validator for the current epoch.
    AnnounceCandidacy {
        /// The account ID which will announce its candidacy to be a validator.
        account_id: AccountId,
    },
    /// Print the wallet balance.
    Balance,
    BeginStaking {
        /// The Account ID which will begin staking.
        account_id: AccountId,
        /// The amount of tokens to stake.
        staked_amount: u64,
        /// The fixed cost of the validator, which it receives as part of its Mana rewards.
        fixed_cost: u64,
        /// The staking period (in epochs). Will default to the staking unbonding period.
        staking_period: Option<u32>,
    },
    /// Burn an amount of native token.
    BurnNativeToken {
        /// Token ID to be burnt, e.g. 0x087d205988b733d97fb145ae340e27a8b19554d1ceee64574d7e5ff66c45f69e7a0100000000.
        token_id: TokenId,
        /// Amount to be burnt, e.g. 100.
        #[arg(value_parser = parse_u256)]
        amount: U256,
    },
    /// Burn an NFT.
    BurnNft {
        /// NFT ID to be burnt, e.g. 0xecadf10e6545aa82da4df2dfd2a496b457c8850d2cab49b7464cb273d3dffb07.
        nft_id: NftId,
    },
    /// Claim outputs with storage deposit return, expiration or timelock unlock conditions.
    Claim {
        /// Output ID to be claimed.
        output_id: Option<OutputId>,
    },
    /// Print details about claimable outputs - if there are any.
    ClaimableOutputs,
    /// Get the committee for the given epoch.
    Committee { epoch: Option<EpochIndex> },
    /// Checks if an account is ready to issue a block.
    Congestion {
        account_id: Option<AccountId>,
        work_score: Option<u32>,
    },
    /// Consolidate all basic outputs into one address.
    Consolidate,
    /// Create a new account output.
    CreateAccountOutput,
    /// Create a delegation.
    CreateDelegation {
        /// The amount to delegate.
        delegated_amount: u64,
        /// The account ID of the validator.
        validator_account_id: AccountId,
        /// The address that will control the delegation. Defaults to the wallet address.
        address: Option<Bech32Address>,
    },
    /// Create a native token.
    CreateNativeToken {
        /// Circulating supply of the native token to be minted, e.g. 100.
        #[arg(value_parser = parse_u256)]
        circulating_supply: U256,
        /// Maximum supply of the native token to be minted, e.g. 500.
        #[arg(value_parser = parse_u256)]
        maximum_supply: U256,
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
    /// Delay the claiming of a delegation.
    DelayDelegationClaiming {
        /// ID of the delegation to be delayed.
        delegation_id: DelegationId,
        /// Whether excess amount above the minimum storage requirement should be reclaimed.
        /// Otherwise the excess will be transferred into a new delegation.
        reclaim_excess: bool,
    },
    /// Destroy an account output.
    DestroyAccount {
        /// Account ID to be destroyed, e.g. 0xed5a90106ae5d402ebaecb9ba36f32658872df789f7a29b9f6d695b912ec6a1e.
        account_id: AccountId,
    },
    /// Destroy a delegation.
    DestroyDelegation {
        /// ID of the delegation to be destroyed.
        delegation_id: DelegationId,
    },
    /// Destroy a foundry.
    DestroyFoundry {
        /// Foundry ID to be destroyed, e.g.
        /// 0x08cb54928954c3eb7ece1bf1cc0c68eb179dc1c4634ae5d23df1c70643d0911c3d0200000000.
        foundry_id: FoundryId,
    },
    /// End a staking and claim the rewards.
    EndStaking {
        /// The Account ID of the staking account.
        account_id: AccountId,
    },
    /// Exit the CLI wallet.
    Exit,
    /// Extend a staking by some additional epochs.
    ExtendStaking {
        /// The Account ID of the staking account.
        account_id: AccountId,
        /// The number of additional epochs to add to the staking period.
        additional_epochs: u32,
    },
    /// Request funds from the faucet.
    Faucet {
        /// Address the faucet sends the funds to. If not provided, the command defaults to the wallet address.
        address: Option<Bech32Address>,
        /// URL of the faucet.
        #[arg(short, long, value_name = "URL", env = "FAUCET_URL", default_value = DEFAULT_FAUCET_URL)]
        url: String,
    },
    /// Returns the implicit account creation address of the wallet if it is Ed25519 based.
    ImplicitAccountCreationAddress,
    /// Transitions an implicit account to an account.
    ImplicitAccountTransition {
        /// Identifier of the implicit account output.
        output_id: OutputId,
    },
    /// Lists the implicit accounts of the wallet.
    ImplicitAccounts,
    /// Adds a block issuer key to an account.
    AddBlockIssuerKey {
        /// The account to which the key should be added.
        account_id: AccountId,
        /// The hex-encoded public key to add.
        // TODO: Use the actual type somehow?
        block_issuer_key: String,
    },
    /// Removes a block issuer key from an account.
    RemoveBlockIssuerKey {
        /// The account from which the key should be removed.
        account_id: AccountId,
        /// The hex-encoded public key to remove.
        // TODO: Use the actual type somehow?
        block_issuer_key: String,
    },
    /// Mint additional native tokens.
    MintNativeToken {
        /// Token ID to be minted, e.g. 0x087d205988b733d97fb145ae340e27a8b19554d1ceee64574d7e5ff66c45f69e7a0100000000.
        token_id: TokenId,
        /// Amount to be minted, e.g. 100.
        #[arg(value_parser = parse_u256)]
        amount: U256,
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
        token_id: TokenId,
        /// Amount to be melted, e.g. 100.
        #[arg(value_parser = parse_u256)]
        amount: U256,
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
    /// Send mana.
    SendMana {
        /// Recipient address, e.g. rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3.
        address: Bech32Address,
        /// Amount of mana to send, e.g. 1000000.
        mana: u64,
        /// Whether to gift the storage deposit or not.
        #[arg(short, long, default_value_t = false)]
        gift: bool,
    },
    /// Send a native token.
    /// This will create an output with an expiration and storage deposit return unlock condition.
    SendNativeToken {
        /// Address to send the native tokens to, e.g. rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3.
        address: Bech32Address,
        /// Token ID to be sent, e.g. 0x087d205988b733d97fb145ae340e27a8b19554d1ceee64574d7e5ff66c45f69e7a0100000000.
        token_id: TokenId,
        /// Amount to send, e.g. 1000000.
        #[arg(value_parser = parse_u256)]
        amount: U256,
        /// Whether to gift the storage deposit for the output or not, e.g. `true`.
        #[arg(short, long)]
        gift_storage_deposit: Option<bool>,
    },
    /// Send an NFT.
    SendNft {
        /// Address to send the NFT to, e.g. rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3.
        address: Bech32Address,
        /// NFT ID to be sent, e.g. 0xecadf10e6545aa82da4df2dfd2a496b457c8850d2cab49b7464cb273d3dffb07.
        nft_id: NftId,
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
    /// Get information of a validator.
    Validator { account_id: AccountId },
    /// List all validators known to the node.
    Validators,
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

fn parse_u256(s: &str) -> Result<U256, Error> {
    Ok(U256::from_dec_str(s)?)
}

/// Select by transaction ID or list index
#[derive(Debug, Copy, Clone)]
pub enum TransactionSelector {
    Id(TransactionId),
    Index(usize),
}

impl FromStr for TransactionSelector {
    type Err = IdentifierError;

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
    type Err = IdentifierError;

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
    let wallet_ledger = wallet.ledger().await;
    let accounts = wallet_ledger.accounts();
    let hrp = wallet.client().get_bech32_hrp().await?;

    println_log_info!("Accounts:\n");

    for account in accounts {
        let output_id = account.output_id;
        let account_id = account.output.as_account().account_id_non_null(&output_id);
        let account_address = account_id.to_bech32(hrp);
        let bic = wallet
            .client()
            .get_account_congestion(&account_id, None)
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
    let account_id = match account_id {
        Some(account_id) => account_id,
        None => wallet
            .first_block_issuer_account_id()
            .await?
            .ok_or(WalletError::AccountNotFound)?,
    };

    let transaction = wallet.allot_mana([ManaAllotment::new(account_id, mana)?], None).await?;

    println_log_info!(
        "Mana allotment transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `announce-candidacy` command
pub async fn announce_candidacy_command(wallet: &Wallet, account_id: AccountId) -> Result<(), Error> {
    println_log_info!("Announcing candidacy for account {account_id}");

    let block_id = wallet.announce_candidacy(account_id).await?;

    println_log_info!("Block submitted: {block_id}");
    Ok(())
}

// `balance` command
pub async fn balance_command(wallet: &Wallet) -> Result<(), Error> {
    let balance = wallet.balance().await?;
    println_log_info!("{balance:#?}");

    Ok(())
}

// `begin-staking` command
pub async fn begin_staking_command(
    wallet: &Wallet,
    account_id: AccountId,
    staked_amount: u64,
    fixed_cost: u64,
    staking_period: Option<u32>,
) -> Result<(), Error> {
    println_log_info!("Begin staking for {account_id}.");

    let transaction = wallet
        .begin_staking(
            BeginStakingParams {
                account_id,
                staked_amount,
                fixed_cost,
                staking_period,
            },
            None,
        )
        .await?;

    println_log_info!(
        "Begin staking transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `burn-native-token` command
pub async fn burn_native_token_command(wallet: &Wallet, token_id: TokenId, amount: U256) -> Result<(), Error> {
    println_log_info!("Burning native token {token_id} {amount}.");

    let transaction = wallet.burn(NativeToken::new(token_id, amount)?, None).await?;

    println_log_info!(
        "Burning transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `burn-nft` command
pub async fn burn_nft_command(wallet: &Wallet, nft_id: NftId) -> Result<(), Error> {
    println_log_info!("Burning nft {nft_id}.");

    let transaction = wallet.burn(nft_id, None).await?;

    println_log_info!(
        "Burning transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `claim` command
pub async fn claim_command(wallet: &Wallet, output_id: Option<OutputId>) -> Result<(), Error> {
    if let Some(output_id) = output_id {
        println_log_info!("Claiming output {output_id}");

        let transaction = wallet.claim_outputs([output_id]).await?;

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
    for output_id in wallet.claimable_outputs(OutputsToClaim::All).await? {
        let wallet_ledger = wallet.ledger().await;
        // Unwrap: for the iterated `OutputId`s this call will always return `Some(...)`.
        let output = &wallet_ledger.get_output(&output_id).unwrap().output;
        let kind = match output {
            Output::Nft(_) => "Nft",
            Output::Basic(_) => "Basic",
            _ => unreachable!(),
        };
        println_log_info!("{output_id:?} ({kind})");

        if let Some(native_token) = output.native_token() {
            println_log_info!("  - native token amount:");
            println_log_info!("    + {} {}", native_token.amount(), native_token.token_id());
        }

        let deposit_return = output
            .unlock_conditions()
            .storage_deposit_return()
            .map(|deposit_return| deposit_return.amount())
            .unwrap_or(0);
        let amount = output.amount() - deposit_return;
        println_log_info!("  - base coin amount: {}", amount);

        if let Some(expiration) = output.unlock_conditions().expiration() {
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

    Ok(())
}

/// `committee` command
pub async fn committee_command(wallet: &Wallet, epoch: Option<EpochIndex>) -> Result<(), Error> {
    let committee = wallet.client().get_committee(epoch).await?;
    println_log_info!("{committee:#?}");
    Ok(())
}

// `congestion` command
pub async fn congestion_command(
    wallet: &Wallet,
    account_id: Option<AccountId>,
    work_score: Option<u32>,
) -> Result<(), Error> {
    let account_id = match account_id {
        Some(account_id) => account_id,
        None => wallet
            .first_block_issuer_account_id()
            .await?
            .ok_or(WalletError::AccountNotFound)?,
    };

    let congestion = wallet.client().get_account_congestion(&account_id, work_score).await?;

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

// `create-delegation` command
pub async fn create_delegation_command(
    wallet: &Wallet,
    address: Option<Bech32Address>,
    delegated_amount: u64,
    validator_account_id: AccountId,
) -> Result<(), Error> {
    println_log_info!("Creating delegation output.");

    let transaction = wallet
        .create_delegation_output(
            CreateDelegationParams {
                address,
                delegated_amount,
                validator_address: AccountAddress::new(validator_account_id),
            },
            None,
        )
        .await?;

    println_log_info!(
        "Delegation creation transaction sent:\n{:?}\n{:?}\n{:?}",
        transaction.transaction.transaction_id,
        transaction.transaction.block_id,
        transaction.delegation_id
    );

    Ok(())
}

// `create-native-token` command
pub async fn create_native_token_command(
    wallet: &Wallet,
    circulating_supply: U256,
    maximum_supply: U256,
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
            .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
            .await?;
        // Sync wallet after the transaction got confirmed, so the account output is available
        wallet.sync(None).await?;
    }

    let params = CreateNativeTokenParams {
        account_id: None,
        circulating_supply,
        maximum_supply,
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

// `delay-delegation-claiming` command
pub async fn delay_delegation_claiming_command(
    wallet: &Wallet,
    delegation_id: DelegationId,
    reclaim_excess: bool,
) -> Result<(), Error> {
    println_log_info!("Delaying delegation claiming.");

    let transaction = wallet.delay_delegation_claiming(delegation_id, reclaim_excess).await?;

    println_log_info!(
        "Delay delegation claiming transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `destroy-account` command
pub async fn destroy_account_command(wallet: &Wallet, account_id: AccountId) -> Result<(), Error> {
    println_log_info!("Destroying account {account_id}.");

    let transaction = wallet.burn(account_id, None).await?;

    println_log_info!(
        "Destroying account transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `destroy-delegation` command
pub async fn destroy_delegation_command(wallet: &Wallet, delegation_id: DelegationId) -> Result<(), Error> {
    println_log_info!("Destroying delegation {delegation_id}.");

    let transaction = wallet.burn(delegation_id, None).await?;

    println_log_info!(
        "Destroying delegation transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `destroy-foundry` command
pub async fn destroy_foundry_command(wallet: &Wallet, foundry_id: FoundryId) -> Result<(), Error> {
    println_log_info!("Destroying foundry {foundry_id}.");

    let transaction = wallet.burn(foundry_id, None).await?;

    println_log_info!(
        "Destroying foundry transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `end-staking` command
pub async fn end_staking_command(wallet: &Wallet, account_id: AccountId) -> Result<(), Error> {
    println_log_info!("Ending staking for {account_id}.");

    let transaction = wallet.end_staking(account_id, None).await?;

    println_log_info!(
        "End staking transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `extend-staking` command
pub async fn extend_staking_command(
    wallet: &Wallet,
    account_id: AccountId,
    additional_epochs: u32,
) -> Result<(), Error> {
    println_log_info!("Extending staking for {account_id} by {additional_epochs} epochs.");

    let transaction = wallet.extend_staking(account_id, additional_epochs, None).await?;

    println_log_info!(
        "Extend staking transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `faucet` command
pub async fn faucet_command(wallet: &Wallet, address: Option<Bech32Address>, url: &str) -> Result<(), Error> {
    let address = if let Some(address) = address {
        address
    } else {
        wallet.address().await
    };

    let response = request_funds_from_faucet(url, &address).await?;

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
pub async fn implicit_account_transition_command(wallet: &Wallet, output_id: OutputId) -> Result<(), Error> {
    let transaction = wallet
        .implicit_account_transition(&output_id, BlockIssuerKeySource::ImplicitAccountAddress)
        .await?;

    println_log_info!(
        "Implicit account transition transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `implicit-accounts` command
pub async fn implicit_accounts_command(wallet: &Wallet) -> Result<(), Error> {
    let wallet_ledger = wallet.ledger().await;
    let implicit_accounts = wallet_ledger.implicit_accounts();
    let hrp = wallet.client().get_bech32_hrp().await?;

    println_log_info!("Implicit accounts:\n");

    for implicit_account in implicit_accounts {
        let output_id = implicit_account.output_id;
        let account_id = AccountId::from(&output_id);
        let account_address = account_id.to_bech32(hrp);
        let bic = wallet
            .client()
            .get_account_congestion(&account_id, None)
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

// `add-block-issuer-key` command
pub async fn add_block_issuer_key(wallet: &Wallet, account_id: AccountId, issuer_key: &str) -> Result<(), Error> {
    let issuer_key: [u8; Ed25519PublicKeyHashBlockIssuerKey::LENGTH] = prefix_hex::decode(issuer_key)?;
    let params = ModifyAccountBlockIssuerKey {
        account_id,
        keys_to_add: vec![Ed25519PublicKeyHashBlockIssuerKey::new(issuer_key).into()],
        keys_to_remove: vec![],
    };

    let transaction = wallet.modify_account_output_block_issuer_keys(params, None).await?;

    println_log_info!(
        "Block issuer key adding transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `remove-block-issuer-key` command
pub async fn remove_block_issuer_key(wallet: &Wallet, account_id: AccountId, issuer_key: &str) -> Result<(), Error> {
    let issuer_key: [u8; Ed25519PublicKeyHashBlockIssuerKey::LENGTH] = prefix_hex::decode(issuer_key)?;
    let params = ModifyAccountBlockIssuerKey {
        account_id,
        keys_to_add: vec![],
        keys_to_remove: vec![Ed25519PublicKeyHashBlockIssuerKey::new(issuer_key).into()],
    };

    let transaction = wallet.modify_account_output_block_issuer_keys(params, None).await?;

    println_log_info!(
        "Block issuer key removing transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `melt-native-token` command
pub async fn melt_native_token_command(wallet: &Wallet, token_id: TokenId, amount: U256) -> Result<(), Error> {
    let transaction = wallet.melt_native_token(token_id, amount, None).await?;

    println_log_info!(
        "Native token melting transaction sent:\n{:?}\n{:?}",
        transaction.transaction_id,
        transaction.block_id
    );

    Ok(())
}

// `mint-native-token` command
pub async fn mint_native_token_command(wallet: &Wallet, token_id: TokenId, amount: U256) -> Result<(), Error> {
    let mint_transaction = wallet.mint_native_token(token_id, amount, None).await?;

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
    metadata: Option<(String, Vec<u8>)>,
    immutable_metadata: Option<(String, Vec<u8>)>,
    tag: Option<String>,
    sender: Option<Bech32Address>,
    issuer: Option<Bech32Address>,
) -> Result<(), Error> {
    let tag = if let Some(hex) = tag {
        Some(prefix_hex::decode(hex)?)
    } else {
        None
    };

    let mut nft_options = MintNftParams::new()
        .with_address(address)
        .with_tag(tag)
        .with_sender(sender)
        .with_issuer(issuer);

    if let Some(metadata) = metadata {
        nft_options = nft_options.with_metadata(MetadataFeature::new([metadata])?);
    }
    if let Some(immutable_metadata) = immutable_metadata {
        nft_options = nft_options.with_immutable_metadata(MetadataFeature::new([immutable_metadata])?);
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
    let node_info = serde_json::to_string_pretty(&wallet.client().get_node_info().await?)?;

    println_log_info!("Current node info: {node_info}");

    Ok(())
}

/// `output` command
pub async fn output_command(wallet: &Wallet, selector: OutputSelector, metadata: bool) -> Result<(), Error> {
    let wallet_ledger = wallet.ledger().await;
    let output = match selector {
        OutputSelector::Id(id) => wallet_ledger.get_output(&id),
        OutputSelector::Index(index) => {
            let mut outputs = wallet_ledger.outputs().values().collect::<Vec<_>>();
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
    print_outputs(wallet.ledger().await.outputs().values().cloned().collect(), "Outputs:")
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

// `send-mana` command
pub async fn send_mana_command(
    wallet: &Wallet,
    address: impl ConvertTo<Bech32Address>,
    mana: u64,
    gift: bool,
) -> Result<(), Error> {
    let params = SendManaParams::new(mana, address.convert()?).with_return_strategy(if gift {
        ReturnStrategy::Gift
    } else {
        ReturnStrategy::Return
    });
    let transaction = wallet.send_mana(params, None).await?;

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
    token_id: TokenId,
    amount: U256,
    gift_storage_deposit: Option<bool>,
) -> Result<(), Error> {
    let address = address.convert()?;
    let transaction = if gift_storage_deposit.unwrap_or(false) {
        // Send native tokens together with the required storage deposit
        let storage_params = wallet.client().get_storage_score_parameters().await?;

        wallet.client().bech32_hrp_matches(address.hrp()).await?;

        let outputs = [BasicOutputBuilder::new_with_minimum_amount(storage_params)
            .add_unlock_condition(AddressUnlockCondition::new(address))
            .with_native_token(NativeToken::new(token_id, amount)?)
            .finish_output()?];

        wallet.send_outputs(outputs, None).await?
    } else {
        // Send native tokens with storage deposit return and expiration
        let outputs = [SendNativeTokenParams::new(address, (token_id, amount))?];
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
    nft_id: NftId,
) -> Result<(), Error> {
    let outputs = [SendNftParams::new(address.convert()?, nft_id)?];
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
    let wallet_ledger = wallet.ledger().await;
    let transaction = match selector {
        TransactionSelector::Id(id) => wallet_ledger.get_transaction(&id),
        TransactionSelector::Index(index) => {
            let mut transactions = wallet_ledger.transactions().values().collect::<Vec<_>>();
            transactions.sort_unstable_by(|a, b| {
                b.payload
                    .transaction()
                    .creation_slot()
                    .cmp(&a.payload.transaction().creation_slot())
            });
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
    let wallet_ledger = wallet.ledger().await;
    let mut transactions = wallet_ledger.transactions().values().collect::<Vec<_>>();
    transactions.sort_unstable_by(|a, b| {
        b.payload
            .transaction()
            .creation_slot()
            .cmp(&a.payload.transaction().creation_slot())
    });

    if transactions.is_empty() {
        println_log_info!("No transactions found");
    } else {
        for (i, tx) in transactions.into_iter().enumerate() {
            if show_details {
                println_log_info!("{:#?}", tx);
            } else {
                let protocol_parameters = wallet.client().get_protocol_parameters().await?;
                let creation_slot = tx.payload.transaction().creation_slot();
                let creation_time = to_utc_date_time(creation_slot.to_timestamp(
                    protocol_parameters.genesis_unix_timestamp(),
                    protocol_parameters.slot_duration_in_seconds(),
                ) as u128)?
                .format("%Y-%m-%d %H:%M:%S UTC")
                .to_string();

                println_log_info!("{:<5}{}\t{}\t{}", i, tx.transaction_id, creation_slot, creation_time);
            }
        }
    }

    Ok(())
}

/// `unspent-outputs` command
pub async fn unspent_outputs_command(wallet: &Wallet) -> Result<(), Error> {
    print_outputs(
        wallet.ledger().await.unspent_outputs().values().cloned().collect(),
        "Unspent outputs:",
    )
}

/// `validator` command
pub async fn validator_command(wallet: &Wallet, account_id: &AccountId) -> Result<(), Error> {
    let validator = wallet.client().get_validator(account_id).await?;
    println_log_info!("{validator:#?}");
    Ok(())
}

/// `validators` command
pub async fn validators_command(wallet: &Wallet) -> Result<(), Error> {
    let validators = wallet.client().get_validators(None, None).await?;
    println_log_info!("{validators:#?}");
    Ok(())
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

    for output_data in wallet.ledger().await.unspent_outputs().values() {
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
            let sdr_amount = output_data
                .output
                .unlock_conditions()
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

async fn ensure_password(wallet: &Wallet) -> Result<(), Error> {
    if matches!(*wallet.secret_manager().read().await, SecretManager::Stronghold(_))
        && !wallet.is_stronghold_password_available().await?
    {
        let password = enter_password("Stronghold password", false)?;
        wallet.set_stronghold_password(password).await?;
    }

    Ok(())
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
                            ensure_password(wallet).await?;
                            allot_mana_command(wallet, mana, account_id).await
                        }
                        WalletCommand::AnnounceCandidacy { account_id } => {
                            ensure_password(wallet).await?;
                            announce_candidacy_command(wallet, account_id).await
                        }
                        WalletCommand::Balance => balance_command(wallet).await,
                        WalletCommand::BeginStaking {
                            account_id,
                            staked_amount,
                            fixed_cost,
                            staking_period,
                        } => {
                            ensure_password(wallet).await?;
                            begin_staking_command(wallet, account_id, staked_amount, fixed_cost, staking_period).await
                        }
                        WalletCommand::BurnNativeToken { token_id, amount } => {
                            ensure_password(wallet).await?;
                            burn_native_token_command(wallet, token_id, amount).await
                        }
                        WalletCommand::BurnNft { nft_id } => {
                            ensure_password(wallet).await?;
                            burn_nft_command(wallet, nft_id).await
                        }
                        WalletCommand::Claim { output_id } => {
                            ensure_password(wallet).await?;
                            claim_command(wallet, output_id).await
                        }
                        WalletCommand::ClaimableOutputs => claimable_outputs_command(wallet).await,
                        WalletCommand::Committee { epoch } => committee_command(wallet, epoch).await,
                        WalletCommand::Congestion { account_id, work_score } => {
                            congestion_command(wallet, account_id, work_score).await
                        }
                        WalletCommand::Consolidate => {
                            ensure_password(wallet).await?;
                            consolidate_command(wallet).await
                        }
                        WalletCommand::CreateAccountOutput => {
                            ensure_password(wallet).await?;
                            create_account_output_command(wallet).await
                        }
                        WalletCommand::CreateDelegation {
                            address,
                            delegated_amount,
                            validator_account_id,
                        } => {
                            ensure_password(wallet).await?;
                            create_delegation_command(wallet, address, delegated_amount, validator_account_id).await
                        }
                        WalletCommand::CreateNativeToken {
                            circulating_supply,
                            maximum_supply,
                            foundry_metadata_key,
                            foundry_metadata_hex,
                            foundry_metadata_file,
                        } => {
                            ensure_password(wallet).await?;
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
                        WalletCommand::DelayDelegationClaiming {
                            delegation_id,
                            reclaim_excess,
                        } => {
                            ensure_password(wallet).await?;
                            delay_delegation_claiming_command(wallet, delegation_id, reclaim_excess).await
                        }
                        WalletCommand::DestroyAccount { account_id } => {
                            ensure_password(wallet).await?;
                            destroy_account_command(wallet, account_id).await
                        }
                        WalletCommand::DestroyDelegation { delegation_id } => {
                            ensure_password(wallet).await?;
                            destroy_delegation_command(wallet, delegation_id).await
                        }
                        WalletCommand::DestroyFoundry { foundry_id } => {
                            ensure_password(wallet).await?;
                            destroy_foundry_command(wallet, foundry_id).await
                        }
                        WalletCommand::EndStaking { account_id } => {
                            ensure_password(wallet).await?;
                            end_staking_command(wallet, account_id).await
                        }
                        WalletCommand::Exit => {
                            return Ok(PromptResponse::Done);
                        }
                        WalletCommand::ExtendStaking {
                            account_id,
                            additional_epochs,
                        } => {
                            ensure_password(wallet).await?;
                            extend_staking_command(wallet, account_id, additional_epochs).await
                        }
                        WalletCommand::Faucet { address, url } => faucet_command(wallet, address, &url).await,
                        WalletCommand::ImplicitAccountCreationAddress => {
                            implicit_account_creation_address_command(wallet).await
                        }
                        WalletCommand::ImplicitAccountTransition { output_id } => {
                            ensure_password(wallet).await?;
                            implicit_account_transition_command(wallet, output_id).await
                        }
                        WalletCommand::ImplicitAccounts => implicit_accounts_command(wallet).await,
                        WalletCommand::AddBlockIssuerKey {
                            account_id,
                            block_issuer_key,
                        } => {
                            ensure_password(wallet).await?;
                            add_block_issuer_key(wallet, account_id, &block_issuer_key).await
                        }
                        WalletCommand::RemoveBlockIssuerKey {
                            account_id,
                            block_issuer_key,
                        } => {
                            ensure_password(wallet).await?;
                            remove_block_issuer_key(wallet, account_id, &block_issuer_key).await
                        }
                        WalletCommand::MeltNativeToken { token_id, amount } => {
                            ensure_password(wallet).await?;
                            melt_native_token_command(wallet, token_id, amount).await
                        }
                        WalletCommand::MintNativeToken { token_id, amount } => {
                            ensure_password(wallet).await?;
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
                            ensure_password(wallet).await?;
                            mint_nft_command(
                                wallet,
                                address,
                                bytes_from_hex_or_file(metadata_hex, metadata_file)
                                    .await?
                                    .map(|value| (metadata_key, value)),
                                bytes_from_hex_or_file(immutable_metadata_hex, immutable_metadata_file)
                                    .await?
                                    .map(|value| (immutable_metadata_key, value)),
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
                            ensure_password(wallet).await?;
                            let allow_micro_amount = if return_address.is_some() || expiration.is_some() {
                                true
                            } else {
                                allow_micro_amount
                            };
                            send_command(wallet, address, amount, return_address, expiration, allow_micro_amount).await
                        }
                        WalletCommand::SendMana { address, mana, gift } => {
                            ensure_password(wallet).await?;
                            send_mana_command(wallet, address, mana, gift).await
                        }
                        WalletCommand::SendNativeToken {
                            address,
                            token_id,
                            amount,
                            gift_storage_deposit,
                        } => {
                            ensure_password(wallet).await?;
                            send_native_token_command(wallet, address, token_id, amount, gift_storage_deposit).await
                        }
                        WalletCommand::SendNft { address, nft_id } => {
                            ensure_password(wallet).await?;
                            send_nft_command(wallet, address, nft_id).await
                        }
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
                        WalletCommand::Validator { account_id } => validator_command(wallet, &account_id).await,
                        WalletCommand::Validators => validators_command(wallet).await,
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
                if output_data.is_spent() { "Spent" } else { "Unspent" },
            );
        }
    }

    Ok(())
}
