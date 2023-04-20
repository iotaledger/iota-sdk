# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk import destroy_wallet, create_wallet, listen_wallet
from iota_sdk.wallet.account import Account, _call_method_routine
from json import dumps


class Wallet():
    def __init__(self, storage_path='./walletdb', client_options=None, coin_type=None, secret_manager=None):
        """Initialize the IOTA Wallet.
        """

        # Setup the options
        options = {'storagePath': storage_path}
        if client_options:
            options['clientOptions'] = client_options
        if coin_type:
            options['coinType'] = int(coin_type)
        if secret_manager:
            options['secretManager'] = secret_manager

        options = dumps(options)

        # Create the message handler
        self.handle = create_wallet(options)

    def get_handle(self):
        return self.handle

    def create_account(self, alias=None, bech32_hrp=None):
        """Create a new account
        """
        return self._call_method(
            'createAccount', {
                'alias': self.__return_str_or_none(alias),
                'bech32Hrp': self.__return_str_or_none(bech32_hrp),
            }
        )

    def get_account(self, account_id):
        """Get the account instance
        """
        return Account(account_id, self.handle)

    @_call_method_routine
    def _call_method(self, name, data=None):
        message = {
            'name': name
        }
        if data:
            message['data'] = data
        return message

    def get_account_data(self, account_id):
        """Get account data
        """
        return self._call_method(
            'getAccount', {
                'accountId': account_id
            }
        )

    def get_accounts(self):
        """Get accounts
        """
        return self._call_method(
            'getAccounts',
        )

    def backup(self, destination, password):
        """Backup storage.
        """
        return self._call_method(
            'backup', {
                'destination': destination,
                'password': password
            }
        )

    def bech32_to_hex(self, bech32_address):
        """Transforms a bech32 encoded address to hex
        """
        return self._call_method(
            'bech32ToHex', {
                'bech32Address': bech32_address,
            }
        )

    def change_stronghold_password(self, password):
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

    def is_stronghold_password_available(self):
        """Is stronghold password available.
        """
        return self._call_method(
            'isStrongholdPasswordAvailable'
        )

    def recover_accounts(self, account_start_index, account_gap_limit, address_gap_limit, sync_options):
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

    def restore_backup(self, source, password):
        """Restore a backup from a Stronghold file
           Replaces client_options, coin_type, secret_manager and accounts. Returns an error if accounts were already created
           If Stronghold is used as secret_manager, the existing Stronghold file will be overwritten. If a mnemonic was
           stored, it will be gone.
        """
        return self._call_method(
            'restoreBackup', {
                'source': source,
                'password': password
            }
        )

    def generate_mnemonic(self):
        """Generates a new mnemonic.
        """
        return self._call_method(
            'generateMnemonic'
        )

    def verify_mnemonic(self, mnemonic):
        """Checks if the given mnemonic is valid.
        """
        return self._call_method(
            'verifyMnemonic', {
                'mnemonic': mnemonic
            }
        )

    def set_client_options(self, client_options):
        """Updates the client options for all accounts.
        """
        return self._call_method(
            'setClientOptions',
            {
                'clientOptions': client_options
            }
        )

    def generate_address(self, account_index, internal, address_index, options=None, bech32_hrp=None):
        """Generate an address without storing it.
        """
        return self._call_method(
            'generateAddress', {
                'accountIndex': account_index,
                'internal': internal,
                'addressIndex': address_index,
                'options': options,
                'bech32Hrp': bech32_hrp
            }
        )

    def get_node_info(self, url, auth):
        """Get node info.
        """
        return self._call_method(
            'getNodeInfo', {
                'url': url,
                'auth': auth
            }
        )

    def set_stronghold_password(self, password):
        """Set stronghold password.
        """
        return self._call_method(
            'setStrongholdPassword', {
                'password': password
            }

        )

    def set_stronghold_password_clear_interval(self, interval_in_milliseconds):
        """Set stronghold password clear interval.
        """
        return self._call_method(
            'setStrongholdPasswordClearInterval', {
                'intervalInMilliseconds': interval_in_milliseconds
            }
        )

    def store_mnemonic(self, mnemonic):
        """Store mnemonic.
        """
        return self._call_method(
            'storeMnemonic', {
                'mnemonic': mnemonic
            }

        )

    def start_background_sync(self, options, interval_in_milliseconds):
        """Start background sync.
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

    def listen(self, handler, events=None):
        """Listen to wallet events, empty array or None will listen to all events
           The default value for events is None
        """
        events_array = [] if events is None else events
        listen_wallet(self.handle, events_array, handler)

    def clear_listeners(self, events=None):
        """Remove wallet event listeners, empty array or None will remove all listeners
           The default value for events is None
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

    @staticmethod
    def __return_str_or_none(str):
        if str:
            return str
        else:
            return None
