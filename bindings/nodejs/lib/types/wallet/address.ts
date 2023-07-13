// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/** An Address of the Account */
export interface AccountAddress {
    address: string;
    keyIndex: number;
    internal: boolean;
    used: boolean;
}

/** Address with a base token amount */
export interface SendParams {
    address: string;
    amount: bigint | string;
    returnAddress?: string;
    expiration?: number;
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
    nativeTokens: [string, bigint][];
    returnAddress?: string;
    expiration?: number;
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
