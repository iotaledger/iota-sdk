// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { HexEncodedString } from '../utils';

/**
 * An Account ID represented as hex-encoded string.
 */
export type AccountId = HexEncodedString;

/**
 * An Anchor ID represented as hex-encoded string.
 */
export type AnchorId = HexEncodedString;

/**
 * An NFT ID represented as hex-encoded string.
 */
export type NftId = HexEncodedString;

/**
 * A Block ID represented as hex-encoded string.
 */
export type BlockId = HexEncodedString;

/**
 * A Token ID represented as hex-encoded string.
 */
export type TokenId = HexEncodedString;

/**
 * A Transaction ID represented as hex-encoded string.
 */
export type TransactionId = HexEncodedString;

/**
 * A Foundry ID represented as hex-encoded string.
 */
export type FoundryId = HexEncodedString;

/**
 * Unique identifier of the Delegation Output, which is the BLAKE2b-256 hash of the Output ID that created it.
 */
export type DelegationId = HexEncodedString;
