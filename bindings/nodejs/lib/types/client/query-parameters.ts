// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Bech32Address } from '../block';

/**
 * Query parameter for filtering output requests
 */
export type QueryParameter =
    | Address
    | AliasAddress
    | HasStorageDepositReturn
    | StorageDepositReturnAddress
    | HasTimelock
    | TimelockedBefore
    | TimelockedAfter
    | HasExpiration
    | ExpiresBefore
    | ExpiresAfter
    | ExpirationReturnAddress
    | Sender
    | Tag
    | Issuer
    | StateController
    | Governor
    | UnlockableByAddress
    | CommonQueryParameters;

/** Query parameters for filtering Alias Outputs */
export type AliasQueryParameter =
    | StateController
    | Governor
    | Issuer
    | Sender
    | UnlockableByAddress
    | CommonQueryParameters;

/** Query parameters for filtering Foundry Outputs */
export type FoundryQueryParameter = AliasAddress | CommonQueryParameters;

/** Query parameters for filtering Nft Outputs */
export type NftQueryParameter =
    | Address
    | HasStorageDepositReturn
    | StorageDepositReturnAddress
    | HasTimelock
    | TimelockedBefore
    | TimelockedAfter
    | HasExpiration
    | ExpiresBefore
    | ExpiresAfter
    | ExpirationReturnAddress
    | Issuer
    | Sender
    | Tag
    | UnlockableByAddress
    | CommonQueryParameters;

/** Shared query parameters*/
type CommonQueryParameters =
    | HasNativeTokens
    | MinNativeTokenCount
    | MaxNativeTokenCount
    | CreatedAfter
    | CreatedBefore
    | PageSize
    | Cursor;

/** Query parameters for filtering alias/basic/NFT/foundry Outputs*/
export type GenericQueryParameter =
    | UnlockableByAddress
    | HasNativeTokens
    | MinNativeTokenCount
    | MaxNativeTokenCount
    | CreatedAfter
    | CreatedBefore
    | PageSize
    | Cursor;

/** Bech32-encoded address that should be searched for. */
interface Address {
    address: Bech32Address;
}
/** Filter foundry outputs based on bech32-encoded address of the controlling alias. */
interface AliasAddress {
    aliasAddress: Bech32Address;
}
/** Filters outputs based on the presence of storage deposit return unlock condition. */
interface HasStorageDepositReturn {
    hasStorageDepositReturn: boolean;
}
/** Filter outputs based on the presence of a specific Bech32-encoded return address
 * in the storage deposit return unlock condition.
 */
interface StorageDepositReturnAddress {
    storageDepositReturnAddress: Bech32Address;
}
/** Filters outputs based on the presence of timelock unlock condition. */
interface HasTimelock {
    hasTimelock: boolean;
}
/** Return outputs that are timelocked before a certain Unix timestamp. */
interface TimelockedBefore {
    timelockedBefore: number;
}
/** Return outputs that are timelocked after a certain Unix timestamp. */
interface TimelockedAfter {
    timelockedAfter: number;
}

/** Filters outputs based on the presence of expiration unlock condition. */
interface HasExpiration {
    hasExpiration: boolean;
}
/** Filters outputs based on the presence of native tokens. */
interface HasNativeTokens {
    hasNativeTokens: boolean;
}
/** Filters outputs that have at most a certain number of distinct native tokens. */
interface MaxNativeTokenCount {
    maxNativeTokenCount: number;
}
/** Filters outputs that have at least a certain number of distinct native tokens. */
interface MinNativeTokenCount {
    minNativeTokenCount: number;
}
/** Return outputs that expire before a certain Unix timestamp. */
interface ExpiresBefore {
    expiresBefore: number;
}
/** Return outputs that expire after a certain Unix timestamp. */
interface ExpiresAfter {
    expiresAfter: number;
}
/** Filter outputs based on the presence of a specific Bech32-encoded return
 * address in the expiration unlock condition.
 * */
interface ExpirationReturnAddress {
    expirationReturnAddress: Bech32Address;
}
/** Filter for a certain sender */
interface Sender {
    sender: string;
}
/** Filter for a certain tag */
interface Tag {
    tag: string;
}
/** Return outputs that were created before a certain Unix timestamp. */
interface CreatedBefore {
    createdBefore: number;
}
/** Return outputs that were created after a certain Unix timestamp. */
interface CreatedAfter {
    createdAfter: number;
}
/** Pass the cursor(confirmationMS+outputId.pageSize) to start the results from */
interface Cursor {
    cursor: string;
}
/** Filter for a certain issuer */
interface Issuer {
    issuer: string;
}
/** Filter outputs based on bech32-encoded state controller address. */
interface StateController {
    stateController: Bech32Address;
}
/** Filter outputs based on bech32-encoded governor (governance controller) address. */
interface Governor {
    governor: Bech32Address;
}
/** Define the page size for the results. */
interface PageSize {
    pageSize: number;
}
/** Returns outputs that are unlockable by the bech32 address. */
interface UnlockableByAddress {
    unlockableByAddress: Bech32Address;
}
