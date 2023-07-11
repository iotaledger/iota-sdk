# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.wallet.common import _call_method_routine
from iota_sdk.wallet.prepared_transaction_data import PreparedTransactionData, PreparedCreateTokenTransaction
from iota_sdk.wallet.sync_options import SyncOptions
from iota_sdk.types.balance import Balance
from iota_sdk.types.burn import Burn
from iota_sdk.types.common import HexStr
from iota_sdk.types.native_token import NativeToken
from iota_sdk.types.output_data import OutputData
from iota_sdk.types.output_id import OutputId
from iota_sdk.types.transaction import Transaction
from iota_sdk.types.transaction_options import TransactionOptions
from typing import List, Optional
from dacite import from_dict

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

    def prepare_burn(self, burn: Burn, options: Optional[TransactionOptions] = None) -> PreparedTransactionData:
        """
        A generic `prepare_burn()` function that can be used to prepare the burn of native tokens, nfts, foundries and aliases.
        """
        prepared = self._call_account_method(
            'prepareBurn', {
                'burn': burn.as_dict(),
                'options': options
            },
        )
        return PreparedTransactionData(self, prepared)

    def prepare_burn_native_token(self,
                                  token_id: HexStr,
                                  burn_amount: int,
                                  options: Optional[TransactionOptions] = None) -> PreparedTransactionData:
        """Burn native tokens. This doesn't require the foundry output which minted them, but will not increase
        the foundries `melted_tokens` field, which makes it impossible to destroy the foundry output. Therefore it's
        recommended to use melting, if the foundry output is available.
        """
        prepared = self._call_account_method(
            'prepareBurn', {
                'burn': Burn().add_native_token(NativeToken(token_id, hex(burn_amount))).as_dict(),
                'options': options
            },
        )
        return PreparedTransactionData(self, prepared)

    def prepare_burn_nft(self,
                         nft_id: HexStr,
                         options: Optional[TransactionOptions] = None) -> PreparedTransactionData:
        """Burn an nft output.
        """
        prepared = self._call_account_method(
            'prepareBurn', {
                'burn': Burn().add_nft(nft_id).as_dict(),
                'options': options
            },
        )
        return PreparedTransactionData(self, prepared)

    def prepare_consolidate_outputs(self,
                                    force: bool,
                                    output_consolidation_threshold: Optional[int] = None) -> PreparedTransactionData:
        """Consolidate outputs.
        """
        prepared = self._call_account_method(
            'prepareConsolidateOutputs', {
                'force': force,
                'outputConsolidationThreshold': output_consolidation_threshold
            }
        )
        return PreparedTransactionData(self, prepared)

    def prepare_create_alias_output(self,
                                    params,
                                    options: Optional[TransactionOptions] = None) -> PreparedTransactionData:
        """Create an alias output.
        """
        prepared = self._call_account_method(
            'prepareCreateAliasOutput', {
                'params': params,
                'options': options
            }
        )
        return PreparedTransactionData(self, prepared)

    def prepare_destroy_alias(self,
                              alias_id: HexStr,
                              options: Optional[TransactionOptions] = None) -> PreparedTransactionData:
        """Destroy an alias output.
        """
        prepared = self._call_account_method(
            'prepareBurn', {
                'burn': Burn().add_alias(alias_id).as_dict(),
                'options': options
            },
        )
        return PreparedTransactionData(self, prepared)

    def prepare_destroy_foundry(self,
                                foundry_id: HexStr,
                                options: Optional[TransactionOptions] = None) -> PreparedTransactionData:
        """Destroy a foundry output with a circulating supply of 0.
        """
        prepared = self._call_account_method(
            'prepareBurn', {
                'burn': Burn().add_foundry(foundry_id).as_dict(),
                'options': options
            },
        )
        return PreparedTransactionData(self, prepared)

    def generate_ed25519_addresses(self, amount: int, options=None):
        """Generate new addresses.
        """
        return self._call_account_method(
            'generateEd25519Addresses', {
                'amount': amount,
                'options': options
            }
        )

    def claimable_outputs(self, outputs_to_claim: List[OutputId]):
        """Get outputs with additional unlock conditions.
        """
        return self._call_account_method(
            'claimableOutputs', {
                'outputsToClaim': outputs_to_claim
            }
        )

    def get_output(self, output_id: OutputId) -> OutputData:
        """Get output.
        """
        return from_dict(OutputData, self._call_account_method(
            'getOutput', {
                'outputId': output_id
            }
        ))

    def get_transaction(self, transaction_id: HexStr) -> Transaction:
        """Get transaction.
        """
        return Transaction.from_dict(self._call_account_method(
            'getTransaction', {
                'transactionId': transaction_id
            }
        ))

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

    def outputs(self, filter_options=None) -> List[OutputData]:
        """Returns all outputs of the account.
        """
        outputs = self._call_account_method(
            'outputs', {
                'filterOptions': filter_options
            }
        )
        return [from_dict(OutputData, o) for o in outputs]

    def unspent_outputs(self, filter_options=None) -> List[OutputData]:
        """Returns all unspent outputs of the account.
        """
        outputs = self._call_account_method(
            'unspentOutputs', {
                'filterOptions': filter_options
            }
        )
        return [from_dict(OutputData, o) for o in outputs]

    def incoming_transactions(self) -> List[Transaction]:
        """Returns all incoming transactions of the account.
        """
        transactions = self._call_account_method(
            'incomingTransactions'
        )
        return [Transaction.from_dict(tx) for tx in transactions]

    def transactions(self) -> List[Transaction]:
        """Returns all transaction of the account.
        """
        transactions = self._call_account_method(
            'transactions'
        )
        return [Transaction.from_dict(tx) for tx in transactions]

    def pending_transactions(self):
        """Returns all pending transactions of the account.
        """
        transactions = self._call_account_method(
            'pendingTransactions'
        )
        return [Transaction.from_dict(tx) for tx in transactions]

    def prepare_create_native_token(self, params, options: Optional[TransactionOptions] = None) -> PreparedTransactionData:
        """Create native token.
        """
        prepared = self._call_account_method(
            'prepareCreateNativeToken', {
                'params': params,
                'options': options
            }
        )
        return PreparedCreateTokenTransaction(account=self, prepared_transaction_data=prepared)

    def prepare_melt_native_token(self,
                                  token_id: HexStr,
                                  melt_amount: int,
                                  options: Optional[TransactionOptions] = None) -> PreparedTransactionData:
        """Melt native tokens. This happens with the foundry output which minted them, by increasing it's
        `melted_tokens` field.
        """
        prepared = self._call_account_method(
            'prepareMeltNativeToken', {
                'tokenId': token_id,
                'meltAmount': hex(melt_amount),
                'options': options
            }
        )
        return PreparedTransactionData(self, prepared)

    def prepare_mint_native_token(self, token_id: HexStr, mint_amount: int, options: Optional[TransactionOptions] = None) -> PreparedTransactionData:
        """Mint additional native tokens.
        """
        prepared = self._call_account_method(
            'prepareMintNativeToken', {
                'tokenId': token_id,
                'mintAmount': hex(mint_amount),
                'options': options
            }
        )
        return PreparedTransactionData(self, prepared)

    def minimum_required_storage_deposit(self, output) -> int:
        """Minimum required storage deposit.
        """
        return int(self._call_account_method(
            'minimumRequiredStorageDeposit', {
                'output': output
            }
        ))

    def prepare_mint_nfts(self, params, options: Optional[TransactionOptions] = None) -> PreparedTransactionData:
        """Mint nfts.
        """
        prepared = self._call_account_method(
            'prepareMintNfts', {
                'params': params,
                'options': options
            }
        )
        return PreparedTransactionData(self, prepared)

    def get_balance(self) -> Balance:
        """Get account balance information.
        """
        return from_dict(Balance, self._call_account_method(
            'getBalance'
        ))

    def prepare_output(self, output_options, transaction_options: Optional[TransactionOptions] = None):
        """Prepare an output for sending
           If the amount is below the minimum required storage deposit, by default the remaining amount will automatically
           be added with a StorageDepositReturn UnlockCondition, when setting the ReturnStrategy to `gift`, the full
           minimum required storage deposit will be sent to the recipient.
           When the assets contain an nft_id, the data from the existing nft output will be used, just with the address
           unlock conditions replaced
        """
        return self._call_account_method(
            'prepareOutput', {
                'params': output_options,
                'transactionOptions': transaction_options
            }
        )

    def prepare_send(self, params, options: Optional[TransactionOptions] = None) -> PreparedTransactionData:
        """Prepare to send base coins.
        """
        prepared = self._call_account_method(
            'prepareSend', {
                'params': params,
                'options': options
            }
        )
        return PreparedTransactionData(self, prepared)

    def prepare_transaction(self, outputs, options: Optional[TransactionOptions] = None) -> PreparedTransactionData:
        """Prepare transaction.
        """
        prepared = self._call_account_method(
            'prepareTransaction', {
                'outputs': outputs,
                'options': options
            }
        )
        return PreparedTransactionData(self, prepared)

    def retry_transaction_until_included(self, transaction_id: HexStr, interval=None, max_attempts=None) -> HexStr:
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

    def sync(self, options: Optional[SyncOptions] = None) -> Balance:
        """Sync the account by fetching new information from the nodes.
           Will also retry pending transactions and consolidate outputs if necessary.
           A custom default can be set using set_default_sync_options
        """
        return from_dict(Balance, self._call_account_method(
            'sync', {
                'options': options,
            }
        ))

    def send(self, params, options: Optional[TransactionOptions] = None) -> Transaction:
        """Send base coins.
        """
        return Transaction.from_dict(self._call_account_method(
            'send', {
                'params': params,
                'options': options
            }
        ))

    def prepare_send_native_tokens(self, params, options: Optional[TransactionOptions] = None) -> PreparedTransactionData:
        """Send native tokens.
        """
        prepared = self._call_account_method(
            'prepareSendNativeTokens', {
                'params': params,
                'options': options
            }
        )
        return PreparedTransactionData(self, prepared)

    def prepare_send_nft(self, params, options: Optional[TransactionOptions] = None) -> PreparedTransactionData:
        """Send nft.
        """
        prepared = self._call_account_method(
            'prepareSendNft', {
                'params': params,
                'options': options
            }
        )
        return PreparedTransactionData(self, prepared)

    def set_alias(self, alias: str):
        """Set alias.
        """
        return self._call_account_method(
            'setAlias', {
                'alias': alias
            }
        )

    def set_default_sync_options(self, options: SyncOptions):
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

    def sign_and_submit_transaction(self, prepared_transaction_data) -> Transaction:
        """Validate the transaction, sign it, submit it to a node and store it in the account.
        """
        return Transaction.from_dict(self._call_account_method(
            'signAndSubmitTransaction', {
                'preparedTransactionData': prepared_transaction_data
            }
        ))

    def submit_and_store_transaction(self, signed_transaction_data) -> Transaction:
        """Submit and store transaction.
        """
        return Transaction.from_dict(self._call_account_method(
            'submitAndStoreTransaction', {
                'signedTransactionData': signed_transaction_data
            }
        ))

    def claim_outputs(self, output_ids_to_claim: List[OutputId]) -> Transaction:
        """Claim outputs.
        """
        return Transaction.from_dict(self._call_account_method(
            'claimOutputs', {
                'outputIdsToClaim': output_ids_to_claim
            }
        ))

    def send_outputs(self, outputs, options: Optional[TransactionOptions] = None) -> Transaction:
        """Send outputs in a transaction.
        """
        return Transaction.from_dict(self._call_account_method(
            'sendOutputs', {
                'outputs': outputs,
                'options': options,
            }
        ))
