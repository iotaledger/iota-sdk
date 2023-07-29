// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/** An Address of the Account */
export interface AccountAddress {
    /** TODO */
    address: string;
    /** TODO */
    keyIndex: number;
    /** TODO */
    internal: boolean;
    /** TODO */
    used: boolean;
}

/** Address with a base token amount */
export interface SendParams {
    /** TODO */
    address: string;
    /** TODO */
    amount: bigint | string;
    /** TODO */
    returnAddress?: string;
    /** TODO */
    expiration?: number;
}

/** Address with unspent outputs */
export interface AddressWithUnspentOutputs {
    /** TODO */
    address: string;
    /** TODO */
    keyIndex: number;
    /** TODO */
    internal: boolean;
    /** TODO */
    outputIds: string[];
}

/** Address with native tokens */
export interface SendNativeTokensParams {
    /** TODO */
    address: string;
    /** TODO */
    nativeTokens: [string, bigint][];
    /** TODO */
    returnAddress?: string;
    /** TODO */
    expiration?: number;
}

/** Address with an NftId */
export interface SendNftParams {
    /** TODO */
    address: string;
    /** TODO */
    nftId: string;
}

/** Options for address generation, useful with a Ledger Nano SecretManager */
export interface GenerateAddressOptions {
    /** TODO */
    internal: boolean;
    /** TODO */
    ledgerNanoPrompt: boolean;
}
