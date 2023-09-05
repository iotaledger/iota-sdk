// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use primitive_types::U256;

use crate::{
    client::secret::SecretManage,
    types::block::{
        address::Bech32Address,
        output::{unlock_condition::UnlockCondition, FoundryId, NativeTokensBuilder, Output, RentCost},
        ConvertTo,
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

impl<S: 'static + SecretManage> Account<S>
where
    Error: From<S::Error>,
{
    /// Get the balance of the account.
    pub async fn balance(&self) -> Result<Balance> {
        log::debug!("[BALANCE] balance");

        let account_details = self.details().await;

        self.balance_inner(account_details.addresses_with_unspent_outputs.iter(), &account_details)
            .await
    }

    /// Get the balance of the given addresses.
    pub async fn addresses_balance(&self, addresses: Vec<impl ConvertTo<Bech32Address>>) -> Result<Balance> {
        log::debug!("[BALANCE] addresses_balance");

        let account_details = self.details().await;

        let addresses_with_unspent_outputs = addresses
            .into_iter()
            .map(|address| {
                let address = address.convert()?;
                account_details
                    .addresses_with_unspent_outputs
                    .iter()
                    .find(|&a| a.address == address)
                    .ok_or(Error::AddressNotFoundInAccount(address))
            })
            .collect::<Result<Vec<&_>>>()?;

        self.balance_inner(addresses_with_unspent_outputs.into_iter(), &account_details)
            .await
    }

    async fn balance_inner(
        &self,
        addresses_with_unspent_outputs: impl Iterator<Item = &AddressWithUnspentOutputs> + Send,
        account_details: &AccountDetails,
    ) -> Result<Balance> {
        let network_id = self.client().get_network_id().await?;
        let rent_structure = self.client().get_rent_structure().await?;
        let mut balance = Balance::default();
        let mut total_rent_amount = 0;
        let mut total_native_tokens = NativeTokensBuilder::default();

        #[cfg(feature = "participation")]
        let voting_output = self.get_voting_output().await?;

        for address_with_unspent_outputs in addresses_with_unspent_outputs {
            #[cfg(feature = "participation")]
            {
                if let Some(voting_output) = &voting_output {
                    if voting_output.output.as_basic().address() == address_with_unspent_outputs.address.inner() {
                        balance.base_coin.voting_power = voting_output.output.amount();
                    }
                }
            }

            for output_id in &address_with_unspent_outputs.output_ids {
                if let Some(data) = account_details.unspent_outputs.get(output_id) {
                    // Check if output is from the network we're currently connected to
                    if data.network_id != network_id {
                        continue;
                    }

                    let output = &data.output;
                    let rent = output.rent_cost(rent_structure);

                    // Add account and foundry outputs here because they can't have a
                    // [`StorageDepositReturnUnlockCondition`] or time related unlock conditions
                    match output {
                        Output::Account(output) => {
                            // Add amount
                            balance.base_coin.total += output.amount();
                            // Add storage deposit
                            balance.required_storage_deposit.account += rent;
                            if !account_details.locked_outputs.contains(output_id) {
                                total_rent_amount += rent;
                            }
                            // Add native tokens
                            total_native_tokens.add_native_tokens(output.native_tokens().clone())?;

                            let account_id = output.account_id_non_null(output_id);
                            balance.accounts.push(account_id);
                        }
                        Output::Foundry(output) => {
                            // Add amount
                            balance.base_coin.total += output.amount();
                            // Add storage deposit
                            balance.required_storage_deposit.foundry += rent;
                            if !account_details.locked_outputs.contains(output_id) {
                                total_rent_amount += rent;
                            }
                            // Add native tokens
                            total_native_tokens.add_native_tokens(output.native_tokens().clone())?;

                            balance.foundries.push(output.id());
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
                                    balance.nfts.push(nft_id);
                                }

                                // Add amount
                                balance.base_coin.total += output.amount();

                                // Add storage deposit
                                if output.is_basic() {
                                    balance.required_storage_deposit.basic += rent;
                                    if output
                                        .native_tokens()
                                        .map(|native_tokens| !native_tokens.is_empty())
                                        .unwrap_or(false)
                                        && !account_details.locked_outputs.contains(output_id)
                                    {
                                        total_rent_amount += rent;
                                    }
                                } else if output.is_nft() {
                                    balance.required_storage_deposit.nft += rent;
                                    if !account_details.locked_outputs.contains(output_id) {
                                        total_rent_amount += rent;
                                    }
                                }

                                // Add native tokens
                                if let Some(native_tokens) = output.native_tokens() {
                                    total_native_tokens.add_native_tokens(native_tokens.clone())?;
                                }
                            } else {
                                // if we have multiple unlock conditions for basic or nft outputs, then we might can't
                                // spend the balance at the moment or in the future

                                let account_addresses = self.addresses().await?;
                                let slot_index = self.client().get_slot_index().await?;
                                let is_claimable =
                                    self.claimable_outputs(OutputsToClaim::All).await?.contains(output_id);

                                // For outputs that are expired or have a timelock unlock condition, but no expiration
                                // unlock condition and we then can unlock them, then
                                // they can never be not available for us anymore
                                // and should be added to the balance
                                if is_claimable {
                                    // check if output can be unlocked always from now on, in that case it should be
                                    // added to the total amount
                                    let output_can_be_unlocked_now_and_in_future =
                                        can_output_be_unlocked_forever_from_now_on(
                                            // We use the addresses with unspent outputs, because other addresses of
                                            // the account without unspent
                                            // outputs can't be related to this output
                                            &account_details.addresses_with_unspent_outputs,
                                            output,
                                            slot_index,
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
                                            balance.nfts.push(nft_id);
                                        }

                                        // Add amount
                                        balance.base_coin.total += amount;

                                        // Add storage deposit
                                        if output.is_basic() {
                                            balance.required_storage_deposit.basic += rent;
                                            // Amount for basic outputs isn't added to total_rent_amount if there aren't
                                            // native tokens, since we can
                                            // spend it without burning.
                                            if output
                                                .native_tokens()
                                                .map(|native_tokens| !native_tokens.is_empty())
                                                .unwrap_or(false)
                                                && !account_details.locked_outputs.contains(output_id)
                                            {
                                                total_rent_amount += rent;
                                            }
                                        } else if output.is_nft() {
                                            balance.required_storage_deposit.nft += rent;
                                            if !account_details.locked_outputs.contains(output_id) {
                                                total_rent_amount += rent;
                                            }
                                        }

                                        // Add native tokens
                                        if let Some(native_tokens) = output.native_tokens() {
                                            total_native_tokens.add_native_tokens(native_tokens.clone())?;
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
            }
        }

        self.finish(
            balance,
            account_details,
            network_id,
            total_rent_amount,
            total_native_tokens,
        )
    }

    fn finish(
        &self,
        mut balance: Balance,
        account_details: &AccountDetails,
        network_id: u64,
        total_rent_amount: u64,
        total_native_tokens: NativeTokensBuilder,
    ) -> Result<Balance> {
        // for `available` get locked_outputs, sum outputs amount and subtract from total_amount
        log::debug!("[BALANCE] locked outputs: {:#?}", account_details.locked_outputs);

        let mut locked_amount = 0;
        let mut locked_native_tokens = NativeTokensBuilder::default();

        for locked_output in &account_details.locked_outputs {
            // Skip potentially_locked_outputs, as their amounts aren't added to the balance
            if balance.potentially_locked_outputs.contains_key(locked_output) {
                continue;
            }
            if let Some(output_data) = account_details.unspent_outputs.get(locked_output) {
                // Only check outputs that are in this network
                if output_data.network_id == network_id {
                    locked_amount += output_data.output.amount();
                    if let Some(native_tokens) = output_data.output.native_tokens() {
                        locked_native_tokens.add_native_tokens(native_tokens.clone())?;
                    }
                }
            }
        }

        log::debug!(
            "[BALANCE] total_amount: {}, locked_amount: {}, total_rent_amount: {}",
            balance.base_coin.total,
            locked_amount,
            total_rent_amount,
        );

        locked_amount += total_rent_amount;

        for native_token in total_native_tokens.finish_set()? {
            // Check if some amount is currently locked
            let locked_native_token_amount = locked_native_tokens.iter().find_map(|(id, amount)| {
                if id == native_token.token_id() {
                    Some(amount)
                } else {
                    None
                }
            });

            let metadata = account_details
                .native_token_foundries
                .get(&FoundryId::from(*native_token.token_id()))
                .and_then(|foundry| foundry.immutable_features().metadata())
                .cloned();

            balance.native_tokens.push(NativeTokensBalance {
                token_id: *native_token.token_id(),
                total: native_token.amount(),
                available: native_token.amount() - *locked_native_token_amount.unwrap_or(&U256::from(0u8)),
                metadata,
            })
        }

        #[cfg(not(feature = "participation"))]
        {
            balance.base_coin.available = balance.base_coin.total.saturating_sub(locked_amount);
        }
        #[cfg(feature = "participation")]
        {
            balance.base_coin.available = balance
                .base_coin
                .total
                .saturating_sub(locked_amount)
                .saturating_sub(balance.base_coin.voting_power);
        }

        Ok(balance)
    }
}
