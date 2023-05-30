// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use primitive_types::U256;

use crate::{
    types::block::{
        address::Bech32AddressLike,
        output::{unlock_condition::UnlockCondition, FoundryId, NativeTokensBuilder, Output, Rent, RentStructure},
    },
    wallet::{
        account::{
            operations::helpers::time::can_output_be_unlocked_forever_from_now_on,
            types::{AddressWithUnspentOutputs, Balance, NativeTokensBalance},
            Account, AccountDetails, OutputsToClaim,
        },
        Error, Result,
    },
};

struct BalanceContext<'a> {
    account_details: &'a AccountDetails,
    network_id: u64,
    rent_structure: RentStructure,
    balance: Balance,
    total_rent_amount: u64,
    total_native_tokens: NativeTokensBuilder,
    locked_amount: u64,
    locked_native_tokens: NativeTokensBuilder,
}

impl<'a> BalanceContext<'a> {
    fn new(account_details: &'a AccountDetails, network_id: u64, rent_structure: RentStructure) -> Self {
        Self {
            account_details,
            network_id,
            rent_structure,
            balance: Balance::default(),
            total_rent_amount: 0,
            total_native_tokens: NativeTokensBuilder::default(),
            locked_amount: 0,
            locked_native_tokens: NativeTokensBuilder::default(),
        }
    }

    fn finish(mut self) -> Result<Balance> {
        // for `available` get locked_outputs, sum outputs amount and subtract from total_amount
        log::debug!("[BALANCE] locked outputs: {:#?}", self.account_details.locked_outputs);

        for locked_output in &self.account_details.locked_outputs {
            // Skip potentially_locked_outputs, as their amounts aren't added to the balance
            if self.balance.potentially_locked_outputs.contains_key(locked_output) {
                continue;
            }
            if let Some(output_data) = self.account_details.unspent_outputs.get(locked_output) {
                // Only check outputs that are in this network
                if output_data.network_id == self.network_id {
                    self.locked_amount += output_data.output.amount();
                    if let Some(native_tokens) = output_data.output.native_tokens() {
                        self.locked_native_tokens.add_native_tokens(native_tokens.clone())?;
                    }
                }
            }
        }

        log::debug!(
            "[BALANCE] total_amount: {}, locked_amount: {}, total_rent_amount: {}",
            self.balance.base_coin.total,
            self.locked_amount,
            self.total_rent_amount,
        );

        self.locked_amount += self.total_rent_amount;

        for native_token in self.total_native_tokens.finish_set()? {
            // Check if some amount is currently locked
            let locked_native_token_amount = self.locked_native_tokens.iter().find_map(|(id, amount)| {
                if id == native_token.token_id() {
                    Some(amount)
                } else {
                    None
                }
            });

            let metadata = self
                .account_details
                .native_token_foundries
                .get(&FoundryId::from(*native_token.token_id()))
                .and_then(|foundry| foundry.immutable_features().metadata())
                .cloned();

            self.balance.native_tokens.push(NativeTokensBalance {
                token_id: *native_token.token_id(),
                metadata,
                total: native_token.amount(),
                available: native_token.amount() - *locked_native_token_amount.unwrap_or(&U256::from(0u8)),
            })
        }

        #[cfg(not(feature = "participation"))]
        {
            self.balance.base_coin.available = self.balance.base_coin.total.saturating_sub(self.locked_amount);
        }
        #[cfg(feature = "participation")]
        {
            self.balance.base_coin.available = self
                .balance
                .base_coin
                .total
                .saturating_sub(self.locked_amount)
                .saturating_sub(self.balance.base_coin.voting_power);
        }

        Ok(self.balance)
    }
}

