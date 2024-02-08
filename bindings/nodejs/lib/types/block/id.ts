// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { HexEncodedString } from '../utils';
import { SlotIndex } from './slot';

/**
 * Base class for IDs with a hex encoded slot index at the end.
 */
export class IdWithSlotIndex extends String {
    slotIndex(): SlotIndex {
        const numberString = super.slice(-8);
        const chunks = [];
        for (
            let i = 0, charsLength = numberString.length;
            i < charsLength;
            i += 2
        ) {
            chunks.push(numberString.substring(i, i + 2));
        }
        const separated = chunks.map((n) => parseInt(n, 16));
        const buf = Uint8Array.from(separated).buffer;
        const view = new DataView(buf);
        return view.getUint32(0, true);
    }
}

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
export class BlockId extends IdWithSlotIndex {}

/**
 * A Token ID represented as hex-encoded string.
 */
export type TokenId = HexEncodedString;

/**
 * A Transaction ID represented as hex-encoded string.
 */
export class TransactionId extends IdWithSlotIndex {}

/**
 * A Foundry ID represented as hex-encoded string.
 */
export type FoundryId = HexEncodedString;

/**
 * Unique identifier of the Delegation Output, which is the BLAKE2b-256 hash of the Output ID that created it.
 */
export type DelegationId = HexEncodedString;
