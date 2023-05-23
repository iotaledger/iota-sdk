# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.wallet.common import _call_method_routine
from iota_sdk.wallet.prepared_transaction_data import PreparedTransactionData, PreparedMintTokenTransaction
from iota_sdk.types.burn import Burn
from iota_sdk.types.native_token import NativeToken
from iota_sdk.types.native_token import NativeToken
from iota_sdk.types.unlock_condition import UnlockCondition
from iota_sdk.types.feature import Feature
from iota_sdk.types.output_id import OutputId
from iota_sdk.types.token_scheme import TokenScheme
from typing import List, Optional, Union


class Account:
    def __init__(self, account_id: Union[str, int], handle):
        self.account_id = account_id
        self.handle = handle

    @_call_method_routine
    def __str__(self):
        message = {
            'name': 'getAccount',
            'data': {
                'accountId': self.account_id,
            }
        }
        return message

    @_call_method_routine
    def _call_account_method(self, method, data=None):
        message = {
            'name': 'callAccountMethod',
            'data': {
                'accountId': self.account_id,
                'method': {
                    'name': method,
                }
            }
        }
        if data:
            message['data']['method']['data'] = data

        return message

    def build_alias_output(self,
                           amount: int,
                           native_tokens: List[NativeToken],
                           alias_id: str,
                           state_index: int,
                           state_metadata: str,
                           foundry_counter: int,
                           unlock_conditions: List[UnlockCondition],
                           features: List[Feature],
                           immutable_features: List[Feature]):
        """Build alias output.
        """
        return self._call_account_method(
            'buildAliasOutput', {
                'amount': amount,
                'nativeTokens': native_tokens,
                'aliasId': alias_id,
                'stateIndex': state_index,
                'stateMetadata': state_metadata,
                'foundryCounter': foundry_counter,
                'unlockConditions': unlock_conditions,
                'features': features,
                'immutableFeatures': immutable_features
            }
        )

    def build_basic_output(self,
                           amount,
                           native_tokens: List[NativeToken],
                           unlock_conditions: List[UnlockCondition],
                           features: List[Feature]):
        """Build basic output.
        """
        return self._call_account_method(
            'buildBasicOutput', {
                'amount': amount,
                'nativeTokens': native_tokens,
                'unlockConditions': unlock_conditions,
                'features': features
            }
        )

    def build_foundry_output(self,
                             amount: int,
                             native_tokens: List[NativeToken],
                             serial_number: int,
                             token_scheme: TokenScheme,
                             unlock_conditions: List[UnlockCondition],
                             features: List[Feature],
                             immutable_features: List[Feature]):
        """Build foundry output.
        """
        return self._call_account_method(
            'buildFoundryOutput', {
                'amount': amount,
                'nativeTokens': native_tokens,
                'serialNumber': serial_number,
                'tokenScheme': token_scheme,
                'unlockConditions': unlock_conditions,
                'features': features,
                'immutableFeatures': immutable_features
            }
        )

    def build_nft_output(self,
                         amount: int,
                         native_tokens: List[NativeToken],
                         nft_id: str,
                         unlock_conditions: List[UnlockCondition],
                         features: List[Feature],
                         immutable_features: List[Feature]):
        """BuildNftOutput.
        """
        return self._call_account_method(
            'buildNftOutput', {
                'amount': amount,
                'nativeTokens': native_tokens,
                'nftId': nft_id,
                'unlockConditions': unlock_conditions,
                'features': features,
                'immutableFeatures': immutable_features
            }
        )

    def prepare_burn(self, burn: Burn, options=None):
        """
        A generic `prepare_burn()` function that can be used to prepare the burn of native tokens, nfts, foundries and aliases.
        """
        prepared = self._call_account_method(
            'prepareBurn', {
                'burn': burn.as_dict(),
                'options': options
            },
        )
        return PreparedTransactionData(account=self, prepared_transaction_data=prepared)

    def prepare_burn_native_token(self,
                          token_id: str,
                          burn_amount: int,
                          options=None):
        """Burn native tokens. This doesn't require the foundry output which minted them, but will not increase
        the foundries `melted_tokens` field, which makes it impossible to destroy the foundry output. Therefore it's
        recommended to use melting, if the foundry output is available.
        """
        prepared = self._call_account_method(
            'prepareBurn', {
                'burn': Burn().add_native_token(NativeToken(token_id, burn_amount)).as_dict(),
                'options': options
            },
        )
        return PreparedTransactionData(account=self, prepared_transaction_data=prepared)

    def prepare_burn_nft(self,
                 nft_id: str,
                 options=None):
        """Burn an nft output.
        """
        prepared = self._call_account_method(
            'prepareBurn', {
                'burn': Burn().add_nft(nft_id).as_dict(),
                'options': options
            },
        )
        return PreparedTransactionData(account=self, prepared_transaction_data=prepared)

    def prepare_consolidate_outputs(self,
                            force: bool,
                            output_consolidation_threshold: Optional[int] = None):
        """Consolidate outputs.
        """
        prepared = self._call_account_method(
            'prepareConsolidateOutputs', {
                'force': force,
                'outputConsolidationThreshold': output_consolidation_threshold
            }
        )
        return PreparedTransactionData(account=self, prepared_transaction_data=prepared)

    def prepare_create_alias_output(self,
                            params,
                            options):
        """Create an alias output.
        """
        prepared = self._call_account_method(
            'prepareCreateAliasOutput', {
                'params': params,
                'options': options
            }
        )
        return PreparedTransactionData(account=self, prepared_transaction_data=prepared)

    def prepare_destroy_alias(self,
                      alias_id: str,
                      options=None):
        """Destroy an alias output.
        """

        prepared = self._call_account_method(
            'prepareBurn', {
                'burn': Burn().add_alias(alias_id).as_dict(),
                'options': options
            },
        )
        return PreparedTransactionData(account=self, prepared_transaction_data=prepared)

    def prepare_destroy_foundry(self,
                        foundry_id: str,
                        options=None):
        """Destroy a foundry output with a circulating supply of 0.
        """
        prepared = self._call_account_method(
            'prepareBurn', {
                'burn': Burn().add_foundry(foundry_id).as_dict(),
                'options': options
            },
        )
        return PreparedTransactionData(account=self, prepared_transaction_data=prepared)

    def generate_addresses(self, amount: int, options=None):
        """Generate new addresses.
        """
        return self._call_account_method(
            'generateAddresses', {
                'amount': amount,
                'options': options
            }
        )

    def get_outputs_with_additional_unlock_conditions(self, outputs_to_claim: List[OutputId]):
        """Get outputs with additional unlock conditions.
        """
        return self._call_account_method(
            'getOutputsWithAdditionalUnlockConditions', {
                'outputsToClaim': outputs_to_claim
            }
        )

    def get_output(self, output_id: OutputId):
        """Get output.
        """
        return self._call_account_method(
            'getOutput', {
                'outputId': output_id
            }
        )

    def get_transaction(self, transaction_id: str):
        """Get transaction.
        """
        return self._call_account_method(
            'getTransaction', {
                'transactionId': transaction_id
            }
        )

    def addresses(self):
        """List addresses.
        """
        return self._call_account_method(
            'addresses'
        )

    def addresses_with_unspent_outputs(self):
        """Returns only addresses of the account with unspent outputs.
        """
        return self._call_account_method(
            'addressesWithUnspentOutputs'
        )

    def outputs(self, filter_options=None):
        """Returns all outputs of the account.
        """
        return self._call_account_method(
            'outputs', {
                'filterOptions': filter_options
            }
        )

    def unspent_outputs(self, filter_options=None):
        """Returns all unspent outputs of the account.
        """
        return self._call_account_method(
            'unspentOutputs', {
                'filterOptions': filter_options
            }
        )

    def incoming_transactions(self):
        """Returns all incoming transactions of the account.
        """
        return self._call_account_method(
            'incomingTransactions'
        )

    def transactions(self):
        """Returns all transaction of the account.
        """
        return self._call_account_method(
            'transactions'
        )

    def pending_transactions(self):
        """Returns all pending transactions of the account.
        """
        return self._call_account_method(
            'pendingTransactions'
        )

    def prepare_decrease_native_token_supply(self,
                                     token_id: str,
                                     melt_amount: int,
                                     options=None):
        """Melt native tokens. This happens with the foundry output which minted them, by increasing it's
        `melted_tokens` field.
        """
        prepared = self._call_account_method(
            'prepareDecreaseNativeTokenSupply', {
                'tokenId': token_id,
                'meltAmount': melt_amount,
                'options': options
            }
        )
        return PreparedTransactionData(account=self, prepared_transaction_data=prepared)

    def prepare_increase_native_token_supply(self, token_id: str, mint_amount: int, options=None):
        """Mint more native token.
        """
        prepared = self._call_account_method(
            'prepareIncreaseNativeTokenSupply', {
                'tokenId': token_id,
                'mintAmount': mint_amount,
                'options': options
            }
        )
        return PreparedMintTokenTransaction(account=self, prepared_transaction_data=prepared)

    def prepare_mint_native_token(self, params, options=None):
        """Mint native token.
        """
        prepared = self._call_account_method(
            'prepareMintNativeToken', {
                'params': params,
                'options': options
            }
        )
        return PreparedMintTokenTransaction(account=self, prepared_transaction_data=prepared)

    def minimum_required_storage_deposit(self, output):
        """Minimum required storage deposit.
        """
        return self._call_account_method(
            'minimumRequiredStorageDeposit', {
                'output': output
            }
        )

    def prepare_mint_nfts(self, params, options=None):
        """Mint nfts.
        """
        prepared = self._call_account_method(
            'prepareMintNfts', {
                'params': params,
                'options': options
            }
        )
        return PreparedTransactionData(account=self, prepared_transaction_data=prepared)

    def get_balance(self):
        """Get account balance information.
        """
        return self._call_account_method(
            'getBalance'
        )

    def prepare_output(self, output_options, transaction_options=None):
        """Prepare an output for sending
           If the amount is below the minimum required storage deposit, by default the remaining amount will automatically
           be added with a StorageDepositReturn UnlockCondition, when setting the ReturnStrategy to `gift`, the full
           minimum required storage deposit will be sent to the recipient.
           When the assets contain an nft_id, the data from the existing nft output will be used, just with the address
           unlock conditions replaced
        """
        return self._call_account_method(
            'prepareOutput', {
                'options': output_options,
                'transactionOptions': transaction_options
            }
        )

    def prepare_send_amount(self, params, options=None):
        """Prepare send amount.
        """
        prepared = self._call_account_method(
            'prepareSendAmount', {
                'params': params,
                'options': options
            }
        )
        return PreparedTransactionData(account=self, prepared_transaction_data=prepared)

    def prepare_transaction(self, outputs, options=None):
        """Prepare transaction.
        """
        prepared = self._call_account_method(
            'prepareTransaction', {
                'outputs': outputs,
                'options': options
            }
        )
        return PreparedTransactionData(account=self, prepared_transaction_data=prepared)

    def retry_transaction_until_included(self, transaction_id: str, interval=None, max_attempts=None):
        """Retries (promotes or reattaches) a transaction sent from the account for a provided transaction id until it's
        included (referenced by a milestone). Returns the included block id.
        """
        return self._call_account_method(
            'retryTransactionUntilIncluded', {
                'transactionId': transaction_id,
                'interval': interval,
                'maxAttempts': max_attempts
            }
        )

    def sync(self, options=None):
        """Sync the account by fetching new information from the nodes.
           Will also retry pending transactions and consolidate outputs if necessary.
           A custom default can be set using set_default_sync_options
        """
        return self._call_account_method(
            'sync', {
                'options': options,
            }
        )

    def send_amount(self, params, options=None):
        """Send amount.
        """
        return self._call_account_method(
            'sendAmount', {
                'params': params,
                'options': options
            }
        )

    def prepare_send_native_tokens(self, params, options=None):
        """Send native tokens.
        """
        prepared = self._call_account_method(
            'prepareSendNativeTokens', {
                'params': params,
                'options': options
            }
        )
        return PreparedTransactionData(account=self, prepared_transaction_data=prepared)

    def prepare_send_nft(self, params, options=None):
        """Send nft.
        """
        prepared = self._call_account_method(
            'prepareSendNft', {
                'params': params,
                'options': options
            }
        )
        return PreparedTransactionData(account=self, prepared_transaction_data=prepared)

    def set_alias(self, alias: str):
        """Set alias.
        """
        return self._call_account_method(
            'setAlias', {
                'alias': alias
            }
        )

    def set_default_sync_options(self, options):
        """Set the fallback SyncOptions for account syncing.
           If storage is enabled, will persist during restarts.
        """
        return self._call_account_method(
            'setDefaultSyncOptions', {
                'options': options
            }
        )

    def sign_transaction_essence(self, prepared_transaction_data):
        """Sign a transaction essence.
        """
        return self._call_account_method(
            'signTransactionEssence', {
                'preparedTransactionData': prepared_transaction_data
            }
        )

    def sign_and_submit_transaction(self, prepared_transaction_data):
        """Validate the transaction, sign it, submit it to a node and store it in the account.
        """
        return self._call_account_method(
            'signAndSubmitTransaction', {
                'preparedTransactionData': prepared_transaction_data
            }
        )

    def submit_and_store_transaction(self, signed_transaction_data):
        """Submit and store transaction.
        """
        return self._call_account_method(
            'submitAndStoreTransaction', {
                'signedTransactionData': signed_transaction_data
            }
        )

    def claim_outputs(self, output_ids_to_claim: List[OutputId]):
        """Claim outputs.
        """
        return self._call_account_method(
            'claimOutputs', {
                'outputIdsToClaim': output_ids_to_claim
            }
        )

    def send_outputs(self, outputs, options=None):
        """Send outputs in a transaction.
        """
        return self._call_account_method(
            'sendOutputs', {
                'outputs': outputs,
                'options': options,
            }
        )
