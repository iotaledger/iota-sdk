// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AliasId, Bech32Address } from '../block';
import { TaggedDataPayload } from '../block/payload/tagged';
import { Burn } from '../client';
import { HexEncodedString } from '../utils';
import { AccountAddress } from './address';

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
    allowMicroAmount?: boolean;
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
    value: AccountAddress;
};

/** Options for creating Native Tokens. */
export interface CreateNativeTokenParams {
    /** The Alias ID of the corresponding Foundry. */
    aliasId?: AliasId;
    /** Hex encoded number */
    circulatingSupply: bigint;
    /** Hex encoded number */
    maximumSupply: bigint;
    /** Hex encoded bytes */
    foundryMetadata?: HexEncodedString;
}

/** Options for minting NFTs. */
export interface MintNftParams {
    /** Bech32 encoded address to which the Nft will be minted. Default will use the
     * first address of the account
     */
    address?: Bech32Address;
    /** Bech32 encoded sender address **/
    sender?: Bech32Address;
    /** Hex encoded bytes */
    metadata?: HexEncodedString;
    /** Hex encoded bytes */
    tag?: HexEncodedString;
    /** Bech32 encoded issuer address **/
    issuer?: Bech32Address;
    /** Hex encoded bytes */
    immutableMetadata?: HexEncodedString;
}

/** Options for the alias output creation */
export interface AliasOutputParams {
    /** Bech32 encoded address to which the Nft will be minted. Default will use the
     * first address of the account
     */
    address?: Bech32Address;
    /** Hex encoded bytes */
    immutableMetadata?: HexEncodedString;
    /** Hex encoded bytes */
    metadata?: HexEncodedString;
    /** Hex encoded bytes */
    stateMetadata?: HexEncodedString;
}