impl Account {
    async fn balance_inner<'a>(
        &self,
        address_with_unspent_outputs: &AddressWithUnspentOutputs,
        context: &mut BalanceContext<'a>,
    ) -> Result<()> {
        for output_id in &address_with_unspent_outputs.output_ids {
            if let Some(data) = context.account_details.unspent_outputs.get(output_id) {
                // Check if output is from the network we're currently connected to
                if data.network_id != context.network_id {
                    continue;
                }

                let output = &data.output;
                let rent = output.rent_cost(&context.rent_structure);

                // Add alias and foundry outputs here because they can't have a
                // [`StorageDepositReturnUnlockCondition`] or time related unlock conditions
                match output {
                    Output::Alias(output) => {
                        // Add amount
                        context.balance.base_coin.total += output.amount();
                        // Add storage deposit
                        context.balance.required_storage_deposit.alias += rent;
                        if !context.account_details.locked_outputs.contains(output_id) {
                            context.total_rent_amount += rent;
                        }

                        // Add native tokens
                        context
                            .total_native_tokens
                            .add_native_tokens(output.native_tokens().clone())?;

                        let alias_id = output.alias_id_non_null(output_id);
                        context.balance.aliases.push(alias_id);
                    }
                    Output::Foundry(output) => {
                        // Add amount
                        context.balance.base_coin.total += output.amount();
                        // Add storage deposit
                        context.balance.required_storage_deposit.foundry += rent;
                        if !context.account_details.locked_outputs.contains(output_id) {
                            context.total_rent_amount += rent;
                        }

                        // Add native tokens
                        context
                            .total_native_tokens
                            .add_native_tokens(output.native_tokens().clone())?;

                        context.balance.foundries.push(output.id());
                    }
                    _ => {
                        // If there is only an [AddressUnlockCondition], then we can spend the output at any time
                        // without restrictions
                        if let [UnlockCondition::Address(_)] = output
                            .unlock_conditions()
                            .expect("output needs to have unlock conditions")
                            .as_ref()
                        {
                            // add nft_id for nft outputs
                            if let Output::Nft(output) = &output {
                                let nft_id = output.nft_id_non_null(output_id);
                                context.balance.nfts.push(nft_id);
                            }

                            // Add amount
                            context.balance.base_coin.total += output.amount();

                            // Add storage deposit
                            if output.is_basic() {
                                context.balance.required_storage_deposit.basic += rent;
                                if output
                                    .native_tokens()
                                    .map(|native_tokens| !native_tokens.is_empty())
                                    .unwrap_or(false)
                                    && !context.account_details.locked_outputs.contains(output_id)
                                {
                                    context.total_rent_amount += rent;
                                }
                            } else if output.is_nft() {
                                context.balance.required_storage_deposit.nft += rent;
                                if !context.account_details.locked_outputs.contains(output_id) {
                                    context.total_rent_amount += rent;
                                }
                            }

                            // Add native tokens
                            if let Some(native_tokens) = output.native_tokens() {
                                context.total_native_tokens.add_native_tokens(native_tokens.clone())?;
                            }
                        } else {
                            // if we have multiple unlock conditions for basic or nft outputs, then we might can't
                            // spend the balance at the moment or in the future

                            let account_addresses = self.addresses().await?;
                            let local_time = self.client().get_time_checked().await?;
                            let output_can_be_unlocked_now = self
                                .get_unlockable_outputs_with_additional_unlock_conditions(OutputsToClaim::All)
                                .await?
                                .contains(output_id);

                            // For outputs that are expired or have a timelock unlock condition, but no expiration
                            // unlock condition and we then can unlock them, then
                            // they can never be not available for us anymore
                            // and should be added to the balance
                            if output_can_be_unlocked_now {
                                // check if output can be unlocked always from now on, in that case it should be
                                // added to the total amount
                                let output_can_be_unlocked_now_and_in_future =
                                    can_output_be_unlocked_forever_from_now_on(
                                        // We use the addresses with unspent outputs, because other addresses of
                                        // the account without unspent
                                        // outputs can't be related to this output
                                        &context.account_details.addresses_with_unspent_outputs,
                                        output,
                                        local_time,
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
                                        context.balance.nfts.push(nft_id);
                                    }

                                    // Add amount
                                    context.balance.base_coin.total += amount;

                                    // Add storage deposit
                                    if output.is_basic() {
                                        context.balance.required_storage_deposit.basic += rent;
                                        // Amount for basic outputs isn't added to total_rent_amount if there aren't
                                        // native tokens, since we can
                                        // spend it without burning.
                                        if output
                                            .native_tokens()
                                            .map(|native_tokens| !native_tokens.is_empty())
                                            .unwrap_or(false)
                                            && !context.account_details.locked_outputs.contains(output_id)
                                        {
                                            context.total_rent_amount += rent;
                                        }
                                    } else if output.is_nft() {
                                        context.balance.required_storage_deposit.nft += rent;
                                        if !context.account_details.locked_outputs.contains(output_id) {
                                            context.total_rent_amount += rent;
                                        }
                                    }

                                    // Add native tokens
                                    if let Some(native_tokens) = output.native_tokens() {
                                        context.total_native_tokens.add_native_tokens(native_tokens.clone())?;
                                    }
                                } else {
                                    // only add outputs that can't be locked now and at any point in the future
                                    context.balance.potentially_locked_outputs.insert(*output_id, true);
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
                                        context.balance.potentially_locked_outputs.insert(*output_id, false);
                                    }
                                } else {
                                    context.balance.potentially_locked_outputs.insert(*output_id, false);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Get the balance of the given addresses.
    pub async fn addresses_balance(&self, addresses: Vec<impl Bech32AddressLike>) -> Result<Balance> {
        log::debug!("[BALANCE] addresses_balance");

        let account_details = self.details().await;
        let network_id = self.client().get_network_id().await?;
        let rent_structure = self.client().get_rent_structure().await?;
        let mut context = BalanceContext::new(&account_details, network_id, rent_structure);

        #[cfg(feature = "participation")]
        let voting_output = self.get_voting_output().await?;

        for address in addresses {
            let address = address.to_bech32()?;

            if let Some(address_with_unspent_outputs) = account_details
                .addresses_with_unspent_outputs
                .iter()
                .find(|a| a.address == address)
            {
                #[cfg(feature = "participation")]
                {
                    if let Some(voting_output) = &voting_output {
                        if voting_output.output.as_basic().address() == address.inner() {
                            context.balance.base_coin.voting_power = voting_output.output.amount();
                        }
                    }
                }
                self.balance_inner(address_with_unspent_outputs, &mut context).await?;
            } else {
                return Err(Error::AddressNotFoundInAccount(address));
            }
        }

        context.finish()
    }

    /// Get the balance of the account.
    pub async fn balance(&self) -> crate::wallet::Result<Balance> {
        log::debug!("[BALANCE] balance");

        let account_details = self.details().await;
        let network_id = self.client().get_network_id().await?;
        let rent_structure = self.client().get_rent_structure().await?;
        let mut context = BalanceContext::new(&account_details, network_id, rent_structure);

        #[cfg(feature = "participation")]
        {
            context.balance.base_coin.voting_power = self.get_voting_power().await?;
        }

        for address_with_unspent_outputs in &account_details.addresses_with_unspent_outputs {
            self.balance_inner(address_with_unspent_outputs, &mut context).await?;
        }

        context.finish()
    }
}
