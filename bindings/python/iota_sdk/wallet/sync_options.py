# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from typing import Optional
from dataclasses import dataclass
from iota_sdk.types.common import json


@json
@dataclass
class WalletSyncOptions:
    """Specifies what outputs should be synced for the ed25519 address from the wallet.

    Attributes:
        basic_outputs: Whether to sync basic outputs.
        account_outputs: whether to sync account outputs.
        nft_outputs: Whether to sync NFT outputs.
        delegation_outputs: Whether to sync delegation outputs.
    """

    basic_outputs: Optional[bool] = None
    account_outputs: Optional[bool] = None
    nft_outputs: Optional[bool] = None
    delegation_outputs: Optional[bool] = None


@json
@dataclass
class AccountSyncOptions:
    """Specifies what outputs should be synced for the address of an account output.

    Attributes:
        basic_outputs: Whether to sync basic outputs.
        account_outputs: Whether to sync account outputs.
        foundry_outputs: Whether to sync foundry outputs.
        nft_outputs: Whether to sync NFT outputs.
        delegation_outputs: Whether to sync delegation outputs.
    """

    basic_outputs: Optional[bool] = None
    account_outputs: Optional[bool] = None
    foundry_outputs: Optional[bool] = None
    nft_outputs: Optional[bool] = None
    delegation_outputs: Optional[bool] = None


@json
@dataclass
class NftSyncOptions:
    """Specifies what outputs should be synced for the address of an nft output.

    Attributes:
        basic_outputs: Whether to sync basic outputs.
        account_outputs: Whether to sync account outputs.
        nft_outputs: Whether to sync NFT outputs.
        delegation_outputs: Whether to sync delegation outputs.
    """

    basic_outputs: Optional[bool] = None
    account_outputs: Optional[bool] = None
    nft_outputs: Optional[bool] = None
    delegation_outputs: Optional[bool] = None


@json
@dataclass
class SyncOptions:
    """The synchronization options.

    **Attributes**
    force_syncing :
        Syncing is usually skipped if it's called repeatedly in a short amount of time as there can only be new changes every
        slot and calling it twice "at the same time" will not return new data.
        When this to true, we sync anyways, even if it's called 0ms after the last sync finished.
    sync_incoming_transactions :
        Try to sync transactions from incoming outputs with their inputs. Some data may not be obtained
        if it has been pruned.
    sync_pending_transactions :
        Checks pending transactions.
    account :
        Specifies what outputs should be synced for the address of an account output.
    wallet :
        Specifies what outputs should be synced for the address of an account output.
    nft :
        Specifies what outputs should be synced for the address of an nft output.
    sync_only_most_basic_outputs :
        Specifies if only basic outputs with just an address unlock condition should be synced.
        This will overwrite the `wallet`, `alias` and `nft` options.
    sync_native_token_foundries :
        Sync native token foundries, so their metadata can be returned in the balance.
    sync_implicit_accounts :
        Sync implicit accounts.
    """

    force_syncing: Optional[bool] = None
    sync_incoming_transactions: Optional[bool] = None
    sync_pending_transactions: Optional[bool] = None
    account: Optional[AccountSyncOptions] = None
    wallet: Optional[WalletSyncOptions] = None
    nft: Optional[NftSyncOptions] = None
    sync_only_most_basic_outputs: Optional[bool] = None
    sync_native_token_foundries: Optional[bool] = None
    sync_implicit_accounts: Optional[bool] = None
