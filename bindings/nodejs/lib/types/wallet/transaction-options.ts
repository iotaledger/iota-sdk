// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { TaggedDataPayload } from '../block/payload/tagged';
import { Burn } from '../client';
import { u256 } from '../utils';

/** Options for the transaction creation */
export interface TransactionOptions {
    remainderValueStrategy?: RemainderValueStrategy;
    taggedDataPayload?: TaggedDataPayload;
    /** Custom inputs that should be used for the transaction */
    customInputs?: string[];
    mandatoryInputs?: string[];
    burn?: Burn;
    /** Optional note, that is only stored locally */
    note?: string;
    allowMicroAmount: boolean;
}

/** The RemainderValueStrategy */
export type RemainderValueStrategy =
    | ChangeAddress
    | ReuseAddress
    | CustomAddress;

/** ChangeAddress variant of RemainderValueStrategy */
export type ChangeAddress = {
    strategy: 'ChangeAddress';
    value: null;
};

/** ReuseAddress variant of RemainderValueStrategy */
export type ReuseAddress = {
    strategy: 'ReuseAddress';
    value: null;
};

/** CustomAddress variant of RemainderValueStrategy */
export type CustomAddress = {
    strategy: 'CustomAddress';
    value: string;
};

/** Native token options for creating */
export interface CreateNativeTokenParams {
    accountId?: string;
    /** Hex encoded number */
    circulatingSupply: u256;
    /** Hex encoded number */
    maximumSupply: u256;
    /** Hex encoded bytes */
    foundryMetadata?: string;
}

/** Nft options for minting */
export interface MintNftParams {
    /** Bech32 encoded address to which the Nft will be minted. Default will use the
     * first address of the account
     */
    address?: string;
    /** Bech32 encoded sender address **/
    sender?: string;
    /** Hex encoded bytes */
    metadata?: string;
    /** Hex encoded bytes */
    tag?: string;
    /** Bech32 encoded issuer address **/
    issuer?: string;
    /** Hex encoded bytes */
    immutableMetadata?: string;
}

/** Options for the alias output creation */
export interface AccountOutputParams {
    /** Bech32 encoded address to which the Nft will be minted. Default will use the
     * first address of the account
     */
    address?: string;
    /** Hex encoded bytes */
    immutableMetadata?: string;
    /** Hex encoded bytes */
    metadata?: string;
    /** Hex encoded bytes */
    stateMetadata?: string;
}
