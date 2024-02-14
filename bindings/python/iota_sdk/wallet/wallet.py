# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from json import dumps
from typing import Any, Dict, List, Optional, Union
from iota_sdk import destroy_wallet, create_wallet, listen_wallet, get_client_from_wallet, get_secret_manager_from_wallet, Client
from iota_sdk.secret_manager.secret_manager import LedgerNanoSecretManager, MnemonicSecretManager, StrongholdSecretManager, SeedSecretManager, SecretManager
from iota_sdk.types.address import AccountAddress
from iota_sdk.types.client_options import ClientOptions
from iota_sdk.wallet.account import Account, _call_method_routine
from iota_sdk.wallet.sync_options import SyncOptions

# pylint: disable=too-many-public-methods


class Wallet():
    """An IOTA Wallet.

    Attributes:
        handle: The wallet handle.
    """

    def __init__(self,
                 storage_path: Optional[str] = None,
                 client_options: Optional[Union[Dict[str,
                                                     Any], ClientOptions]] = None,
                 coin_type: Optional[int] = None,
                 secret_manager: Optional[Union[LedgerNanoSecretManager, MnemonicSecretManager, SeedSecretManager, StrongholdSecretManager]] = None):
        """Initialize `self`.
        """

        # Setup the options
        options: Dict[str, Any] = {'storagePath': storage_path}
        if client_options:
            if isinstance(client_options, ClientOptions):
                options['clientOptions'] = client_options.as_dict()
            else:
                options['clientOptions'] = client_options
        if coin_type:
            options['coinType'] = coin_type
        if secret_manager:
            options['secretManager'] = secret_manager

        options_str: str = dumps(options)

        # Create the message handler
        self.handle = create_wallet(options_str)

    def get_handle(self):
        """Return the wallet handle.
        """
        return self.handle

    def create_account(self, alias: Optional[str] = None, bech32_hrp: Optional[str]
                       = None, addresses: Optional[AccountAddress] = None) -> Account:
        """Create a new account.

        Args:
            alias: The alias of the newaccount.
            bech32_hrp: The Bech32 HRP of the new account.

        Returns:
            An account object.
        """
        account_data = self._call_method(
            'createAccount', {
                'alias': self.__return_str_or_none(alias),
                'bech32Hrp': self.__return_str_or_none(bech32_hrp),
                'addresses': addresses,
            }
        )
        return Account(account_data, self.handle)

    def get_account(self, account_id: Union[str, int]) -> Account:
        """Get the account associated with the given account ID or index.
        """
        account_data = self._call_method(
            'getAccount', {
                'accountId': account_id,
            }
        )
        return Account(account_data, self.handle)

    def get_client(self):
        """Get the client associated with the wallet.
        """
        return Client(client_handle=get_client_from_wallet(self.handle))

    def get_secret_manager(self):
        """Get the secret manager associated with the wallet.
        """
        return SecretManager(
            secret_manager_handle=get_secret_manager_from_wallet(self.handle))

    @_call_method_routine
    def _call_method(self, name: str, data=None):
        message = {
            'name': name
        }
        if data:
            message['data'] = data
        return message

    def get_account_data(self, account_id: Union[str, int]):
        """Get account data associated with the given account ID or index.
        """
        return self._call_method(
            'getAccount', {
                'accountId': account_id
            }
        )

    def get_accounts(self):
        """Get all accounts.
        """
        accounts_data = self._call_method(
            'getAccounts',
        )
        return [Account(account_data, self.handle)
                for account_data in accounts_data]

    def backup(self, destination: str, password: str):
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

    def recover_accounts(self, account_start_index: int, account_gap_limit: int,
                         address_gap_limit: int, sync_options: Optional[SyncOptions] = None):
        """Recover accounts.
        """
        return self._call_method(
            'recoverAccounts', {
                'accountStartIndex': account_start_index,
                'accountGapLimit': account_gap_limit,
                'addressGapLimit': address_gap_limit,
                'syncOptions': sync_options
            }
        )

    def remove_latest_account(self):
        """Remove latest account.
        """
        return self._call_method(
            'removeLatestAccount'
        )

    def restore_backup(self, source: str, password: str):
        """Restore a backup from a Stronghold file.
        Replaces `client_options`, `coin_type`, `secret_manager` and accounts.
        Returns an error if accounts were already created. If Stronghold is used
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
        """Update the client options for all accounts.
        """
        return self._call_method(
            'setClientOptions',
            {
                'clientOptions': client_options.as_dict()
            }
        )

    def generate_ed25519_address(self, account_index: int, internal: bool, address_index: int,
                                 options=None, bech32_hrp: Optional[str] = None) -> List[str]:
        """Generate an address without storing it.
        """
        return self._call_method(
            'generateEd25519Address', {
                'accountIndex': account_index,
                'internal': internal,
                'addressIndex': address_index,
                'options': options,
                'bech32Hrp': bech32_hrp
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

    def store_mnemonic(self, mnemonic: str):
        """Store mnemonic.
        """
        return self._call_method(
            'storeMnemonic', {
                'mnemonic': mnemonic
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

    def destroy(self):
        """Destroys the wallet instance.
        """
        return destroy_wallet(self.handle)

    # pylint: disable=redefined-builtin
    @staticmethod
    def __return_str_or_none(str):
        if str:
            return str
        return None
