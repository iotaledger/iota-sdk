// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { StrongParents, WeakParents, ShallowLikeParents } from '../parents';
import { HexEncodedString } from '../../utils';
import { BlockBody } from './block-body';

/**
 * A Validation Block Body is a special type of block body used by validators to secure the network.
 * It is recognised by the Congestion Control of the IOTA 2.0 protocol and can be issued without
 * burning Mana within the constraints of the allowed validator throughput.
 *
 * It is allowed to reference more parent blocks than a normal Basic Block Body.
 */
export class ValidationBlockBody extends BlockBody {
    /**
     * Blocks that are strongly directly approved.
     */
    readonly strongParents!: StrongParents;
    /**
     * Blocks that are weakly directly approved.
     */
    readonly weakParents?: WeakParents;
    /**
     * Blocks that are directly referenced to adjust opinion.
     */
    readonly shallowLikeParents?: ShallowLikeParents;

    /**
     * The highest supported protocol version the issuer of this block supports.
     */
    readonly highestSupportedVersion!: number;

    /**
     * The hash of the protocol parameters for the Highest Supported Version.
     */
    readonly protocolParametersHash!: HexEncodedString;
}
