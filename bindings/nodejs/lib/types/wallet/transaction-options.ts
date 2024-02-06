// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AccountAddress, AccountId, Bech32Address } from '../block';
import { TaggedDataPayload } from '../block/payload/tagged';
import { Burn } from '../client';
import { u256, HexEncodedString, NumericString } from '../utils';
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
    allowMicroAmount?: boolean;
}

/** The possible remainder value strategies. */
export type RemainderValueStrategy = ReuseAddress | CustomAddress;

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
    accountId?: AccountId;
    /** Hex encoded number */
    circulatingSupply: u256;
    /** Hex encoded number */
    maximumSupply: u256;
    /** Hex encoded bytes */
    foundryMetadata?: HexEncodedString;
}

/** Options for minting NFTs. */
export interface MintNftParams {
    /** Bech32 encoded address to which the Nft will be minted. Default will use the
     * address of the wallet.
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

/** Options for the account output creation */
export interface AccountOutputParams {
    /** Bech32 encoded address to which the Nft will be minted. Default will use the
     * address of the wallet.
     */
    address?: Bech32Address;
    /** Hex encoded bytes */
    immutableMetadata?: HexEncodedString;
    /** Hex encoded bytes */
    metadata?: HexEncodedString;
}

/** Options for delegation output creation */
export interface CreateDelegationParams {
    /** Bech32 encoded address which will control the delegation. By default, the ed25519 wallet address will be used. */
    address?: Bech32Address;
    /** The amount to delegate. */
    delegatedAmount: NumericString;
    /** The Account Address of the validator to which this output will delegate. */
    validatorAddress: AccountAddress;
}

/** Options for beginning staking. */
export interface BeginStakingParams {
    /** The account id which will become a validator. */
    accountId: Bech32Address;
    /** The amount of tokens to stake. */
    stakedAmount: NumericString;
    /** The fixed cost of the validator, which it receives as part of its Mana rewards. */
    fixedCost: NumericString;
    /** The staking period (in epochs). Will default to the staking unbonding period. */
    stakingPeriod?: number;
}
