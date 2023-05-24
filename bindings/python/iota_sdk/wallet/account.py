# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.wallet.common import _call_method_routine
from iota_sdk.types.burn import Burn
from iota_sdk.types.common import HexStr
from iota_sdk.types.native_token import NativeToken
from iota_sdk.types.output_id import OutputId
from typing import List, Optional


class Account:
    def __init__(self, account_id: str | int, handle):
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

    def burn(self, burn: Burn, options=None):
        """
        A generic `burn()` function that can be used to burn native tokens, nfts, foundries and aliases.
        """
        return self._call_account_method(
            'burn', {
                'burn': burn.as_dict(),
                'options': options
            },
        )

    def burn_native_token(self,
                          token_id: HexStr,
                          burn_amount: int,
                          options=None):
        """Burn native tokens. This doesn't require the foundry output which minted them, but will not increase
        the foundries `melted_tokens` field, which makes it impossible to destroy the foundry output. Therefore it's
        recommended to use melting, if the foundry output is available.
        """
        return self._call_account_method(
            'burn', {
                'burn': Burn().add_native_token(NativeToken(token_id, burn_amount)).as_dict(),
                'options': options
            },
        )

    def burn_nft(self,
                 nft_id: HexStr,
                 options=None):
        """Burn an nft output.
        """
        return self._call_account_method(
            'burn', {
                'burn': Burn().add_nft(nft_id).as_dict(),
                'options': options
            },
        )

    def consolidate_outputs(self,
                            force: bool,
                            output_consolidation_threshold: Optional[int] = None):
        """Consolidate outputs.
        """
        return self._call_account_method(
            'consolidateOutputs', {
                'force': force,
                'outputConsolidationThreshold': output_consolidation_threshold
            }
        )

    def create_alias_output(self,
                            params,
                            options):
        """Create an alias output.
        """
        return self._call_account_method(
            'createAliasOutput', {
                'params': params,
                'options': options
            }
        )

    def destroy_alias(self,
                      alias_id: HexStr,
                      options=None):
        """Destroy an alias output.
        """

        return self._call_account_method(
            'burn', {
                'burn': Burn().add_alias(alias_id).as_dict(),
                'options': options
            },
        )

    def destroy_foundry(self,
                        foundry_id: HexStr,
                        options=None):
        """Destroy a foundry output with a circulating supply of 0.
        """
        return self._call_account_method(
            'burn', {
                'burn': Burn().add_foundry(foundry_id).as_dict(),
                'options': options
            },
        )
        pass

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

    def get_transaction(self, transaction_id: HexStr):
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

    def decrease_native_token_supply(self,
                                     token_id: HexStr,
                                     melt_amount: int,
                                     options=None):
        """Melt native tokens. This happens with the foundry output which minted them, by increasing it's
        `melted_tokens` field.
        """
        return self._call_account_method(
            'decreaseNativeTokenSupply', {
                'tokenId': token_id,
                'meltAmount': melt_amount,
                'options': options
            }
        )

    def increase_native_token_supply(self, token_id: HexStr, mint_amount: int, options=None):
        """Mint more native token.
        """
        return self._call_account_method(
            'increaseNativeTokenSupply', {
                'tokenId': token_id,
                'mintAmount': mint_amount,
                'options': options
            }
        )

    def mint_native_token(self, params, options=None):
        """Mint native token.
        """
        return self._call_account_method(
            'mintNativeToken', {
                'params': params,
                'options': options
            }
        )

    def minimum_required_storage_deposit(self, output):
        """Minimum required storage deposit.
        """
        return self._call_account_method(
            'minimumRequiredStorageDeposit', {
                'output': output
            }
        )

    def mint_nfts(self, params, options=None):
        """Mint nfts.
        """
        return self._call_account_method(
            'mintNfts', {
                'params': params,
                'options': options
            }
        )

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
        return self._call_account_method(
            'prepareSendAmount', {
                'params': params,
                'options': options
            }
        )

    def prepare_transaction(self, outputs, options=None):
        """Prepare transaction.
        """
        return self._call_account_method(
            'prepareTransaction', {
                'outputs': outputs,
                'options': options
            }
        )

    def retry_transaction_until_included(self, transaction_id: HexStr, interval=None, max_attempts=None):
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

    def send_native_tokens(self, params, options=None):
        """Send native tokens.
        """
        return self._call_account_method(
            'sendNativeTokens', {
                'params': params,
                'options': options
            }
        )

    def send_nft(self, params, options=None):
        """Send nft.
        """
        return self._call_account_method(
            'sendNft', {
                'params': params,
                'options': options
            }
        )

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
