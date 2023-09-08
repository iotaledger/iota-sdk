// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { TaggedDataPayload } from '../block/payload/tagged';
import { Burn } from '../client';
import { u256 } from '../utils';
import { Bip44Address } from './address';

/** Options for creating a transaction. */
export interface TransactionOptions {
    /** The strategy applied for base coin remainders. */
    remainderValueStrategy?: RemainderValueStrategy;
    /** An optional tagged data payload. */
    taggedDataPayload?: TaggedDataPayload;
    /**
     * Custom inputs that should be used for the transaction.
     * If custom inputs are provided, only those are used.
     * If also other additional inputs should be used, `mandatoryInputs` should be used instead.
     */
    customInputs?: string[];
    /** Inputs that must be used for the transaction. */
    mandatoryInputs?: string[];
    /** Specifies what needs to be burned during input selection. */
    burn?: Burn;
    /** Optional note, that is only stored locally. */
    note?: string;
    /** Whether to allow sending a micro amount. */
    allowMicroAmount: boolean;
}

/** The possible remainder value strategies. */
export type RemainderValueStrategy =
    | ChangeAddress
    | ReuseAddress
    | CustomAddress;

/**
 * Allows to move the remainder value to a change address.
 */
export type ChangeAddress = {
    /** The name of the strategy. */
    strategy: 'ChangeAddress';
    /** Only required for `CustomAddress`. */
    value: null;
};

/**
 * Allows to keep the remainder value on the source address.
 */
export type ReuseAddress = {
    /** The name of the strategy. */
    strategy: 'ReuseAddress';
    /** Only required for `CustomAddress`. */
    value: null;
};

/** CustomAddress variant of RemainderValueStrategy */
export type CustomAddress = {
    /** The name of the strategy. */
    strategy: 'CustomAddress';
    value: Bip44Address;
};

/** Options for creating Native Tokens. */
export interface CreateNativeTokenParams {
    /** The account ID of the corresponding Foundry. */
    accountId?: string;
    /** Hex encoded number */
    circulatingSupply: u256;
    /** Hex encoded number */
    maximumSupply: u256;
    /** Hex encoded bytes */
    foundryMetadata?: string;
}

/** Options for minting NFTs. */
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

/** Options for the account output creation */
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
