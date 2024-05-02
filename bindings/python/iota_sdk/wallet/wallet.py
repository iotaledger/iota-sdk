# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from json import dumps
from typing import List, Optional, Union
from dataclasses import dataclass
from iota_sdk import destroy_wallet, create_wallet, listen_wallet, get_client_from_wallet, get_secret_manager_from_wallet, Client
from iota_sdk.secret_manager.secret_manager import LedgerNanoSecretManager, MnemonicSecretManager, StrongholdSecretManager, SeedSecretManager, SecretManager
from iota_sdk.wallet.common import _call_wallet_method_routine
from iota_sdk.wallet.prepared_transaction import PreparedCreateDelegationTransaction, PreparedCreateDelegationTransactionData, PreparedCreateTokenTransactionData, PreparedTransaction, PreparedCreateTokenTransaction
from iota_sdk.wallet.sync_options import SyncOptions
from iota_sdk.types.balance import Balance
from iota_sdk.types.block.id import BlockId
from iota_sdk.types.burn import Burn
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.client_options import ClientOptions
from iota_sdk.types.filter_options import FilterOptions
from iota_sdk.types.native_token import NativeToken
from iota_sdk.types.output import BasicOutput, NftOutput, Output, deserialize_output
from iota_sdk.types.output_data import OutputData
from iota_sdk.types.output_id import OutputId
from iota_sdk.types.output_params import OutputParams
from iota_sdk.types.transaction_data import PreparedTransactionData, SignedTransactionData
from iota_sdk.types.transaction_id import TransactionId
from iota_sdk.types.send_params import BeginStakingParams, CreateAccountOutputParams, CreateDelegationParams, CreateNativeTokenParams, MintNftParams, SendManaParams, SendNativeTokenParams, SendNftParams, SendParams
from iota_sdk.types.signature import Bip44
from iota_sdk.types.transaction_with_metadata import CreateDelegationTransaction, CreateNativeTokenTransaction, TransactionWithMetadata
from iota_sdk.types.transaction_options import TransactionOptions
from iota_sdk.types.consolidation_params import ConsolidationParams


@json
@dataclass
class WalletOptions:
    """Options for the Wallet builder."""
    address: Optional[str] = None
    alias: Optional[str] = None
    bip_path: Optional[Bip44] = None
    client_options: Optional[ClientOptions] = None
    secret_manager: Optional[Union[LedgerNanoSecretManager,
                                   MnemonicSecretManager, SeedSecretManager, StrongholdSecretManager]] = None
    storage_path: Optional[str] = None


# pylint: disable=too-many-public-methods


