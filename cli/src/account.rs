// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use colored::Colorize;
use iota_sdk::{
    client::secret::SecretManager,
    wallet::{Account, Wallet},
};
use rustyline::{error::ReadlineError, history::MemHistory, Config, Editor};

use crate::{
    command::{
        account::{
            address_command, addresses_command, balance_command, burn_native_token_command, burn_nft_command,
            claim_command, claimable_outputs_command, consolidate_command, create_alias_outputs_command,
            create_native_token_command, decrease_voting_power_command, destroy_alias_command, destroy_foundry_command,
            faucet_command, increase_voting_power_command, melt_native_token_command, mint_native_token,
            mint_nft_command, new_address_command, node_info_command, output_command, outputs_command,
            participation_overview_command, send_command, send_native_token_command, send_nft_command,
            stop_participating_command, sync_command, transaction_command, transactions_command,
            unspent_outputs_command, vote_command, voting_output_command, voting_power_command, AccountCli,
            AccountCommand,
        },
        account_completion::AccountPromptHelper,
    },
    error::Error,
    helper::{bytes_from_hex_or_file, get_password},
    println_log_error,
};

// loop on the account prompt
pub async fn account_prompt(wallet: &Wallet, mut account: Account) -> Result<(), Error> {
    let config = Config::builder()
        .auto_add_history(true)
        .history_ignore_space(true)
        .completion_type(rustyline::CompletionType::List)
        .edit_mode(rustyline::EditMode::Emacs)
        .build();

    let mut rl = Editor::with_history(config, MemHistory::with_config(config))?;
    rl.set_helper(Some(AccountPromptHelper::default()));

    loop {
        match account_prompt_internal(wallet, &account, &mut rl).await {
            Ok(res) => match res {
                AccountPromptResponse::Reprompt => (),
                AccountPromptResponse::Done => {
                    return Ok(());
                }
                AccountPromptResponse::Switch(new_account) => {
                    account = new_account;
                }
            },
            Err(e) => {
                println_log_error!("{e}");
            }
        }
    }
}

pub enum AccountPromptResponse {
    Reprompt,
    Done,
    Switch(Account),
}

async fn ensure_password(wallet: &Wallet) -> Result<(), Error> {
    if matches!(*wallet.get_secret_manager().read().await, SecretManager::Stronghold(_))
        && !wallet.is_stronghold_password_available().await?
    {
        let password = get_password("Stronghold password", false)?;
        wallet.set_stronghold_password(password).await?;
    }

    Ok(())
}

