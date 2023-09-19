// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Payload, PayloadDiscriminator } from '../payload';
import { Type } from 'class-transformer';
import { StrongParents, WeakParents, ShallowLikeParents } from '../parents';
import { Block } from './block';
import { u64 } from '../../utils';

/**
 * Basic Block layout.
 */
export class BasicBlock extends Block {
    /**
     * Blocks that are strongly directly approved.
     */
    readonly strongParents!: StrongParents;
    /**
     * Blocks that are weakly directly approved.
     */
    readonly weakParents!: WeakParents;
    /**
     * Blocks that are directly referenced to adjust opinion.
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
    readonly burnedMana!: u64;
}
