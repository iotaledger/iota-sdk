// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    AccountId,
    DelegationId,
    FoundryId,
    NftId,
    TokenId,
} from '../block/id';
import { DecayedMana, HexEncodedString, u256, u64 } from '../utils';
import { ClientOptions } from '../client';
import { Bip44, SecretManagerType } from '../secret_manager/secret-manager';
import { Bech32Address, SlotIndex } from '../block';

/** Options for the Wallet builder. */
export interface WalletOptions {
    /** The wallet address. */
    address?: Bech32Address;
    /** The alias of the wallet. */
    alias?: string;
    /** The the BIP44 path of the wallet. */
    bipPath?: Bip44;
    /** The node client options. */
    clientOptions?: ClientOptions;
    /** The secret manager to use. */
    secretManager?: SecretManagerType;
    /** The path to the wallet database. */
    storagePath?: string;
}

/** A balance */
export interface Balance {
    /** Total and available amount of the base coin */
    baseCoin: BaseCoinBalance;
    /** Total and available mana  */
    mana: ManaBalance;
    /** The required storage deposit for the outputs */
    requiredStorageDeposit: RequiredStorageDeposit;
    /** The balance of the native tokens */
    nativeTokens: { [tokenId: TokenId]: NativeTokenBalance };
    /** Account outputs */
    accounts: AccountId[];
    /** Foundry outputs */
    foundries: FoundryId[];
    /** Nft outputs */
    nfts: NftId[];
    /** Delegation outputs */
    delegations: DelegationId[];
    /**
     * Outputs with multiple unlock conditions and if they can currently be spent or not. If there is a
     * TimelockUnlockCondition or ExpirationUnlockCondition this can change at any time
     */
    potentiallyLockedOutputs: { [outputId: HexEncodedString]: boolean };
}

/** The balance of the base coin */
export interface BaseCoinBalance {
    /** The total amount of the outputs */
    total: u64;
    /** The amount of the outputs that aren't used in a transaction */
    available: u64;
    /** Voting power */
    votingPower: u64;
}

/** Mana balances */
export interface ManaBalance {
    /** The total mana of the outputs */
    total: DecayedMana;
    /** The mana of the outputs that isn't used in a transaction */
    available: DecayedMana;
    /** Mana rewards. */
    rewards: u64;
}

/** The required storage deposit per output type */
export interface RequiredStorageDeposit {
    /** The required amount for Basic outputs. */
    basic: u64;
    /** The required amount for Account outputs. */
    account: u64;
    /** The required amount for Foundry outputs. */
    foundry: u64;
    /** The required amount for NFT outputs. */
    nft: u64;
    /** The required amount for Delegation outputs. */
    delegation: u64;
}

/** The balance of a native token */
export interface NativeTokenBalance {
    /** Some metadata of the native token. */
    metadata?: string;
    /** The total native token balance. */
    total: u256;
    /** The available amount of the total native token balance. */
    available: u256;
}

/** Sync options for a wallet */
export interface SyncOptions {
    /**
     * Syncing is usually skipped if it's called repeatedly in a short amount of time as there can only be new changes every
     * slot and calling it twice "at the same time" will not return new data.
     * When this to true, we sync anyways, even if it's called 0ms after the last sync finished. Default: false.
     */
    forceSyncing?: boolean;
    /// Try to sync transactions from incoming outputs with their inputs. Some data may not be obtained if it has been
    /// pruned.
    syncIncomingTransactions?: boolean;
    /** Checks pending transactions. Default: true. */
    syncPendingTransactions?: boolean;
    /** Specifies what outputs should be synced for the ed25519 address from the wallet. */
    wallet?: WalletSyncOptions;
    /** Specifies what outputs should be synced for the address of an account output. */
    account?: AccountSyncOptions;
    /** Specifies what outputs should be synced for the address of an nft output. */
    nft?: NftSyncOptions;
    /** Specifies if only basic outputs with an AddressUnlockCondition alone should be synced, will overwrite `wallet`, `account` and `nft` options. Default: false. */
    syncOnlyMostBasicOutputs?: boolean;
    /** Sync native token foundries, so their metadata can be returned in the balance. Default: false. */
    syncNativeTokenFoundries?: boolean;
    /// Sync implicit accounts.
    syncImplicitAccounts?: boolean;
}

/** Specifies what outputs should be synced for the ed25519 address from the wallet. */
export interface WalletSyncOptions {
    /** Whether to sync Basic outputs. */
    basicOutputs?: boolean;
    /** Whether to sync Account outputs. */
    accountOutputs?: boolean;
    /** Whether to sync NFT outputs. */
    nftOutputs?: boolean;
    /** Whether to sync delegation outputs. */
    delegationOutputs?: boolean;
}

/** Specifies what outputs should be synced for the address of an account output. */
export interface AccountSyncOptions {
    /** Whether to sync Basic outputs. */
    basicOutputs?: boolean;
    /** Whether to sync Account outputs. */
    accountOutputs?: boolean;
    /** Whether to sync foundry outputs. */
    foundryOutputs?: boolean;
    /** Whether to sync NFT outputs. */
    nftOutputs?: boolean;
    /** Whether to sync delegation outputs. */
    delegationOutputs?: boolean;
}

/** Specifies what outputs should be synced for the address of an nft output. */
export interface NftSyncOptions {
    /** Whether to sync Basic outputs. */
    basicOutputs?: boolean;
    /** Whether to sync Account outputs. */
    accountOutputs?: boolean;
    /** Whether to sync NFT outputs. */
    nftOutputs?: boolean;
    /** Whether to sync delegation outputs. */
    delegationOutputs?: boolean;
}

/** Options to filter outputs */
export interface FilterOptions {
    /** Include all outputs where the included slot is below the specified slot */
    includedBelowSlot?: SlotIndex;
    /** Include all outputs where the included slot is above the specified slot */
    includedAboveSlot?: SlotIndex;
    /** Filter all outputs for the provided types (Basic = 3, Account = 4, Foundry = 5, NFT = 6) */
    outputTypes?: number[];
    /** Return all account outputs matching these IDs. */
    accountIds?: AccountId[];
    /** Return all foundry outputs matching these IDs. */
    foundryIds?: FoundryId[];
    /** Return all NFT outputs matching these IDs. */
    nftIds?: NftId[];
    /** Return all delegation outputs matching these IDs. */
    delegationIds?: DelegationId[];
}
