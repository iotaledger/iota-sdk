# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from typing import List, Optional, Union
from dataclasses import dataclass
from dacite import from_dict
from iota_sdk.wallet.common import _call_method_routine
from iota_sdk.wallet.prepared_transaction import PreparedTransaction, PreparedCreateTokenTransaction
from iota_sdk.wallet.sync_options import SyncOptions
from iota_sdk.types.address import AccountAddress, AddressWithUnspentOutputs
from iota_sdk.types.balance import Balance
from iota_sdk.types.burn import Burn
from iota_sdk.types.common import HexStr
from iota_sdk.types.filter_options import FilterOptions
from iota_sdk.types.native_token import NativeToken
from iota_sdk.types.output_data import OutputData
from iota_sdk.types.output_id import OutputId
from iota_sdk.types.output import BasicOutput, NftOutput, Output, output_from_dict
from iota_sdk.types.output_params import OutputParams
from iota_sdk.types.transaction_data import PreparedTransactionData, SignedTransactionData
from iota_sdk.types.send_params import CreateAliasOutputParams, CreateNativeTokenParams, MintNftParams, SendNativeTokensParams, SendNftParams, SendParams
from iota_sdk.types.transaction import Transaction
from iota_sdk.types.transaction_options import TransactionOptions
from iota_sdk.types.consolidation_params import ConsolidationParams


@dataclass
class AccountMetadata:
    """Account metadata.

    Attributes:
        alias: The alias name of the account.
        coinType: The type of coin managed with the account.
        index: The account index.
    """
    alias: str
    coinType: int
    index: int


