// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Type } from 'class-transformer';
import {
    TransactionEssence,
    TransactionEssenceDiscriminator,
    Unlock,
    UnlockDiscriminator,
} from '.';
import { Payload, PayloadType } from '../payload';

/**
 * Transaction payload.
 */
class TransactionPayload extends Payload {
    /**
     * The index name.
     */
    @Type(() => TransactionEssence, {
        discriminator: TransactionEssenceDiscriminator,
    })
    essence: TransactionEssence;
    /**
     * The unlocks.
     */
    @Type(() => Unlock, {
        discriminator: UnlockDiscriminator,
    })
    unlocks: Unlock[];

    /**
     * @param essence The transaction essence.
     * @param unlocks The unlocks of the transaction.
     */
    constructor(essence: TransactionEssence, unlocks: Unlock[]) {
        super(PayloadType.Transaction);
        this.essence = essence;
        this.unlocks = unlocks;
    }
}

export { TransactionPayload };
