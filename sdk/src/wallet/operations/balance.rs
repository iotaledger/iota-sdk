// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use primitive_types::U256;

use crate::{
    client::secret::SecretManage,
    types::block::{
        address::ImplicitAccountCreationAddress,
        output::{
            unlock_condition::UnlockCondition, DecayedMana, FoundryId, MinimumOutputAmount, NativeTokensBuilder, Output,
        },
    },
    wallet::{
        operations::{helpers::time::can_output_be_unlocked_from_now_on, output_claiming::OutputsToClaim},
        types::{Balance, NativeTokensBalance},
        Wallet, WalletError,
    },
};

impl<S: 'static + SecretManage> Wallet<S> {
    /// Get the balance of the wallet.
    pub async fn balance(&self) -> Result<Balance, WalletError> {
        log::debug!("[BALANCE] balance");

        let protocol_parameters = self.client().get_protocol_parameters().await?;
        let slot_index = self.client().get_slot_index().await?;

        let wallet_address = self.address().await;
        let wallet_ledger = self.ledger().await;
        let network_id = protocol_parameters.network_id();
        let storage_score_params = protocol_parameters.storage_score_parameters();

        let mut balance = Balance::default();
        let mut total_storage_cost = 0;
        let mut delegation_implicit_accounts_amount = 0;
        let mut total_native_tokens = NativeTokensBuilder::default();

        #[cfg(feature = "participation")]
        let voting_output = wallet_ledger.get_voting_output();

        let claimable_outputs = wallet_ledger.claimable_outputs(
            wallet_address.inner().clone(),
            OutputsToClaim::All,
            slot_index,
            &protocol_parameters,
        )?;

        #[cfg(feature = "participation")]
        {
            if let Some(voting_output) = &voting_output {
                if voting_output.output.as_basic().address() == wallet_address.inner() {
                    balance.base_coin.voting_power = voting_output.output.amount();
                }
            }
        }

        let controlled_addresses = wallet_ledger.controlled_addresses(wallet_address.inner().clone());

        let mut reward_outputs = HashSet::new();

        for (output_id, output_data) in &wallet_ledger.unspent_outputs {
            // Check if output is from the network we're currently connected to
            if output_data.network_id != network_id {
                continue;
            }

            let output = &output_data.output;
            let storage_cost = output.minimum_amount(storage_score_params);

            // Add account, foundry, and delegation outputs here because they can't have a
            // [`StorageDepositReturnUnlockCondition`] or time related unlock conditions
            match output {
                Output::Account(account) => {
                    // Add amount
                    balance.base_coin.total += account.amount();
                    balance.mana.total += output.decayed_mana(
                        &protocol_parameters,
                        output_id.transaction_id().slot_index(),
                        slot_index,
                    )?;
                    // Add mana rewards
                    if account.features().staking().is_some() {
                        reward_outputs.insert(*output_id);
                    }

                    // Add storage deposit
                    balance.required_storage_deposit.account += storage_cost;
                    if !wallet_ledger.locked_outputs.contains(output_id) {
                        total_storage_cost += storage_cost;
                    }

                    let account_id = account.account_id_non_null(output_id);
                    balance.accounts.insert(account_id);
                }
                Output::Foundry(foundry) => {
                    // Add amount
                    balance.base_coin.total += foundry.amount();
                    // Add storage deposit
                    balance.required_storage_deposit.foundry += storage_cost;
                    if !wallet_ledger.locked_outputs.contains(output_id) {
                        total_storage_cost += storage_cost;
                    }

                    // Add native token
                    if let Some(native_token) = output.native_token() {
                        total_native_tokens.add_native_token(*native_token)?;
                    }

                    balance.foundries.insert(foundry.id());
                }
                Output::Delegation(delegation) => {
                    // Add amount
                    balance.base_coin.total += delegation.amount();
                    delegation_implicit_accounts_amount += delegation.amount();
                    // Add mana rewards
                    reward_outputs.insert(*output_id);
                    // Add storage deposit
                    balance.required_storage_deposit.delegation += storage_cost;
                    if !wallet_ledger.locked_outputs.contains(output_id) {
                        total_storage_cost += storage_cost;
                    }

                    let delegation_id = delegation.delegation_id_non_null(output_id);
                    balance.delegations.insert(delegation_id);
                }
                _ => {
                    // If there is only an [AddressUnlockCondition], then we can spend the output at any time
                    // without restrictions
                    if let [UnlockCondition::Address(address_unlock_cond)] = output
                        .unlock_conditions()
                        .expect("output needs to have unlock conditions")
                        .as_ref()
                    {
                        // add nft_id for nft outputs
                        if let Output::Nft(nft) = &output {
                            let nft_id = nft.nft_id_non_null(output_id);
                            balance.nfts.insert(nft_id);
                        }

                        // Add amount
                        balance.base_coin.total += output.amount();
                        if address_unlock_cond.address().kind() == ImplicitAccountCreationAddress::KIND {
                            delegation_implicit_accounts_amount += output.amount();
                        }
                        // Add decayed mana
                        balance.mana.total += output.decayed_mana(
                            &protocol_parameters,
                            output_id.transaction_id().slot_index(),
                            slot_index,
                        )?;

                        // Add storage deposit
                        if output.is_basic() {
                            balance.required_storage_deposit.basic += storage_cost;
                            if output.native_token().is_some() && !wallet_ledger.locked_outputs.contains(output_id) {
                                total_storage_cost += storage_cost;
                            }
                        } else if output.is_nft() {
                            balance.required_storage_deposit.nft += storage_cost;
                            if !wallet_ledger.locked_outputs.contains(output_id) {
                                total_storage_cost += storage_cost;
                            }
                        }

                        // Add native token
                        if let Some(native_token) = output.native_token() {
                            total_native_tokens.add_native_token(*native_token)?;
                        }
                    } else {
                        // if we have multiple unlock conditions for basic or nft outputs, then we can't
                        // spend the balance at the moment or in the future

                        let is_claimable = claimable_outputs.contains(output_id);

                        // For outputs that are expired or have a timelock unlock condition, but no expiration
                        // unlock condition and we then can unlock them, then
                        // they can never be not available for us anymore
                        // and should be added to the balance
                        if is_claimable {
                            // check if output can be unlocked always from now on, in that case it should be
                            // added to the total amount
                            let output_can_be_unlocked_now_and_in_future = can_output_be_unlocked_from_now_on(
                                &controlled_addresses,
                                output,
                                slot_index,
                                protocol_parameters.committable_age_range(),
                            );

                            if output_can_be_unlocked_now_and_in_future {
                                // If output has a StorageDepositReturnUnlockCondition, the amount of it should
                                // be subtracted, because this part
                                // needs to be sent back
                                let amount = output
                                    .unlock_conditions()
                                    .and_then(|u| u.storage_deposit_return())
                                    .map_or_else(
                                        || output.amount(),
                                        |sdr| {
                                            if wallet_address.inner() == sdr.return_address() {
                                                // sending to ourself, we get the full amount
                                                output.amount()
                                            } else {
                                                // Sending to someone else
                                                output.amount() - sdr.amount()
                                            }
                                        },
                                    );

                                // add nft_id for nft outputs
                                if let Output::Nft(output) = &output {
                                    let nft_id = output.nft_id_non_null(output_id);
                                    balance.nfts.insert(nft_id);
                                }

                                // Add amount
                                balance.base_coin.total += amount;
                                // Add decayed mana
                                balance.mana.total += output.decayed_mana(
                                    &protocol_parameters,
                                    output_id.transaction_id().slot_index(),
                                    slot_index,
                                )?;

                                // Add storage deposit
                                if output.is_basic() {
                                    balance.required_storage_deposit.basic += storage_cost;
                                    // Amount for basic outputs isn't added to total storage cost if there aren't native
                                    // tokens, since we can spend it without burning.
                                    if output.native_token().is_some()
                                        && !wallet_ledger.locked_outputs.contains(output_id)
                                    {
                                        total_storage_cost += storage_cost;
                                    }
                                } else if output.is_nft() {
                                    balance.required_storage_deposit.nft += storage_cost;
                                    if !wallet_ledger.locked_outputs.contains(output_id) {
                                        total_storage_cost += storage_cost;
                                    }
                                }

                                // Add native token
                                if let Some(native_token) = output.native_token() {
                                    total_native_tokens.add_native_token(*native_token)?;
                                }
                            } else {
                                // only add outputs that can't be locked now and at any point in the future
                                balance.potentially_locked_outputs.insert(*output_id, true);
                            }
                        } else {
                            // Don't add expired outputs that can't ever be unlocked by us
                            if let Some(expiration) = output
                                .unlock_conditions()
                                .expect("output needs to have unlock conditions")
                                .expiration()
                            {
                                // Not expired, could get unlockable when it's expired, so we insert it
                                if slot_index < expiration.slot_index() {
                                    balance.potentially_locked_outputs.insert(*output_id, false);
                                }
                            } else {
                                balance.potentially_locked_outputs.insert(*output_id, false);
                            }
                        }
                    }
                }
            }
        }

        // for `available` get locked_outputs, sum outputs amount and subtract from total_amount
        log::debug!("[BALANCE] locked outputs: {:#?}", wallet_ledger.locked_outputs);

        let mut locked_amount = 0;
        let mut locked_mana = DecayedMana::default();
        let mut locked_native_tokens = NativeTokensBuilder::default();

        for locked_output in &wallet_ledger.locked_outputs {
            // Skip potentially_locked_outputs, as their amounts aren't added to the balance
            if balance.potentially_locked_outputs.contains_key(locked_output) {
                continue;
            }
            if let Some(output_data) = wallet_ledger.unspent_outputs.get(locked_output) {
                // Only check outputs that are in this network
                if output_data.network_id == network_id {
                    locked_amount += output_data.output.amount();
                    locked_mana += output_data.output.decayed_mana(
                        &protocol_parameters,
                        output_data.output_id.transaction_id().slot_index(),
                        slot_index,
                    )?;

                    if let Some(native_token) = output_data.output.native_token() {
                        locked_native_tokens.add_native_token(*native_token)?;
                    }
                }
            }
        }

        log::debug!(
            "[BALANCE] total_amount: {}, total_potential: {}, total_stored: {}, locked_amount: {}, locked_mana: {:?}, total_storage_cost: {}",
            balance.base_coin.total,
            balance.mana.total.potential,
            balance.mana.total.stored,
            locked_amount,
            locked_mana,
            total_storage_cost,
        );

        locked_amount += total_storage_cost;

        for native_token in total_native_tokens.finish_set()? {
            // Check if some amount is currently locked
            let locked_native_token_amount = locked_native_tokens.iter().find_map(|(id, amount)| {
                if id == native_token.token_id() {
                    Some(amount)
                } else {
                    None
                }
            });

            let metadata = wallet_ledger
                .native_token_foundries
                .get(&FoundryId::from(*native_token.token_id()))
                .and_then(|foundry| foundry.immutable_features().metadata())
                .cloned();

            balance.native_tokens.insert(
                *native_token.token_id(),
                NativeTokensBalance {
                    total: native_token.amount(),
                    available: native_token.amount() - *locked_native_token_amount.unwrap_or(&U256::from(0u8)),
                    metadata,
                },
            );
        }

        drop(wallet_ledger);

        for output_id in reward_outputs {
            if let Ok(response) = self.client().get_output_mana_rewards(&output_id, slot_index).await {
                balance.mana.rewards += response.rewards;
            }
        }

        #[cfg(not(feature = "participation"))]
        {
            balance.base_coin.available = balance
                .base_coin
                .total
                .saturating_sub(delegation_implicit_accounts_amount)
                .saturating_sub(locked_amount);
        }
        #[cfg(feature = "participation")]
        {
            balance.base_coin.available = balance
                .base_coin
                .total
                .saturating_sub(delegation_implicit_accounts_amount)
                .saturating_sub(locked_amount)
                .saturating_sub(balance.base_coin.voting_power);
        }
        balance.mana.available = DecayedMana {
            potential: balance.mana.total.potential.saturating_sub(locked_mana.potential),
            stored: balance.mana.total.stored.saturating_sub(locked_mana.stored),
        };

        Ok(balance)
    }
}