// loop on the account prompt
pub async fn account_prompt_internal(
    wallet: &Wallet,
    account: &Account,
    rl: &mut Editor<AccountPromptHelper, MemHistory>,
) -> Result<AccountPromptResponse, Error> {
    let alias = account.details().await.alias().clone();

    let prompt = format!("Account \"{alias}\": ");
    if let Some(helper) = rl.helper_mut() {
        helper.set_prompt(prompt.green().to_string());
    }

    let input = rl.readline(&prompt);

    match input {
        Ok(command) => {
            match command.trim() {
                "" => {}
                "h" | "help" => AccountCli::print_help()?,
                "c" | "clear" => {
                    // Clear console
                    let _ = std::process::Command::new("clear").status();
                }
                "accounts" => {
                    // List all accounts
                    let accounts = wallet.get_accounts().await?;
                    println!("INDEX\tALIAS");
                    for account in accounts {
                        let details = &*account.details().await;
                        println!("{}\t{}", details.index(), details.alias());
                    }
                }
                _ => {
                    // Prepend `Account: ` so the parsing will be correct
                    let command = format!("Account: {command}");
                    let account_cli = match AccountCli::try_parse_from(command.split_whitespace()) {
                        Ok(account_cli) => account_cli,
                        Err(err) => {
                            println!("{err}");
                            return Ok(AccountPromptResponse::Reprompt);
                        }
                    };
                    match account_cli.command {
                        AccountCommand::Address { selector } => address_command(account, selector).await,
                        AccountCommand::Addresses => addresses_command(account).await,
                        AccountCommand::Balance { addresses } => balance_command(account, addresses).await,
                        AccountCommand::BurnNativeToken { token_id, amount } => {
                            ensure_password(wallet).await?;
                            burn_native_token_command(account, token_id, amount).await
                        }
                        AccountCommand::BurnNft { nft_id } => {
                            ensure_password(wallet).await?;
                            burn_nft_command(account, nft_id).await
                        }
                        AccountCommand::Claim { output_id } => {
                            ensure_password(wallet).await?;
                            claim_command(account, output_id).await
                        }
                        AccountCommand::ClaimableOutputs => claimable_outputs_command(account).await,
                        AccountCommand::Consolidate => {
                            ensure_password(wallet).await?;
                            consolidate_command(account).await
                        }
                        AccountCommand::CreateAliasOutput => {
                            ensure_password(wallet).await?;
                            create_alias_outputs_command(account).await
                        }
                        AccountCommand::CreateNativeToken {
                            circulating_supply,
                            maximum_supply,
                            foundry_metadata_hex,
                            foundry_metadata_file,
                        } => {
                            ensure_password(wallet).await?;
                            create_native_token_command(
                                account,
                                circulating_supply,
                                maximum_supply,
                                bytes_from_hex_or_file(foundry_metadata_hex, foundry_metadata_file).await?,
                            )
                            .await
                        }
                        AccountCommand::DestroyAlias { alias_id } => {
                            ensure_password(wallet).await?;
                            destroy_alias_command(account, alias_id).await
                        }
                        AccountCommand::DestroyFoundry { foundry_id } => {
                            ensure_password(wallet).await?;
                            destroy_foundry_command(account, foundry_id).await
                        }
                        AccountCommand::Exit => {
                            return Ok(AccountPromptResponse::Done);
                        }
                        AccountCommand::Faucet { address, url } => faucet_command(account, address, url).await,
                        AccountCommand::MeltNativeToken { token_id, amount } => {
                            ensure_password(wallet).await?;
                            melt_native_token_command(account, token_id, amount).await
                        }
                        AccountCommand::MintNativeToken { token_id, amount } => {
                            ensure_password(wallet).await?;
                            mint_native_token(account, token_id, amount).await
                        }
                        AccountCommand::MintNft {
                            address,
                            immutable_metadata_hex,
                            immutable_metadata_file,
                            metadata_hex,
                            metadata_file,
                            tag,
                            sender,
                            issuer,
                        } => {
                            ensure_password(wallet).await?;
                            mint_nft_command(
                                account,
                                address,
                                bytes_from_hex_or_file(immutable_metadata_hex, immutable_metadata_file).await?,
                                bytes_from_hex_or_file(metadata_hex, metadata_file).await?,
                                tag,
                                sender,
                                issuer,
                            )
                            .await
                        }
                        AccountCommand::NewAddress => {
                            ensure_password(wallet).await?;
                            new_address_command(account).await
                        }
                        AccountCommand::NodeInfo => node_info_command(account).await,
                        AccountCommand::Output { selector } => output_command(account, selector).await,
                        AccountCommand::Outputs => outputs_command(account).await,
                        AccountCommand::Send {
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
                            send_command(
                                account,
                                address,
                                amount,
                                return_address,
                                expiration.map(|e| e.as_secs() as u32),
                                allow_micro_amount,
                            )
                            .await
                        }
                        AccountCommand::SendNativeToken {
                            address,
                            token_id,
                            amount,
                            gift_storage_deposit,
                        } => {
                            ensure_password(wallet).await?;
                            send_native_token_command(account, address, token_id, amount, gift_storage_deposit).await
                        }
                        AccountCommand::SendNft { address, nft_id } => {
                            ensure_password(wallet).await?;
                            send_nft_command(account, address, nft_id).await
                        }
                        AccountCommand::Switch { account_id } => {
                            return Ok(AccountPromptResponse::Switch(wallet.get_account(account_id).await?));
                        }
                        AccountCommand::Sync => sync_command(account).await,
                        AccountCommand::Transaction { selector } => transaction_command(account, selector).await,
                        AccountCommand::Transactions { show_details } => {
                            transactions_command(account, show_details).await
                        }
                        AccountCommand::UnspentOutputs => unspent_outputs_command(account).await,
                        AccountCommand::Vote { event_id, answers } => {
                            ensure_password(wallet).await?;
                            vote_command(account, event_id, answers).await
                        }
                        AccountCommand::StopParticipating { event_id } => {
                            ensure_password(wallet).await?;
                            stop_participating_command(account, event_id).await
                        }
                        AccountCommand::ParticipationOverview { event_ids } => {
                            let event_ids = (!event_ids.is_empty()).then_some(event_ids);
                            participation_overview_command(account, event_ids).await
                        }
                        AccountCommand::VotingPower => voting_power_command(account).await,
                        AccountCommand::IncreaseVotingPower { amount } => {
                            ensure_password(wallet).await?;
                            increase_voting_power_command(account, amount).await
                        }
                        AccountCommand::DecreaseVotingPower { amount } => {
                            ensure_password(wallet).await?;
                            decrease_voting_power_command(account, amount).await
                        }
                        AccountCommand::VotingOutput => voting_output_command(account).await,
                    }
                    .unwrap_or_else(|err| {
                        println_log_error!("{err}");
                    });
                }
            }
        }
        Err(ReadlineError::Interrupted) => {
            return Ok(AccountPromptResponse::Done);
        }
        Err(err) => {
            println_log_error!("{err}");
        }
    }

    Ok(AccountPromptResponse::Reprompt)
}
