// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use colored::Colorize;
use iota_sdk::wallet::{Account, Wallet};
use rustyline::{error::ReadlineError, history::MemHistory, Config, Editor};

use crate::{
    command::{
        account::{
            addresses_command, balance_command, burn_native_token_command, burn_nft_command, claim_command,
            claimable_outputs_command, consolidate_command, create_account_output_command, create_native_token_command,
            decrease_voting_power_command, destroy_account_command, destroy_foundry_command, faucet_command,
            increase_voting_power_command, melt_native_token_command, mint_native_token, mint_nft_command,
            new_address_command, node_info_command, output_command, outputs_command, participation_overview_command,
            send_command, send_native_token_command, send_nft_command, stop_participating_command, sync_command,
            transaction_command, transactions_command, unspent_outputs_command, vote_command, voting_output_command,
            voting_power_command, AccountCli, AccountCommand,
        },
        account_completion::AccountPromptHelper,
    },
    error::Error,
    helper::bytes_from_hex_or_file,
    println_log_error,
};

// loop on the account prompt
pub async fn prompt(wallet: &Wallet) -> Result<(), Error> {
    let config = Config::builder()
        .auto_add_history(true)
        .history_ignore_space(true)
        .completion_type(rustyline::CompletionType::List)
        .edit_mode(rustyline::EditMode::Emacs)
        .build();

    let mut rl = Editor::with_history(config, MemHistory::with_config(config))?;
    rl.set_helper(Some(AccountPromptHelper::default()));

    loop {
        match prompt_internal(wallet, &mut rl).await {
            Ok(res) => match res {
                AccountPromptResponse::Reprompt => (),
                AccountPromptResponse::Done => {
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

// loop on the account prompt
pub async fn prompt_internal(
    wallet: &Wallet,
    rl: &mut Editor<AccountPromptHelper, MemHistory>,
) -> Result<AccountPromptResponse, Error> {
    let alias = wallet.alias().await;
    let prompt = format!("Wallet \"{alias}\": ");

    if let Some(helper) = rl.helper_mut() {
        helper.set_prompt(prompt.green().to_string());
    }

    let input = rl.readline(&prompt);
    match input {
        Ok(command) => {
            match command.as_str() {
                "h" | "help" => ProtocolCli::print_help()?,
                "c" | "clear" => {
                    // Clear console
                    let _ = std::process::Command::new("clear").status();
                }
                _ => {
                    // Prepend `Wallet: ` so the parsing will be correct
                    let command = format!("Wallet: {}", command.trim());
                    let account_cli = match ProtocolCli::try_parse_from(command.split_whitespace()) {
                        Ok(account_cli) => account_cli,
                        Err(err) => {
                            println!("{err}");
                            return Ok(AccountPromptResponse::Reprompt);
                        }
                    };
                    match account_cli.command {
                        ProtocolCommand::Address => address_command(wallet).await,
                        ProtocolCommand::Balance { addresses } => balance_command(wallet, addresses).await,
                        ProtocolCommand::BurnNativeToken { token_id, amount } => {
                            burn_native_token_command(wallet, token_id, amount).await
                        }
                        ProtocolCommand::BurnNft { nft_id } => burn_nft_command(wallet, nft_id).await,
                        ProtocolCommand::Claim { output_id } => claim_command(wallet, output_id).await,
                        ProtocolCommand::ClaimableOutputs => claimable_outputs_command(wallet).await,
                        ProtocolCommand::Consolidate => consolidate_command(wallet).await,
                        ProtocolCommand::CreateAccountOutput => create_account_output_command(wallet).await,
                        ProtocolCommand::CreateNativeToken {
                            circulating_supply,
                            maximum_supply,
                            foundry_metadata_hex,
                            foundry_metadata_file,
                        } => {
                            create_native_token_command(
                                wallet,
                                circulating_supply,
                                maximum_supply,
                                bytes_from_hex_or_file(foundry_metadata_hex, foundry_metadata_file).await?,
                            )
                            .await
                        }
                        ProtocolCommand::DestroyAccount { account_id } => {
                            destroy_account_command(wallet, account_id).await
                        }
                        ProtocolCommand::DestroyFoundry { foundry_id } => {
                            destroy_foundry_command(wallet, foundry_id).await
                        }
                        ProtocolCommand::Exit => {
                            return Ok(AccountPromptResponse::Done);
                        }
                        ProtocolCommand::Faucet { address, url } => faucet_command(wallet, address, url).await,
                        ProtocolCommand::MeltNativeToken { token_id, amount } => {
                            melt_native_token_command(wallet, token_id, amount).await
                        }
                        ProtocolCommand::MintNativeToken { token_id, amount } => {
                            mint_native_token(wallet, token_id, amount).await
                        }
                        ProtocolCommand::MintNft {
                            address,
                            immutable_metadata_hex,
                            immutable_metadata_file,
                            metadata_hex,
                            metadata_file,
                            tag,
                            sender,
                            issuer,
                        } => {
                            mint_nft_command(
                                wallet,
                                address,
                                bytes_from_hex_or_file(immutable_metadata_hex, immutable_metadata_file).await?,
                                bytes_from_hex_or_file(metadata_hex, metadata_file).await?,
                                tag,
                                sender,
                                issuer,
                            )
                            .await
                        }
                        ProtocolCommand::NodeInfo => node_info_command(wallet).await,
                        ProtocolCommand::Output { output_id } => output_command(wallet, output_id).await,
                        ProtocolCommand::Outputs => outputs_command(wallet).await,
                        ProtocolCommand::Send {
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
                        ProtocolCommand::SendNativeToken {
                            address,
                            token_id,
                            amount,
                            gift_storage_deposit,
                        } => send_native_token_command(wallet, address, token_id, amount, gift_storage_deposit).await,
                        ProtocolCommand::SendNft { address, nft_id } => send_nft_command(wallet, address, nft_id).await,
                        ProtocolCommand::Sync => sync_command(wallet).await,
                        ProtocolCommand::Transaction { selector } => transaction_command(wallet, selector).await,
                        ProtocolCommand::Transactions { show_details } => {
                            transactions_command(wallet, show_details).await
                        }
                        ProtocolCommand::UnspentOutputs => unspent_outputs_command(wallet).await,
                        ProtocolCommand::Vote { event_id, answers } => vote_command(wallet, event_id, answers).await,
                        ProtocolCommand::StopParticipating { event_id } => {
                            stop_participating_command(wallet, event_id).await
                        }
                        ProtocolCommand::ParticipationOverview { event_ids } => {
                            let event_ids = (!event_ids.is_empty()).then_some(event_ids);
                            participation_overview_command(wallet, event_ids).await
                        }
                        ProtocolCommand::VotingPower => voting_power_command(wallet).await,
                        ProtocolCommand::IncreaseVotingPower { amount } => {
                            increase_voting_power_command(wallet, amount).await
                        }
                        ProtocolCommand::DecreaseVotingPower { amount } => {
                            decrease_voting_power_command(wallet, amount).await
                        }
                        ProtocolCommand::VotingOutput => voting_output_command(wallet).await,
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
