// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use primitive_types::U256;

use crate::{
    types::block::output::{
        unlock_condition::UnlockCondition, AliasId, AliasOutput, FoundryId, FoundryOutput, NativeTokensBuilder, NftId,
        Output, OutputId, Rent,
    },
    wallet::{
        account::{
            handle::AccountHandle,
            operations::helpers::time::can_output_be_unlocked_forever_from_now_on,
            types::{AccountBalance, BaseCoinBalance, NativeTokenBalance, RequiredStorageDeposit},
            OutputsToClaim,
        },
        Result,
    },
};

#[derive(Default)]
pub struct AccountBalanceBuilder {
    base_coin: BaseCoinBalance,
    required_storage_deposit: RequiredStorageDeposit,
    native_tokens: Vec<NativeTokenBalance>,
    nfts: Vec<NftId>,
    aliases: Vec<AliasId>,
    foundries: Vec<FoundryId>,
    potentially_locked_outputs: HashMap<OutputId, bool>,

    locked_amount: u64,
    locked_native_tokens: NativeTokensBuilder,
    total_rent_amount: u64,
    total_native_tokens: NativeTokensBuilder,
}

impl AccountBalanceBuilder {
    fn add_alias(&mut self, output: &AliasOutput, output_id: &OutputId, rent: u64, locked: bool) -> Result<()> {
        // Add amount
        self.base_coin.total += output.amount();
        // Add storage deposit
        self.required_storage_deposit.alias += rent;
        if !locked {
            self.total_rent_amount += rent;
        }

        // Add native tokens
        self.total_native_tokens
            .add_native_tokens(output.native_tokens().clone())?;

        let alias_id = output.alias_id_non_null(output_id);
        self.aliases.push(alias_id);

        Ok(())
    }

    fn add_foundry(&mut self, output: &FoundryOutput, rent: u64, locked: bool) -> Result<()> {
        // Add amount
        self.base_coin.total += output.amount();
        // Add storage deposit
        self.required_storage_deposit.foundry += rent;
        if !locked {
            self.total_rent_amount += rent;
        }

        // Add native tokens
        self.total_native_tokens
            .add_native_tokens(output.native_tokens().clone())?;

        self.foundries.push(output.id());

        Ok(())
    }

    fn build(
        mut self,
        // TODO that way ?
        native_token_foundries: &HashMap<FoundryId, FoundryOutput>,
    ) -> Result<AccountBalance> {
        self.locked_amount += self.total_rent_amount;

        for native_token in self.total_native_tokens.finish_vec()? {
            // Check if some amount is currently locked
            let locked_amount = self.locked_native_tokens.get(native_token.token_id());
            let metadata = native_token_foundries
                .get(&FoundryId::from(*native_token.token_id()))
                .and_then(|foundry| foundry.immutable_features().metadata())
                .cloned();

            self.native_tokens.push(NativeTokenBalance {
                token_id: *native_token.token_id(),
                metadata,
                total: native_token.amount(),
                available: native_token.amount() - *locked_amount.unwrap_or(&U256::from(0u8)),
            })
        }

        #[cfg(not(feature = "participation"))]
        {
            self.base_coin.available = self.base_coin.total.saturating_sub(locked_amount);
        }
        #[cfg(feature = "participation")]
        {
            self.base_coin.available = self
                .base_coin
                .total
                .saturating_sub(self.locked_amount)
                .saturating_sub(self.base_coin.voting_power);
        }

        Ok(AccountBalance {
            base_coin: self.base_coin,
            required_storage_deposit: self.required_storage_deposit,
            native_tokens: self.native_tokens,
            nfts: self.nfts,
            aliases: self.aliases,
            foundries: self.foundries,
            potentially_locked_outputs: self.potentially_locked_outputs,
        })
    }
}

