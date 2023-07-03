from typing import List, Optional


class AccountSyncOptions():
    """Sync options for Ed25519 addresses from the account
    """

    def __init__(self,
                 basic_outputs: Optional[bool] = None,
                 nft_outputs: Optional[bool] = None,
                 alias_outputs: Optional[bool] = None):
        """Initialize AccountSyncOptions
        """
        self.basicOutputs = basic_outputs
        self.nftOutputs = nft_outputs
        self.aliasOutputs = alias_outputs


class AliasSyncOptions():
    """Sync options for addresses from alias outputs
    """

    def __init__(self,
                 basic_outputs: Optional[bool] = None,
                 nft_outputs: Optional[bool] = None,
                 alias_outputs: Optional[bool] = None,
                 foundry_outputs: Optional[bool] = None):
        """Initialize AliasSyncOptions
        """
        self.basicOutputs = basic_outputs
        self.nftOutputs = nft_outputs
        self.aliasOutputs = alias_outputs
        self.foundryOutputs = foundry_outputs


class NftSyncOptions():
    """Sync options for addresses from NFT outputs
    """

    def __init__(self,
                 basic_outputs: Optional[bool] = None,
                 nft_outputs: Optional[bool] = None,
                 alias_outputs: Optional[bool] = None):
        """Initialize NftSyncOptions
        """
        self.basicOutputs = basic_outputs
        self.nftOutputs = nft_outputs
        self.aliasOutputs = alias_outputs


class SyncOptions():
    """The synchronization options
    """

    def __init__(self,
                 addresses: Optional[List[str]] = None,
                 address_start_index: Optional[int] = None,
                 address_start_index_internal: Optional[int] = None,
                 force_syncing: Optional[bool] = None,
                 sync_incoming_transactions: Optional[bool] = None,
                 sync_pending_transactions: Optional[bool] = None,
                 account: Optional[AccountSyncOptions] = None,
                 alias: Optional[AliasSyncOptions] = None,
                 nft: Optional[NftSyncOptions] = None,
                 sync_only_most_basic_outputs: Optional[bool] = None,
                 sync_native_token_foundries: Optional[bool] = None):
        """Initialize SyncOptions
        """
        self.addresses = addresses
        self.addressStartIndex = address_start_index
        self.addressStartIndexInternal = address_start_index_internal
        self.forceSyncing = force_syncing
        self.syncIncomingTransactions = sync_incoming_transactions
        self.syncPendingTransactions = sync_pending_transactions
        self.account = account
        self.alias = alias
        self.nft = nft
        self.syncOnlyMostBasicOutputs = sync_only_most_basic_outputs
        self.syncNativeTokenFoundries = sync_native_token_foundries

    def as_dict(self):
        return dict(self.__dict__)
