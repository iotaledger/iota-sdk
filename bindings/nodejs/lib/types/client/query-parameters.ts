// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier?: Apache-2.0

import { SlotIndex } from '../block/slot';
import { Bech32Address, TokenId } from '../block';

/**
 * Common query parameters for output requests.
 */
export interface CommonOutputQueryParameters {
    /// The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
    /// returned too. The parameter is ignored when pageSize is defined via the cursor parameter.
    pageSize?: number;
    /// Starts the search from the cursor (createdSlotIndex+outputId.pageSize). If an empty String is provided, only
    /// the first page is returned.
    cursor?: String;
    /// Returns outputs that were created before a certain slot index.
    createdBefore?: SlotIndex;
    /// Returns outputs that were created after a certain slot index.
    createdAfter?: SlotIndex;
}

/**
 * Query parameters for output requests.
 */
export interface OutputQueryParameters extends CommonOutputQueryParameters {
    /// Filters outputs based on the presence of a native token.
    hasNativeToken?: boolean;
    /// Filters outputs based on the presence of a specific native token.
    nativeToken?: TokenId;
    /// Returns outputs that are unlockable by the bech32 address.
    unlockableByAddress?: Bech32Address;
}

/**
 * Query parameters for basic output requests.
 */
export interface BasicOutputQueryParameters
    extends CommonOutputQueryParameters {
    /// Filters outputs based on the presence of a native token.
    hasNativeToken?: boolean;
    /// Filters outputs based on the presence of a specific native token.
    nativeToken?: TokenId;
    /// Returns outputs that are unlockable by the bech32 address.
    unlockableByAddress?: Bech32Address;
    /// Bech32-encoded address that should be searched for.
    address?: Bech32Address;
    /// Filters outputs based on the presence of storage deposit return unlock condition.
    hasStorageDepositReturn?: boolean;
    /// Filters outputs based on the presence of a specific return address in the storage deposit return unlock
    /// condition.
    storageDepositReturnAddress?: Bech32Address;
    /// Filters outputs based on the presence of expiration unlock condition.
    hasExpiration?: boolean;
    /// Filters outputs based on the presence of a specific Bech32-encoded return address in the expiration unlock
    /// condition.
    expirationReturnAddress?: Bech32Address;
    /// Returns outputs that expire before a certain slot index.
    expiresBefore?: SlotIndex;
    /// Returns outputs that expire after a certain slot index.
    expiresAfter?: SlotIndex;
    /// Filters outputs based on the presence of timelock unlock condition.
    hasTimelock?: boolean;
    /// Returns outputs that are timelocked before a certain slot index.
    timelockedBefore?: SlotIndex;
    /// Returns outputs that are timelocked after a certain slot index.
    timelockedAfter?: SlotIndex;
    /// Filters outputs based on the presence of validated Sender (bech32 encoded).
    sender?: Bech32Address;
    /// Filters outputs based on matching Tag Block.
    tag?: String;
}

/**
 * Query parameters for account output requests.
 */
export interface AccountOutputQueryParameters
    extends CommonOutputQueryParameters {
    /// Bech32-encoded address that should be searched for.
    address?: Bech32Address;
    /// Filters outputs based on bech32-encoded issuer address.
    issuer?: Bech32Address;
    /// Filters outputs based on the presence of validated Sender (bech32 encoded).
    sender?: Bech32Address;
}

/**
 * Query parameters for anchor output requests.
 */
export interface AnchorOutputQueryParameters
    extends CommonOutputQueryParameters {
    /// Returns outputs that are unlockable by the bech32 address.
    unlockableByAddress?: Bech32Address;
    /// Filters outputs based on bech32-encoded state controller address.
    stateController?: Bech32Address;
    /// Filters outputs based on bech32-encoded governor (governance controller) address.
    governor?: Bech32Address;
    /// Filters outputs based on bech32-encoded issuer address.
    issuer?: Bech32Address;
    /// Filters outputs based on the presence of validated Sender (bech32 encoded).
    sender?: Bech32Address;
}

/**
 * Query parameters for delegation output requests.
 */
export interface DelegationOutputQueryParameters
    extends CommonOutputQueryParameters {
    /// Bech32-encoded address that should be searched for.
    address?: Bech32Address;
    /// Filter foundry outputs based on bech32-encoded address of the validator.
    validator?: Bech32Address;
}

/**
 * Query parameters for foundry output requests.
 */
export interface FoundryOutputQueryParameters
    extends CommonOutputQueryParameters {
    /// Filters outputs based on the presence of a native token.
    hasNativeToken?: boolean;
    /// Filters outputs based on the presence of a specific native token.
    nativeToken?: TokenId;
    /// Filter foundry outputs based on bech32-encoded address of the controlling account.
    account?: Bech32Address;
}

/**
 * Query parameters for NFT output requests.
 */
export interface NftOutputQueryParameters extends CommonOutputQueryParameters {
    /// Returns outputs that are unlockable by the bech32 address.
    unlockableByAddress?: Bech32Address;
    /// Bech32-encoded address that should be searched for.
    address?: Bech32Address;
    /// Filters outputs based on the presence of storage deposit return unlock condition.
    hasStorageDepositReturn?: boolean;
    /// Filters outputs based on the presence of a specific return address in the storage deposit return unlock
    /// condition.
    storageDepositReturnAddress?: Bech32Address;
    /// Filters outputs based on the presence of expiration unlock condition.
    hasExpiration?: boolean;
    /// Filters outputs based on the presence of a specific Bech32-encoded return address in the expiration unlock
    /// condition.
    expirationReturnAddress?: Bech32Address;
    /// Returns outputs that expire before a certain slot index.
    expiresBefore?: SlotIndex;
    /// Returns outputs that expire after a certain slot index.
    expiresAfter?: SlotIndex;
    /// Filters outputs based on the presence of timelock unlock condition.
    hasTimelock?: boolean;
    /// Returns outputs that are timelocked before a certain slot index.
    timelockedBefore?: SlotIndex;
    /// Returns outputs that are timelocked after a certain slot index.
    timelockedAfter?: SlotIndex;
    /// Filters outputs based on bech32-encoded issuer address.
    issuer?: Bech32Address;
    /// Filters outputs based on the presence of validated Sender (bech32 encoded).
    sender?: Bech32Address;
    /// Filters outputs based on matching Tag Block.
    tag?: String;
}
