// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Payload, PayloadDiscriminator } from '../payload';
import { Type } from 'class-transformer';
import { StrongParents, WeakParents, ShallowLikeParents } from '../parents';
import { Block } from './block';

/**
 * Basic Block layout.
 */
export class BasicBlock extends Block {
    /**
     * Blocks that are strongly directly approved, in the range of 1..8.
     */
    readonly strongParents!: StrongParents;
    /**
     * Blocks that are weakly directly approved, in the range of 0..8.
     */
    readonly weakParents!: WeakParents;
    /**
     * Blocks that are directly referenced to adjust opinion, in the range of 0..8.
     */
    readonly shallowLikeParents!: ShallowLikeParents;
    /**
     * The payload contents.
     */
    @Type(() => Payload, {
        discriminator: PayloadDiscriminator,
    })
    readonly payload?: Payload;
    /**
     * The amount of mana the Account identified by IssuerID is at most
     * willing to burn for this block.
     */
    readonly burnedMana!: string;
}