class Wallet:
    """An IOTA Wallet.

    Attributes:
        handle: The wallet handle.
    """

    def __init__(self, options: WalletOptions):
        """Initialize `self`.
        """
        # Create the message handler
        self.handle = create_wallet(dumps(options.to_dict()))

    def get_handle(self):
        """Return the wallet handle.
        """
        return self.handle

    @_call_wallet_method_routine
    def _call_method(self, name: str, data=None):
        message = {
            'name': name
        }
        if data:
            message['data'] = data
        return message

    def backup_to_stronghold_snapshot(self, destination: str, password: str):
        """Backup storage.
        """
        return self._call_method(
            'backup', {
                'destination': destination,
                'password': password
            }
        )

    def change_stronghold_password(self, password: str):
        """Change stronghold password.
        """
        return self._call_method(
            'changeStrongholdPassword', {
                'currentPassword': password,
                'newPassword': password
            }
        )

    def clear_stronghold_password(self):
        """Clear stronghold password.
        """
        return self._call_method(
            'clearStrongholdPassword'
        )

    def is_stronghold_password_available(self) -> bool:
        """Return whether a Stronghold password is available.
        """
        return self._call_method(
            'isStrongholdPasswordAvailable'
        )

    def destroy(self):
        """Destroys the wallet instance.
        """
        return destroy_wallet(self.handle)

    def emit_test_event(self, event) -> bool:
        """Helper function to test events.
        """
        return self._call_method(
            'emitTestEvent', {
                'event': event,
            },
        )

    def get_client(self):
        """Get the client associated with the wallet.
        """
        return Client(client_handle=get_client_from_wallet(self.handle))

    def get_secret_manager(self):
        """Get the secret manager associated with the wallet.
        """
        return SecretManager(
            secret_manager_handle=get_secret_manager_from_wallet(self.handle))

    def listen(self, handler, events: Optional[List[int]] = None):
        """Listen to wallet events, empty array or None will listen to all events.
        The default value for events is None.
        """
        events_array = [] if events is None else events
        listen_wallet(self.handle, events_array, handler)

    def clear_listeners(self, events: Optional[List[int]] = None):
        """Remove wallet event listeners, empty array or None will remove all listeners.
        The default value for events is None.
        """
        events_array = [] if events is None else events
        return self._call_method(
            'clearListeners', {
                'eventTypes': events_array
            }
        )

    def restore_from_stronghold_snapshot(self, source: str, password: str):
        """Restore a backup from a Stronghold file.
        Replaces `client_options`, `coin_type`, `secret_manager` and wallet.
        Returns an error if the wallet was already created. If Stronghold is used
        as the secret_manager, the existing Stronghold file will be overwritten.
        Be aware that if a mnemonic was stored, it will be lost.
        """
        return self._call_method(
            'restoreBackup', {
                'source': source,
                'password': password
            }
        )

    def set_client_options(self, client_options):
        """Update the options of the wallet client.
        """
        return self._call_method(
            'setClientOptions',
            {
                'clientOptions': client_options.to_dict()
            }
        )

    def set_stronghold_password(self, password: str):
        """Set stronghold password.
        """
        return self._call_method(
            'setStrongholdPassword', {
                'password': password
            }

        )

    def set_stronghold_password_clear_interval(
            self, interval_in_milliseconds: int):
        """Set stronghold password clear interval.
        """
        return self._call_method(
            'setStrongholdPasswordClearInterval', {
                'intervalInMilliseconds': interval_in_milliseconds
            }
        )

    def start_background_sync(
            self, options: Optional[SyncOptions] = None, interval_in_milliseconds: Optional[int] = None):
        """Start background syncing.
        """
        return self._call_method(
            'startBackgroundSync', {
                'options': options,
                'intervalInMilliseconds': interval_in_milliseconds
            }
        )

    def stop_background_sync(self):
        """Stop background syncing.
        """
        return self._call_method(
            'stopBackgroundSync',
        )

    def store_mnemonic(self, mnemonic: str):
        """Store mnemonic.
        """
        return self._call_method(
            'storeMnemonic', {
                'mnemonic': mnemonic
            }
        )

    def update_node_auth(self, url: str, auth=None):
        """Update the authentication for the provided node.
        """
        return self._call_method(
            'updateNodeAuth', {
                'url': url,
                'auth': auth
            }
        )

    def accounts(self) -> List[OutputData]:
        """Returns the accounts of the wallet.
        """
        outputs = self._call_method(
            'accounts'
        )
        return [OutputData.from_dict(o) for o in outputs]

    def burn(
            self, burn: Burn, options: Optional[TransactionOptions] = None) -> TransactionWithMetadata:
        """A generic function that can be used to burn native tokens, nfts, foundries and aliases.
        """
        return self.prepare_burn(burn, options).send()

    def prepare_burn(
            self, burn: Burn, options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """A generic `prepare_burn()` function that can be used to prepare the burn of native tokens, nfts, foundries and accounts.
        """
        prepared = PreparedTransactionData.from_dict(self._call_method(
            'prepareBurn', {
                'burn': burn.to_dict(),
                'options': options
            },
        ))
        return PreparedTransaction(self, prepared)

    def prepare_burn_native_token(self,
                                  token_id: HexStr,
                                  burn_amount: int,
                                  options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Burn native tokens. This doesn't require the foundry output which minted them, but will not increase
        the foundries `melted_tokens` field, which makes it impossible to destroy the foundry output. Therefore it's
        recommended to use melting, if the foundry output is available.
        """
        prepared = PreparedTransactionData.from_dict(self._call_method(
            'prepareBurn', {
                'burn': Burn().add_native_token(NativeToken(token_id, hex(burn_amount))).to_dict(),
                'options': options
            },
        ))
        return PreparedTransaction(self, prepared)

    def prepare_burn_nft(self,
                         nft_id: HexStr,
                         options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Burn an nft output.
        """
        prepared = PreparedTransactionData.from_dict(self._call_method(
            'prepareBurn', {
                'burn': Burn().add_nft(nft_id).to_dict(),
                'options': options
            },
        ))
        return PreparedTransaction(self, prepared)

    def claim_outputs(
            self, output_ids_to_claim: List[OutputId], options: Optional[TransactionOptions] = None) -> TransactionWithMetadata:
        """Claim outputs.
        """
        return self.prepare_claim_outputs(output_ids_to_claim, options).send()

    def prepare_claim_outputs(
            self, output_ids_to_claim: List[OutputId], options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Claim outputs.
        """
        return PreparedTransaction(self, PreparedTransactionData.from_dict(self._call_method(
            'prepareClaimOutputs', {
                'outputIdsToClaim': output_ids_to_claim,
                'options': options
            }
        )))

    def consolidate_outputs(
            self, params: ConsolidationParams) -> TransactionWithMetadata:
        """Consolidate outputs.
        """
        return self.prepare_consolidate_outputs(params).send()

    def prepare_consolidate_outputs(
            self, params: ConsolidationParams) -> PreparedTransaction:
        """Consolidate outputs.
        """
        prepared = PreparedTransactionData.from_dict(self._call_method(
            'prepareConsolidateOutputs', {
                'params': params
            }
        ))
        return PreparedTransaction(self, prepared)

    def create_account_output(self,
                              params: Optional[CreateAccountOutputParams] = None,
                              options: Optional[TransactionOptions] = None) -> TransactionWithMetadata:
        """Create an account output.
        """
        return self.prepare_create_account_output(params, options).send()

    def prepare_create_account_output(self,
                                      params: Optional[CreateAccountOutputParams] = None,
                                      options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Create an account output.
        """
        prepared = PreparedTransactionData.from_dict(self._call_method(
            'prepareCreateAccountOutput', {
                'params': params,
                'options': options
            }
        ))
        return PreparedTransaction(self, prepared)

    def melt_native_token(self,
                          token_id: HexStr,
                          melt_amount: int,
                          options: Optional[TransactionOptions] = None) -> TransactionWithMetadata:
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
        prepared = PreparedTransactionData.from_dict(self._call_method(
            'prepareMeltNativeToken', {
                'tokenId': token_id,
                'meltAmount': hex(melt_amount),
                'options': options
            }
        ))
        return PreparedTransaction(self, prepared)

    def prepare_destroy_account(self,
                                account_id: HexStr,
                                options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Destroy an account output.
        """
        prepared = PreparedTransactionData.from_dict(self._call_method(
            'prepareBurn', {
                'burn': Burn().add_account(account_id).to_dict(),
                'options': options
            },
        ))
        return PreparedTransaction(self, prepared)

    def prepare_destroy_foundry(self,
                                foundry_id: HexStr,
                                options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Destroy a foundry output with a circulating supply of 0.
        """
        prepared = PreparedTransactionData.from_dict(self._call_method(
            'prepareBurn', {
                'burn': Burn().add_foundry(foundry_id).to_dict(),
                'options': options
            },
        ))
        return PreparedTransaction(self, prepared)

    def get_balance(self) -> Balance:
        """Get wallet balance information.
        """
        return Balance.from_dict(self._call_method(
            'getBalance'
        ))

    def get_output(self, output_id: OutputId) -> OutputData:
        """Get output.
        """
        return OutputData.from_dict(self._call_method(
            'getOutput', {
                'outputId': output_id
            }
        ))

    def get_foundry_output(self, token_id: HexStr):
        """Get a `FoundryOutput` by native token ID. It will try to get the foundry from the wallet, if it isn't in the wallet it will try to get it from the node.
        """
        return self._call_method(
            'getFoundryOutput', {
                'tokenId': token_id
            }
        )

    def claimable_outputs(self, outputs_to_claim: List[OutputId]):
        """Get outputs with additional unlock conditions.
        """
        return self._call_method(
            'claimableOutputs', {
                'outputsToClaim': outputs_to_claim
            }
        )

    def get_transaction(
            self, transaction_id: TransactionId) -> TransactionWithMetadata:
        """Get transaction.
        """
        return TransactionWithMetadata.from_dict(self._call_method(
            'getTransaction', {
                'transactionId': transaction_id
            }
        ))

    def address(self) -> str:
        """Get the address of the wallet.
        """
        return self._call_method(
            'getAddress'
        )

    def outputs(
            self, filter_options: Optional[FilterOptions] = None) -> List[OutputData]:
        """Returns all outputs of the wallet.
        """
        outputs = self._call_method(
            'outputs', {
                'filterOptions': filter_options
            }
        )
        return [OutputData.from_dict(o) for o in outputs]

    def pending_transactions(self):
        """Returns all pending transactions of the wallet.
        """
        transactions = self._call_method(
            'pendingTransactions'
        )
        return [TransactionWithMetadata.from_dict(tx) for tx in transactions]

    def implicit_account_creation_address(self) -> str:
        """Returns the implicit account creation address of the wallet if it is Ed25519 based.
        """
        return self._call_method(
            'implicitAccountCreationAddress'
        )

    def implicit_account_transition(
            self, output_id: OutputId) -> TransactionWithMetadata:
        """Transitions an implicit account to an account.
        """
        return self.prepare_implicit_account_transition(output_id).send()

    def prepare_implicit_account_transition(
            self, output_id: OutputId) -> PreparedTransaction:
        """Prepares to transition an implicit account to an account.
        """
        prepared = PreparedTransactionData.from_dict(self._call_method(
            'prepareImplicitAccountTransition', {
                'outputId': output_id
            }
        ))
        return PreparedTransaction(self, prepared)

    def implicit_accounts(self) -> List[OutputData]:
        """Returns the implicit accounts of the wallet.
        """
        outputs = self._call_method(
            'implicitAccounts'
        )
        return [OutputData.from_dict(o) for o in outputs]

    def incoming_transactions(self) -> List[TransactionWithMetadata]:
        """Returns all incoming transactions of the wallet.
        """
        transactions = self._call_method(
            'incomingTransactions'
        )
        return [TransactionWithMetadata.from_dict(tx) for tx in transactions]

    def transactions(self) -> List[TransactionWithMetadata]:
        """Returns all transaction of the wallet.
        """
        transactions = self._call_method(
            'transactions'
        )
        return [TransactionWithMetadata.from_dict(tx) for tx in transactions]

    def unspent_outputs(
            self, filter_options: Optional[FilterOptions] = None) -> List[OutputData]:
        """Returns all unspent outputs of the wallet.
        """
        outputs = self._call_method(
            'unspentOutputs', {
                'filterOptions': filter_options
            }
        )
        return [OutputData.from_dict(o) for o in outputs]

    def mint_native_token(self, token_id: HexStr, mint_amount: int,
                          options: Optional[TransactionOptions] = None) -> TransactionWithMetadata:
        """Mint additional native tokens.
        """
        return self.prepare_mint_native_token(
            token_id, mint_amount, options).send()

    def prepare_mint_native_token(self, token_id: HexStr, mint_amount: int,
                                  options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Mint additional native tokens.
        """
        prepared = PreparedTransactionData.from_dict(self._call_method(
            'prepareMintNativeToken', {
                'tokenId': token_id,
                'mintAmount': hex(mint_amount),
                'options': options
            }
        ))
        return PreparedTransaction(self, prepared)

    def create_native_token(self, params: CreateNativeTokenParams,
                            options: Optional[TransactionOptions] = None) -> CreateNativeTokenTransaction:
        """Create native token.
        """
        return self.prepare_create_native_token(params, options).send()

    def prepare_create_native_token(self, params: CreateNativeTokenParams,
                                    options: Optional[TransactionOptions] = None) -> PreparedCreateTokenTransaction:
        """Create native token.
        """
        prepared = PreparedCreateTokenTransactionData.from_dict(self._call_method(
            'prepareCreateNativeToken', {
                'params': params,
                'options': options
            }
        ))
        return PreparedCreateTokenTransaction(self, prepared)

    def mint_nfts(self, params: List[MintNftParams],
                  options: Optional[TransactionOptions] = None) -> TransactionWithMetadata:
        """Mint NFTs.
        """
        return self.prepare_mint_nfts(params, options).send()

    def prepare_mint_nfts(self, params: List[MintNftParams],
                          options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Mint NFTs.
        """
        prepared = PreparedTransactionData.from_dict(self._call_method(
            'prepareMintNfts', {
                'params': params,
                'options': options
            }
        ))
        return PreparedTransaction(self, prepared)

    def prepare_output(self, params: OutputParams,
                       transaction_options: Optional[TransactionOptions] = None) -> Union[BasicOutput, NftOutput]:
        """Prepare an output for sending.
           If the amount is below the minimum required storage deposit, by default the remaining amount will automatically
           be added with a StorageDepositReturn UnlockCondition, when setting the ReturnStrategy to `gift`, the full
           minimum required storage deposit will be sent to the recipient.
           When the assets contain an nft_id, the data from the existing nft output will be used, just with the address
           unlock conditions replaced
        """
        return deserialize_output(self._call_method(
            'prepareOutput', {
                'params': params,
                'transactionOptions': transaction_options
            })
        )

    def create_delegation(self, params: CreateDelegationParams,
                          options: Optional[TransactionOptions] = None) -> CreateDelegationTransaction:
        """Create a delegation.
        """
        return self.prepare_create_delegation(params, options).send()

    def prepare_create_delegation(self, params: CreateDelegationParams,
                                  options: Optional[TransactionOptions] = None) -> PreparedCreateDelegationTransaction:
        """Prepare to create a delegation.
        """
        prepared = PreparedCreateDelegationTransactionData.from_dict(self._call_method(
            'prepareCreateDelegation', {
                'params': params,
                'options': options
            }
        ))
        return PreparedCreateDelegationTransaction(self, prepared)

    def delay_delegation_claiming(
            self, delegation_id: HexStr, reclaim_excess: bool) -> TransactionWithMetadata:
        """Delay a delegation's claiming.
        """
        return self.prepare_delay_delegation_claiming(
            delegation_id, reclaim_excess).send()

    def prepare_delay_delegation_claiming(
            self, delegation_id: HexStr, reclaim_excess: bool) -> PreparedTransaction:
        """Prepare to delay a delegation's claiming.
        """
        prepared = PreparedTransactionData.from_dict(self._call_method(
            'prepareDelayDelegationClaiming', {
                'delegationId': delegation_id,
                'reclaimExcess': reclaim_excess,
            }
        ))
        return PreparedTransaction(self, prepared)

    def begin_staking(self, params: BeginStakingParams,
                      options: Optional[TransactionOptions] = None) -> TransactionWithMetadata:
        """Begin staking.
        """
        return self.prepare_begin_staking(params, options).send()

    def prepare_begin_staking(self, params: BeginStakingParams,
                              options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Prepare to begin staking.
        """
        prepared = PreparedTransactionData.from_dict(self._call_method(
            'prepareBeginStaking', {
                'params': params,
                'options': options
            }
        ))
        return PreparedTransaction(self, prepared)

    def extend_staking(self, account_id: HexStr,
                       additional_epochs: int) -> TransactionWithMetadata:
        """Extend staking by additional epochs.
        """
        return self.prepare_extend_staking(
            account_id, additional_epochs).send()

    def prepare_extend_staking(self, account_id: HexStr,
                               additional_epochs: int) -> PreparedTransaction:
        """Prepare to extend staking by additional epochs.
        """
        prepared = PreparedTransactionData.from_dict(self._call_method(
            'prepareExtendStaking', {
                'accountId': account_id,
                'additionalEpochs': additional_epochs
            }
        ))
        return PreparedTransaction(self, prepared)

    def end_staking(self, account_id: HexStr) -> TransactionWithMetadata:
        """End staking and claim rewards.
        """
        return self.prepare_end_staking(account_id).send()

    def prepare_end_staking(self, account_id: HexStr) -> PreparedTransaction:
        """Prepare to end staking and claim rewards.
        """
        prepared = PreparedTransactionData.from_dict(self._call_method(
            'prepareEndStaking', {
                'accountId': account_id,
            }
        ))
        return PreparedTransaction(self, prepared)

    def announce_candidacy(self, account_id: HexStr) -> BlockId:
        """Announce a staking account's candidacy for the staking period.
        """
        return BlockId(self._call_method(
            'announceCandidacy', {
                'accountId': account_id,
            }
        ))

    def send_outputs(
            self, outputs: List[Output], options: Optional[TransactionOptions] = None) -> TransactionWithMetadata:
        """Send outputs.
        """
        return self.prepare_send_outputs(outputs, options).send()

    def prepare_send_outputs(
            self, outputs: List[Output], options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Prepare to send outputs.
        """
        prepared = PreparedTransactionData.from_dict(self._call_method(
            'prepareSendOutputs', {
                'outputs': outputs,
                'options': options
            }
        ))
        return PreparedTransaction(self, prepared)

    def wait_for_transaction_acceptance(
            self, transaction_id: TransactionId, interval=None, max_attempts=None):
        """Checks the transaction state for a provided transaction id until it's accepted. Interval in milliseconds.
        """
        return self._call_method(
            'waitForTransactionAcceptance', {
                'transactionId': transaction_id,
                'interval': interval,
                'maxAttempts': max_attempts
            }
        )

    def send(self, amount: int, address: str,
             options: Optional[TransactionOptions] = None) -> TransactionWithMetadata:
        """Send base coins.
        """
        return self.prepare_send([SendParams(address, amount)], options).send()

    def send_with_params(
            self, params: List[SendParams], options: Optional[TransactionOptions] = None) -> TransactionWithMetadata:
        """Send base coins to multiple addresses or with additional parameters.
        """
        return self.prepare_send(params, options).send()

    def prepare_send(self, params: List[SendParams],
                     options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Prepare to send with params.
        """
        prepared = PreparedTransactionData.from_dict(self._call_method(
            'prepareSend', {
                'params': params,
                'options': options
            }
        ))
        return PreparedTransaction(self, prepared)

    def send_native_tokens(
            self, params: List[SendNativeTokenParams], options: Optional[TransactionOptions] = None) -> TransactionWithMetadata:
        """Send native tokens.
        """
        return self.prepare_send_native_tokens(params, options).send()

    def prepare_send_native_tokens(
            self,
            params: List[SendNativeTokenParams],
            options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Send native tokens.
        """
        prepared = PreparedTransactionData.from_dict(self._call_method(
            'prepareSendNativeTokens', {
                'params': params,
                'options': options
            }
        ))
        return PreparedTransaction(self, prepared)

    def send_nft(self, params: List[SendNftParams],
                 options: Optional[TransactionOptions] = None) -> TransactionWithMetadata:
        """Send nft.
        """
        return self.prepare_send_nft(params, options).send()

    def prepare_send_nft(self, params: List[SendNftParams],
                         options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Send nft.
        """
        prepared = PreparedTransactionData.from_dict(self._call_method(
            'prepareSendNft', {
                'params': params,
                'options': options
            }
        ))
        return PreparedTransaction(self, prepared)

    def send_mana(
            self, params: SendManaParams, options: Optional[TransactionOptions] = None) -> TransactionWithMetadata:
        """Send mana.
        """
        return self.prepare_send_mana(params, options).send()

    def prepare_send_mana(self, params: SendManaParams,
                          options: Optional[TransactionOptions] = None) -> PreparedTransaction:
        """Prepare to send mana.
        """
        prepared = PreparedTransactionData.from_dict(self._call_method(
            'prepareSendMana', {
                'params': params,
                'options': options
            }
        ))
        return PreparedTransaction(self, prepared)

    def set_alias(self, alias: str):
        """Set alias.
        """
        return self._call_method(
            'setAlias', {
                'alias': alias
            }
        )

    def set_default_sync_options(self, options: SyncOptions):
        """Set the fallback SyncOptions for wallet syncing.
        If storage is enabled, will persist during restarts.
        """
        return self._call_method(
            'setDefaultSyncOptions', {
                'options': options
            }
        )

    def sign_transaction(
            self, prepared_transaction_data: PreparedTransactionData) -> SignedTransactionData:
        """Sign a transaction.
        """
        return SignedTransactionData.from_dict(self._call_method(
            'signTransaction', {
                'preparedTransactionData': prepared_transaction_data
            }
        ))

    def sign_and_submit_transaction(
            self, prepared_transaction_data: PreparedTransactionData) -> TransactionWithMetadata:
        """Validate the transaction, sign it, submit it to a node and store it in the wallet.
        """
        return TransactionWithMetadata.from_dict(self._call_method(
            'signAndSubmitTransaction', {
                'preparedTransactionData': prepared_transaction_data
            }
        ))

    def submit_and_store_transaction(
            self, signed_transaction_data: SignedTransactionData) -> TransactionWithMetadata:
        """Submit and store transaction.
        """
        return TransactionWithMetadata.from_dict(self._call_method(
            'submitAndStoreTransaction', {
                'signedTransactionData': signed_transaction_data
            }
        ))

    def sync(self, options: Optional[SyncOptions] = None) -> Balance:
        """Sync the wallet by fetching new information from the nodes.
        Will also reissue pending transactions and consolidate outputs if necessary.
        A custom default can be set using set_default_sync_options.
        """
        return Balance.from_dict(self._call_method(
            'sync', {
                'options': options,
            }
        ))
