// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Payload, PayloadDiscriminator } from '../payload';
import { Type } from 'class-transformer';
import { StrongParents, WeakParents, ShallowLikeParents } from '../parents';
import { BlockWrapper } from './block_wrapper';

/**
 * Basic Block layout.
 */
export class BasicBlock extends BlockWrapper {
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
     * The payload contents.
     */
    @Type(() => Payload, {
        discriminator: PayloadDiscriminator,
    })
    payload?: Payload;
    /**
     * The amount of mana the Account identified by IssuerID is at most
     * willing to burn for this block.
     */
    burnedMana!: string;
}
