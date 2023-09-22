# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from typing import List, Optional
from dataclasses import dataclass
from iota_sdk.types.common import json

@json
@dataclass
class AccountSyncOptions():
    """Sync options for addresses from the account.

    Attributes:
        basic_outputs: Whether to sync basic outputs.
        nft_outputs: Whether to sync NFT outputs.
        account_outputs: whether to sync account outputs.
    """

    basic_outputs: Optional[bool] = None
    nft_outputs: Optional[bool] = None
    account_outputs: Optional[bool] = None


@json
@dataclass
class AliasSyncOptions():
    """Sync options for addresses from account outputs.

    Attributes:
        basic_outputs: Whether to sync basic outputs.
        nft_outputs: Whether to sync NFT outputs.
        account_outputs: Whether to sync account outputs.
        foundry_outputs: Whether to sync foundry outputs.
    """

    basic_outputs: Optional[bool] = None
    nft_outputs: Optional[bool] = None
    account_outputs: Optional[bool] = None
    foundry_outputs: Optional[bool] = None


@json
@dataclass
class NftSyncOptions():
    """Sync options for addresses from NFT outputs.

    Attributes:
        basic_outputs: Whether to sync basic outputs.
        nft_outputs: Whether to sync NFT outputs.
        account_outputs: Whether to sync account outputs.
    """

    basic_outputs: Optional[bool] = None
    nft_outputs: Optional[bool] = None
    account_outputs: Optional[bool] = None


@json
@dataclass
class SyncOptions():
    """The synchronization options.

    **Attributes**
    addresses :
        Specific Bech32 encoded addresses of the account to sync. If addresses are provided,
        then `address_start_index` will be ignored.
    address_start_index :
        Address index from which to start syncing addresses. 0 by default.
        Using a higher index will be faster because addresses with a lower index will be skipped,
        but this could result in a wrong balance for that reason.
    address_start_index_internal :
        Address index from which to start syncing internal addresses. 0 by default.
        Using a higher index will be faster because addresses with a lower index will be skipped,
        but this could result in a wrong balance for internal addresses for that reason.
    force_syncing :
        Usually syncing is skipped if it's called in between 200ms, because there can only be new
        changes every milestone and calling it twice "at the same time" will not return new data.
        When this is set to true, we will sync anyways, even if it's called 0ms after the last sync
        finished.
    sync_incoming_transactions :
        Try to sync transactions from incoming outputs with their inputs. Some data may not be obtained
        if it has been pruned.
    sync_pending_transactions :
        Checks pending transactions and reissues them if necessary.
    account :
        Specifies what outputs should be synced for the Ed25519 addresses from the account.
    alias :
        Specifies what outputs should be synced for the address of an account output.
    nft :
        Specifies what outputs should be synced for the address of an nft output.
    sync_only_most_basic_outputs :
        Specifies if only basic outputs with just an address unlock condition should be synced.
        This will overwrite the `account`, `alias` and `nft` options.
    sync_native_token_foundries :
        Sync native token foundries, so their metadata can be returned in the balance.
    """

    addresses: Optional[List[str]] = None
    address_start_index: Optional[int] = None
    address_start_index_internal: Optional[int] = None
    force_syncing: Optional[bool] = None
    sync_incoming_transactions: Optional[bool] = None
    sync_pending_transactions: Optional[bool] = None
    account: Optional[AccountSyncOptions] = None
    # TODO Rename when we are done with Account changes
    # https://github.com/iotaledger/iota-sdk/issues/647.
    alias: Optional[AliasSyncOptions] = None
    nft: Optional[NftSyncOptions] = None
    sync_only_most_basic_outputs: Optional[bool] = None
    sync_native_token_foundries: Optional[bool] = None
