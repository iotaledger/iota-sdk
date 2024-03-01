// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SlotIndex } from '../block/slot';
import { Bech32Address, NftId, TokenId } from '../block';
import { NumericString, u256, u64 } from '../utils';
import { ReturnStrategy } from './output-params';

/** Address with a base token amount */
export interface SendParams {
    /** The Bech32 address to send the amount to. */
    address: Bech32Address;
    /** The amount to send. */
    amount: u64 | NumericString;
    /**
     * Bech32 encoded address, to which the storage deposit will be returned if one is necessary
     * given the provided amount. If a storage deposit is needed and a return address is not provided, it will
     * default to the address of the wallet.
     */
    returnAddress?: Bech32Address;
    /**
     * Expiration in seconds, after which the output will be available for the sender again, if not spent by the
     * receiver already. The expiration will only be used if one is necessary given the provided amount. If an
     * expiration is needed but not provided, it will default to one day.
     */
    expiration?: SlotIndex;
}

/** Address with native token */
export interface SendNativeTokenParams {
    /** The Bech32 address. */
    address: Bech32Address;
    /** The Native Token to send. */
    nativeToken: [TokenId, u256];
    /**
     * Bech32 encoded address, to which the storage deposit will be returned.
     * Default will use the address of the wallet.
     */
    returnAddress?: Bech32Address;
    /**
     * Expiration in seconds, after which the output will be available for the sender again, if not spent by the
     * receiver before. Default is 1 day.
     */
    expiration?: SlotIndex;
}

/** Address with an NftId */
export interface SendNftParams {
    /** The Bech32 address. */
    address: Bech32Address;
    /** The ID of the NFT to send. */
    nftId: NftId;
}

/** Parameters for sending mana. */
export interface SendManaParams {
    /** Amount of mana to send, e.g. 1000000. */
    mana: u64 | NumericString;
    /** Recipient address, e.g. rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3. */
    address: Bech32Address;
    /** Whether to gift the storage deposit or not. */
    return_strategy?: ReturnStrategy;
}
