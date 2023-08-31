// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SlotIndex } from '../block/slot';
import { u256, u64 } from '../utils';

/** A Bip44 address */
export interface Bip44Address {
    address: string;
    keyIndex: number;
    internal: boolean;
}

/** Address with a base token amount */
export interface SendParams {
    address: string;
    amount: u64 | string;
    returnAddress?: string;
    expiration?: SlotIndex;
}

/** Address with unspent outputs */
export interface AddressWithUnspentOutputs {
    address: string;
    keyIndex: number;
    internal: boolean;
    outputIds: string[];
}

/** Address with native tokens */
export interface SendNativeTokensParams {
    address: string;
    nativeTokens: [string, u256][];
    returnAddress?: string;
    expiration?: SlotIndex;
}

/** Address with an NftId */
export interface SendNftParams {
    address: string;
    nftId: string;
}

/** Options for address generation, useful with a Ledger Nano SecretManager */
export interface GenerateAddressOptions {
    internal: boolean;
    ledgerNanoPrompt: boolean;
}