impl AccountHandle {
    /// Get the AccountBalance
    pub async fn balance(&self) -> Result<AccountBalance> {
        log::debug!("[BALANCE] get balance");
        let mut balance_builder = AccountBalanceBuilder::default();
        #[cfg(feature = "participation")]
        {
            balance_builder.base_coin.voting_power = self.get_voting_power().await?;
        }

        let unlockable_outputs_with_multiple_unlock_conditions = self
            .get_unlockable_outputs_with_additional_unlock_conditions(OutputsToClaim::All)
            .await?;

        let account_addresses = self.addresses().await?;

        let network_id = self.client.get_network_id().await?;
        let rent_structure = self.client.get_rent_structure().await?;

        let local_time = self.client.get_time_checked().await?;

        let account = self.read().await;

        // for `available` get locked_outputs, sum outputs amount and subtract from total_amount
        log::debug!("[BALANCE] locked outputs: {:#?}", account.locked_outputs);

        let relevant_unspent_outputs = account
            .unspent_outputs
            .values()
            // Check if output is from the network we're currently connected to
            .filter(|data| data.network_id == network_id)
            .map(|data| (&data.output_id, &data.output));

        for (output_id, output) in relevant_unspent_outputs {
            let rent = output.rent_cost(&rent_structure);
            let locked = account.locked_outputs.contains(output_id);

            if locked {
                balance_builder.locked_amount += output.amount();
                if let Some(native_tokens) = output.native_tokens() {
                    balance_builder
                        .locked_native_tokens
                        .add_native_tokens(native_tokens.clone())?;
                }
            }

            // Add alias and foundry outputs here because they can't have a [`StorageDepositReturnUnlockCondition`]
            // or time related unlock conditions
            match output {
                Output::Alias(output) => balance_builder.add_alias(output, output_id, rent, locked)?,
                Output::Foundry(output) => balance_builder.add_foundry(output, rent, locked)?,
                _ => {
                    // If there is only an [AddressUnlockCondition], then we can spend the output at any time without
                    // restrictions
                    if let [UnlockCondition::Address(_)] = output
                        .unlock_conditions()
                        .expect("output needs to have unlock conditions")
                        .as_ref()
                    {
                        // add nft_id for nft outputs
                        if let Output::Nft(output) = &output {
                            let nft_id = output.nft_id_non_null(output_id);
                            balance_builder.nfts.push(nft_id);
                        }

                        // Add amount
                        balance_builder.base_coin.total += output.amount();

                        // Add storage deposit
                        if output.is_basic() {
                            balance_builder.required_storage_deposit.basic += rent;
                            if output
                                .native_tokens()
                                .map(|native_tokens| !native_tokens.is_empty())
                                .unwrap_or(false)
                                && !locked
                            {
                                balance_builder.total_rent_amount += rent;
                            }
                        } else if output.is_nft() {
                            balance_builder.required_storage_deposit.nft += rent;
                            if !locked {
                                balance_builder.total_rent_amount += rent;
                            }
                        }

                        // Add native tokens
                        if let Some(native_tokens) = output.native_tokens() {
                            balance_builder
                                .total_native_tokens
                                .add_native_tokens(native_tokens.clone())?;
                        }
                    } else {
                        // if we have multiple unlock conditions for basic or nft outputs, then we might can't spend the
                        // balance at the moment or in the future

                        let output_can_be_unlocked_now =
                            unlockable_outputs_with_multiple_unlock_conditions.contains(output_id);

                        // For outputs that are expired or have a timelock unlock condition, but no expiration unlock
                        // condition and we then can unlock them, then they can never be not available for us anymore
                        // and should be added to the balance
                        if output_can_be_unlocked_now {
                            // check if output can be unlocked always from now on, in that case it should be added to
                            // the total amount
                            let output_can_be_unlocked_now_and_in_future = can_output_be_unlocked_forever_from_now_on(
                                // We use the addresses with unspent outputs, because other addresses of the
                                // account without unspent outputs can't be related to this output
                                &account.addresses_with_unspent_outputs,
                                output,
                                local_time,
                            );

                            if output_can_be_unlocked_now_and_in_future {
                                // If output has a StorageDepositReturnUnlockCondition, the amount of it should be
                                // subtracted, because this part needs to be sent back
                                let amount = output
                                    .unlock_conditions()
                                    .and_then(|u| u.storage_deposit_return())
                                    .map_or_else(
                                        || output.amount(),
                                        |sdr| {
                                            if account_addresses
                                                .iter()
                                                .any(|a| a.address.inner == *sdr.return_address())
                                            {
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
                                    balance_builder.nfts.push(nft_id);
                                }

                                // Add amount
                                balance_builder.base_coin.total += amount;

                                // Add storage deposit
                                if output.is_basic() {
                                    balance_builder.required_storage_deposit.basic += rent;
                                    // Amount for basic outputs isn't added to total_rent_amount if there aren't native
                                    // tokens, since we can spend it without burning.
                                    if output
                                        .native_tokens()
                                        .map(|native_tokens| !native_tokens.is_empty())
                                        .unwrap_or(false)
                                        && !locked
                                    {
                                        balance_builder.total_rent_amount += rent;
                                    }
                                } else if output.is_nft() {
                                    balance_builder.required_storage_deposit.nft += rent;
                                    if !locked {
                                        balance_builder.total_rent_amount += rent;
                                    }
                                }

                                // Add native tokens
                                if let Some(native_tokens) = output.native_tokens() {
                                    balance_builder
                                        .total_native_tokens
                                        .add_native_tokens(native_tokens.clone())?;
                                }
                            } else {
                                // only add outputs that can't be locked now and at any point in the future
                                balance_builder.potentially_locked_outputs.insert(*output_id, true);
                            }
                        } else {
                            // Don't add expired outputs that can't ever be unlocked by us
                            if let Some(expiration) = output
                                .unlock_conditions()
                                .expect("output needs to have unlock conditions")
                                .expiration()
                            {
                                // Not expired, could get unlockable when it's expired, so we insert it
                                if local_time < expiration.timestamp() {
                                    balance_builder.potentially_locked_outputs.insert(*output_id, false);
                                }
                            } else {
                                balance_builder.potentially_locked_outputs.insert(*output_id, false);
                            }
                        }
                    }
                }
            }
        }

        log::debug!(
            "[BALANCE] total_amount: {}, locked_amount: {}, total_rent_amount: {}",
            balance_builder.base_coin.total,
            balance_builder.locked_amount,
            balance_builder.total_rent_amount,
        );

        balance_builder.build(&account.native_token_foundries)
    }
}
