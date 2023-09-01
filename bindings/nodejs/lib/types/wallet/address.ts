// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SlotIndex } from '../block/slot';
import { u256, u64 } from '../utils';

/** A Bip44 address */
export interface Bip44Address {
    /** The Bech32 address. */
    address: string;
    /** The address key index. */
    keyIndex: number;
    /** Whether the address is a public or an internal (change) address. */
    internal: boolean;
}

/** Address with a base token amount */
export interface SendParams {
    /** The Bech32 address to send the amount to. */
    address: string;
    /** The amount to send. */
    amount: u64 | string;
    /**
     * Bech32 encoded address, to which the storage deposit will be returned if one is necessary
     * given the provided amount. If a storage deposit is needed and a return address is not provided, it will
     * default to the first address of the account.
     */
    returnAddress?: string;
    /**
     * Expiration in seconds, after which the output will be available for the sender again, if not spent by the
     * receiver already. The expiration will only be used if one is necessary given the provided amount. If an
     * expiration is needed but not provided, it will default to one day.
     */
    expiration?: SlotIndex;
}

/** Address with unspent outputs */
export interface AddressWithUnspentOutputs {
    /** The Bech32 address. */
    address: string;
    /** The address key index. */
    keyIndex: number;
    /** Whether the address is a public or an internal (change) address. */
    internal: boolean;
    /** The IDs of associated unspent outputs. */
    outputIds: string[];
}

/** Address with native tokens */
export interface SendNativeTokensParams {
    /** The Bech32 address. */
    address: string;
    /** The Native Tokens to send. */
    nativeTokens: [string, u256][];
    /**
     * Bech32 encoded address, to which the storage deposit will be returned.
     * Default will use the first address of the account.
     */
    returnAddress?: string;
    /**
     * Expiration in seconds, after which the output will be available for the sender again, if not spent by the
     * receiver before. Default is 1 day.
     */
    expiration?: SlotIndex;
}

/** Address with an NftId */
export interface SendNftParams {
    /** The Bech32 address. */
    address: string;
    /** The ID of the NFT to send. */
    nftId: string;
}

/** Options for address generation, useful with a Ledger Nano SecretManager */
export interface GenerateAddressOptions {
    /** Whether to generate a public or an internal (change) address. */
    internal: boolean;
    /** Whether to display the generated address on Ledger Nano devices. */
    ledgerNanoPrompt: boolean;
}
