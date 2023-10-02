// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { AccountAddress, AddressWithUnspentOutputs } from './address';
import { AliasId, FoundryId, NftId } from '../block/id';
import type { OutputData } from './output';
import type { Transaction } from './transaction';
import { CoinType } from '../../client';
import { HexEncodedString } from '../utils';
import { Bech32Address } from '../block/address';

/**
 * Account identifier
 * Could be the account index (number) or account alias (string)
 */
export type AccountId = number | string;

/** A balance */
export interface Balance {
    /** The balance of the base coin */
    baseCoin: BaseCoinBalance;
    /** The required storage deposit for the outputs */
    requiredStorageDeposit: RequiredStorageDeposit;
    /** The balance of the native tokens */
    nativeTokens: NativeTokenBalance[];
    /** Nft outputs */
    nfts: string[];
    /** Alias outputs */
    aliases: string[];
    /** Foundry outputs */
    foundries: string[];
    /**
     * Outputs with multiple unlock conditions and if they can currently be spent or not. If there is a
     * TimelockUnlockCondition or ExpirationUnlockCondition this can change at any time
     */
    potentiallyLockedOutputs: { [outputId: string]: boolean };
}

/** The balance of the base coin */
export interface BaseCoinBalance {
    /** The total amount of the outputs */
    total: bigint;
    /** The amount of the outputs that aren't used in a transaction */
    available: bigint;
    /** Voting power */
    votingPower: string;
}

/** The required storage deposit per output type */
export interface RequiredStorageDeposit {
    /** The required amount for Alias outputs. */
    alias: bigint;
    /** The required amount for Basic outputs. */
    basic: bigint;
    /** The required amount for Foundry outputs. */
    foundry: bigint;
    /** The required amount for NFT outputs. */
    nft: bigint;
}

/** The balance of a native token */
export interface NativeTokenBalance {
    /** The native token id. */
    tokenId: HexEncodedString;
    /** Some metadata of the native token. */
    metadata?: string;
    /** The total native token balance. */
    total: bigint;
    /** The available amount of the total native token balance. */
    available: bigint;
}

/** Sync options for an account */
export interface SyncOptions {
    /**
     * Specific Bech32 encoded addresses of the account to sync, if addresses are provided,
     * then `address_start_index` will be ignored
     */
    addresses?: Bech32Address[];
    /**
     * Address index from which to start syncing addresses. 0 by default, using a higher index will be faster because
     * addresses with a lower index will be skipped, but could result in a wrong balance for that reason
     */
    addressStartIndex?: number;
    /**
     * Address index from which to start syncing internal addresses. 0 by default, using a higher index will be faster
     * because addresses with a lower index will be skipped, but could result in a wrong balance for that reason
     */
    addressStartIndexInternal?: number;
    /**
     * Usually syncing is skipped if it's called in between 200ms, because there can only be new changes every
     * milestone and calling it twice "at the same time" will not return new data
     * When this to true, we will sync anyways, even if it's called 0ms after the las sync finished. Default: false.
     */
    forceSyncing?: boolean;
    /// Try to sync transactions from incoming outputs with their inputs. Some data may not be obtained if it has been
    /// pruned.
    syncIncomingTransactions?: boolean;
    /** Checks pending transactions and promotes/reattaches them if necessary. Default: true. */
    syncPendingTransactions?: boolean;
    /** Specifies what outputs should be synced for the ed25519 addresses from the account. */
    account?: AccountSyncOptions;
    /** Specifies what outputs should be synced for the address of an alias output. */
    alias?: AliasSyncOptions;
    /** Specifies what outputs should be synced for the address of an nft output. */
    nft?: NftSyncOptions;
    /** Specifies if only basic outputs with an AddressUnlockCondition alone should be synced, will overwrite `account`, `alias` and `nft` options. Default: false. */
    syncOnlyMostBasicOutputs?: boolean;
    /** Sync native token foundries, so their metadata can be returned in the balance. Default: false. */
    syncNativeTokenFoundries?: boolean;
}

/** Specifies what outputs should be synced for the ed25519 addresses from the account. */
export interface AccountSyncOptions {
    /** Whether to sync Basic outputs. */
    basicOutputs?: boolean;
    /** Whether to sync Alias outputs. */
    aliasOutputs?: boolean;
    /** Whether to sync NFT outputs. */
    nftOutputs?: boolean;
}

/** Specifies what outputs should be synced for the address of an alias output. */
export interface AliasSyncOptions {
    /** Whether to sync Basic outputs. */
    basicOutputs?: boolean;
    /** Whether to sync Alias outputs. */
    aliasOutputs?: boolean;
    /** Whether to sync NFT outputs. */
    nftOutputs?: boolean;
    /** Whether to sync foundry outputs. */
    foundryOutputs?: boolean;
}

/** Specifies what outputs should be synced for the address of an nft output. */
export interface NftSyncOptions {
    /** Whether to sync Basic outputs. */
    basicOutputs?: boolean;
    /** Whether to sync Alias outputs. */
    aliasOutputs?: boolean;
    /** Whether to sync NFT outputs. */
    nftOutputs?: boolean;
}

/** The account object. */
export interface AccountMeta {
    /** The account index. */
    index: number;
    /** The type of coin managed with the account. */
    coinType: CoinType;
    /** The alias name of the account. */
    alias: string;
    /** All public addresses. */
    publicAddresses: AccountAddress[];
    /** All internal addresses. */
    internalAddresses: AccountAddress[];
    /** All addresses with unspent outputs. */
    addressesWithUnspentOutputs: AddressWithUnspentOutputs[];
    /** All outputs of the account. */
    outputs: { [outputId: string]: OutputData };
    /** All IDs of unspent outputs that are currently used as inputs for transactions. */
    lockedOutputs: Set<string>;
    /** All unspent outputs of the account. */
    unspentOutputs: { [outputId: string]: OutputData };
    /** All transactions of the account. */
    transactions: { [transactionId: string]: Transaction };
    /** All pending transactions of the account. */
    pendingTransactions: Set<string>;
    /** All incoming transactions of the account (with their inputs if available and not already pruned). */
    incomingTransactions: {
        [transactionId: string]: [Transaction];
    };
}

/** The account metadata. */
export interface AccountMetadata {
    /** The account alias */
    alias: string;
    /** The used coin type */
    coinType: CoinType;
    /** The account index which will be used in the BIP32 path */
    index: number;
}

/** Options for account creation. */
export interface CreateAccountPayload {
    /** An account alias name. */
    alias?: string;
    /** The Bech32 HRP (human readable part) to use. */
    bech32Hrp?: string;
    /** Account addresses to use. */
    addresses?: AccountAddress[];
}

/** Options to filter outputs */
export interface FilterOptions {
    /** Filter all outputs where the booked milestone index is below the specified timestamp */
    lowerBoundBookedTimestamp?: number;
    /** Filter all outputs where the booked milestone index is above the specified timestamp */
    upperBoundBookedTimestamp?: number;
    /** Filter all outputs for the provided types (Basic = 3, Alias = 4, Foundry = 5, NFT = 6) */
    outputTypes?: number[];
    /** Return all alias outputs matching these IDs. */
    aliasIds?: AliasId[];
    /** Return all foundry outputs matching these IDs. */
    foundryIds?: FoundryId[];
    /** Return all NFT outputs matching these IDs. */
    nftIds?: NftId[];
}