# pylint: disable=too-many-public-methods
class Account:
    """A wallet account.

    Attributes:
        meta: Some account metadata.
        handle: The account handle.
    """

    def __init__(self, meta: dict, handle):
        """Initializes an account.

        Args:
            meta: The account data.
            handle: The account handle.
        """
        self.meta = meta
        self.handle = handle

    @_call_method_routine
    def _call_account_method(self, method, data=None):
        message = {
            'name': 'callAccountMethod',
            'data': {
                'accountId': self.meta["index"],
                'method': {
                    'name': method,
                }
            }
        }
        if data:
            message['data']['method']['data'] = data

        return message

    def get_metadata(self) -> AccountMetadata:
        """Get the accounts metadata.
        """
        return AccountMetadata(
            self.meta["alias"], self.meta["coinType"], self.meta["index"])

    def burn(
            self, burn: Burn, options: Optional[TransactionOptions] = None) -> Transaction:
        """A generic function that can be used to burn native tokens, nfts, foundries and aliases.
        """
        return self.prepare_burn(burn, options).send()

    def prepare_burn(
            self, burn: Burn, options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """A generic `prepare_burn()` function that can be used to prepare the burn of native tokens, nfts, foundries and aliases.
        """
        prepared = self._call_account_method(
            'prepareBurn', {
                'burn': burn.as_dict(),
                'options': options
            },
        )
        return PreparedTransaction(self, prepared)

    def prepare_burn_native_token(self,
                                  token_id: HexStr,
                                  burn_amount: int,
                                  options: Optional[TransactionOptions] = None) -> PreparedTransaction:
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
        return PreparedTransaction(self, prepared)

    def prepare_burn_nft(self,
                         nft_id: HexStr,
                         options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Burn an nft output.
        """
        prepared = self._call_account_method(
            'prepareBurn', {
                'burn': Burn().add_nft(nft_id).as_dict(),
                'options': options
            },
        )
        return PreparedTransaction(self, prepared)

    def consolidate_outputs(
            self, params: ConsolidationParams) -> Transaction:
        """Consolidate outputs.
        """
        return self.prepare_consolidate_outputs(params).send()

    def prepare_consolidate_outputs(
            self, params: ConsolidationParams) -> PreparedTransaction:
        """Consolidate outputs.
        """
        prepared = self._call_account_method(
            'prepareConsolidateOutputs', {
                'params': params
            }
        )
        return PreparedTransaction(self, prepared)

    def create_alias_output(self,
                            params: Optional[CreateAliasOutputParams] = None,
                            options: Optional[TransactionOptions] = None) -> Transaction:
        """Create an alias output.
        """
        return self.prepare_create_alias_output(params, options).send()

    def prepare_create_alias_output(self,
                                    params: Optional[CreateAliasOutputParams] = None,
                                    options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Create an alias output.
        """
        prepared = self._call_account_method(
            'prepareCreateAliasOutput', {
                'params': params,
                'options': options
            }
        )
        return PreparedTransaction(self, prepared)

    def prepare_destroy_alias(self,
                              alias_id: HexStr,
                              options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Destroy an alias output.
        """
        prepared = self._call_account_method(
            'prepareBurn', {
                'burn': Burn().add_alias(alias_id).as_dict(),
                'options': options
            },
        )
        return PreparedTransaction(self, prepared)

    def prepare_destroy_foundry(self,
                                foundry_id: HexStr,
                                options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Destroy a foundry output with a circulating supply of 0.
        """
        prepared = self._call_account_method(
            'prepareBurn', {
                'burn': Burn().add_foundry(foundry_id).as_dict(),
                'options': options
            },
        )
        return PreparedTransaction(self, prepared)

    def generate_ed25519_addresses(
            self, amount: int, options=None) -> List[AccountAddress]:
        """Generate new addresses.
        """
        addresses = self._call_account_method(
            'generateEd25519Addresses', {
                'amount': amount,
                'options': options
            }
        )
        return [from_dict(AccountAddress, address) for address in addresses]

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

    def addresses(self) -> List[AccountAddress]:
        """List addresses.
        """
        addresses = self._call_account_method(
            'addresses'
        )
        return [from_dict(AccountAddress, address) for address in addresses]

    def addresses_with_unspent_outputs(
            self) -> List[AddressWithUnspentOutputs]:
        """Returns only addresses of the account with unspent outputs.
        """
        addresses = self._call_account_method(
            'addressesWithUnspentOutputs'
        )
        return [from_dict(AddressWithUnspentOutputs, address)
                for address in addresses]

    def outputs(
            self, filter_options: Optional[FilterOptions] = None) -> List[OutputData]:
        """Returns all outputs of the account.
        """
        outputs = self._call_account_method(
            'outputs', {
                'filterOptions': filter_options
            }
        )
        return [from_dict(OutputData, o) for o in outputs]

    def unspent_outputs(
            self, filter_options: Optional[FilterOptions] = None) -> List[OutputData]:
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

    def create_native_token(self, params: CreateNativeTokenParams,
                            options: Optional[TransactionOptions] = None) -> Transaction:
        """Create native token.
        """
        return self.prepare_create_native_token(params, options).send()

    def prepare_create_native_token(self, params: CreateNativeTokenParams,
                                    options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Create native token.
        """
        prepared = self._call_account_method(
            'prepareCreateNativeToken', {
                'params': params,
                'options': options
            }
        )
        return PreparedCreateTokenTransaction(
            account=self, prepared_transaction_data=prepared)

    def melt_native_token(self,
                          token_id: HexStr,
                          melt_amount: int,
                          options: Optional[TransactionOptions] = None) -> Transaction:
        """Melt native tokens. This happens with the foundry output which minted them, by increasing it's
        `melted_tokens` field.
        """
        return self.prepare_melt_native_token(
            token_id, melt_amount, options).send()

    def prepare_melt_native_token(self,
                                  token_id: HexStr,
                                  melt_amount: int,
                                  options: Optional[TransactionOptions] = None) -> PreparedTransaction:
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
        return PreparedTransaction(self, prepared)

    def mint_native_token(self, token_id: HexStr, mint_amount: int,
                          options: Optional[TransactionOptions] = None) -> Transaction:
        """Mint additional native tokens.
        """
        return self.prepare_mint_native_token(
            token_id, mint_amount, options).send()

    def prepare_mint_native_token(self, token_id: HexStr, mint_amount: int,
                                  options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Mint additional native tokens.
        """
        prepared = self._call_account_method(
            'prepareMintNativeToken', {
                'tokenId': token_id,
                'mintAmount': hex(mint_amount),
                'options': options
            }
        )
        return PreparedTransaction(self, prepared)

    def mint_nfts(self, params: List[MintNftParams],
                  options: Optional[TransactionOptions] = None) -> Transaction:
        """Mint NFTs.
        """
        return self.prepare_mint_nfts(params, options).send()

    def prepare_mint_nfts(self, params: List[MintNftParams],
                          options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Mint NFTs.
        """
        prepared = self._call_account_method(
            'prepareMintNfts', {
                'params': params,
                'options': options
            }
        )
        return PreparedTransaction(self, prepared)

    def get_balance(self) -> Balance:
        """Get account balance information.
        """
        return from_dict(Balance, self._call_account_method(
            'getBalance'
        ))

    def prepare_output(self, params: OutputParams,
                       transaction_options: Optional[TransactionOptions] = None) -> Union[BasicOutput, NftOutput]:
        """Prepare an output for sending.
           If the amount is below the minimum required storage deposit, by default the remaining amount will automatically
           be added with a StorageDepositReturn UnlockCondition, when setting the ReturnStrategy to `gift`, the full
           minimum required storage deposit will be sent to the recipient.
           When the assets contain an nft_id, the data from the existing nft output will be used, just with the address
           unlock conditions replaced
        """
        return output_from_dict(self._call_account_method(
            'prepareOutput', {
                'params': params,
                'transactionOptions': transaction_options
            })
        )

    def prepare_send(self, params: List[SendParams],
                     options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Prepare to send base coins.
        """
        prepared = self._call_account_method(
            'prepareSend', {
                'params': params,
                'options': options
            }
        )
        return PreparedTransaction(self, prepared)

    def send_transaction(
            self, outputs: List[Output], options: Optional[TransactionOptions] = None) -> Transaction:
        """Send a transaction.
        """
        return self.prepare_transaction(outputs, options).send()

    def prepare_transaction(
            self, outputs: List[Output], options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Prepare transaction.
        """
        prepared = self._call_account_method(
            'prepareTransaction', {
                'outputs': outputs,
                'options': options
            }
        )
        return PreparedTransaction(self, prepared)

    def retry_transaction_until_included(
            self, transaction_id: HexStr, interval=None, max_attempts=None) -> HexStr:
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
        A custom default can be set using set_default_sync_options.
        """
        return from_dict(Balance, self._call_account_method(
            'sync', {
                'options': options,
            }
        ))

    def send(self, amount: str, address: str,
             options: Optional[TransactionOptions] = None) -> Transaction:
        """Send base coins.
        """
        return Transaction.from_dict(self._call_account_method(
            'send', {
                'amount': str(amount),
                'address': address,
                'options': options
            }
        ))

    def send_with_params(
            self, params: List[SendParams], options: Optional[TransactionOptions] = None) -> Transaction:
        """Send base coins to multiple addresses or with additional parameters.
        """
        return Transaction.from_dict(self._call_account_method(
            'sendWithParams', {
                'params': params,
                'options': options
            }
        ))

    def send_native_tokens(
            self, params: List[SendNativeTokensParams], options: Optional[TransactionOptions] = None) -> Transaction:
        """Send native tokens.
        """
        return self.prepare_send_native_tokens(params, options).send()

    def prepare_send_native_tokens(
            self, params: List[SendNativeTokensParams], options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Send native tokens.
        """
        prepared = self._call_account_method(
            'prepareSendNativeTokens', {
                'params': params,
                'options': options
            }
        )
        return PreparedTransaction(self, prepared)

    def send_nft(self, params: List[SendNftParams],
                 options: Optional[TransactionOptions] = None) -> Transaction:
        """Send nft.
        """
        return self.prepare_send_nft(params, options).send()

    def prepare_send_nft(self, params: List[SendNftParams],
                         options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Send nft.
        """
        prepared = self._call_account_method(
            'prepareSendNft', {
                'params': params,
                'options': options
            }
        )
        return PreparedTransaction(self, prepared)

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

    def sign_transaction_essence(
            self, prepared_transaction_data: PreparedTransactionData) -> SignedTransactionData:
        """Sign a transaction essence.
        """
        return from_dict(SignedTransactionData, self._call_account_method(
            'signTransactionEssence', {
                'preparedTransactionData': prepared_transaction_data
            }
        ))

    def sign_and_submit_transaction(
            self, prepared_transaction_data: PreparedTransactionData) -> Transaction:
        """Validate the transaction, sign it, submit it to a node and store it in the account.
        """
        return Transaction.from_dict(self._call_account_method(
            'signAndSubmitTransaction', {
                'preparedTransactionData': prepared_transaction_data
            }
        ))

    def submit_and_store_transaction(
            self, signed_transaction_data: SignedTransactionData) -> Transaction:
        """Submit and store transaction.
        """
        return Transaction.from_dict(self._call_account_method(
            'submitAndStoreTransaction', {
                'signedTransactionData': signed_transaction_data
            }
        ))

    def claim_outputs(
            self, output_ids_to_claim: List[OutputId]) -> Transaction:
        """Claim outputs.
        """
        return self.prepare_claim_outputs(output_ids_to_claim).send()

    def prepare_claim_outputs(
            self, output_ids_to_claim: List[OutputId]) -> PreparedTransaction:
        """Claim outputs.
        """
        return PreparedTransaction(self, self._call_account_method(
            'prepareClaimOutputs', {
                'outputIdsToClaim': output_ids_to_claim
            }
        ))

    def send_outputs(
            self, outputs: List[Output], options: Optional[TransactionOptions] = None) -> Transaction:
        """Send outputs in a transaction.
        """
        return Transaction.from_dict(self._call_account_method(
            'sendOutputs', {
                'outputs': outputs,
                'options': options,
            }
        ))
