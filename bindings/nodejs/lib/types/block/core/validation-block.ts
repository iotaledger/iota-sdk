// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { StrongParents, WeakParents, ShallowLikeParents } from '../parents';
import { HexEncodedString } from '../../utils';
import { BlockWrapper } from './block';

export type ValidationBlock = BlockWrapper<ValidationBlockData>;

/**
 * A Validation Block is a special type of block used by validators to secure the network.
 * It is recognised by the Congestion Control of the IOTA 2.0 protocol and can be issued without
 * burning Mana within the constraints of the allowed validator throughput.
 *
 * It is allowed to reference more parent blocks than a normal Basic Block.
 * Validation Block uses the introduced Block ID as its identifier.
 */
export class ValidationBlockData {
    /**
     * Blocks that are strongly directly approved.
     */
    strongParents!: StrongParents;
    /**
     * Blocks that are weakly directly approved.
     */
    weakParents!: WeakParents;
    /**
     * Blocks that are directly referenced to adjust opinion.
     */
    shallowLikeParents!: ShallowLikeParents;

    /**
     * The highest supported protocol version the issuer of this block supports.
     */
    highestSupportedVersion!: number;

    /**
     * The hash of the protocol parameters for the Highest Supported Version.
     */
    protocolParametersHash!: HexEncodedString;
}
