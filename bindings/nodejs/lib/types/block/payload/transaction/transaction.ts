// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { TransactionEssence, Unlock } from '.';
import { Payload, PayloadType } from '../payload';

/**
 * Transaction payload.
 */
class TransactionPayload extends Payload {
    /**
     * The index name.
     */
    essence: TransactionEssence;
    /**
     * The unlocks.
     */
    unlocks: Unlock[];

    constructor(essence: TransactionEssence, unlocks: Unlock[]) {
        super(PayloadType.Transaction);
        this.essence = essence;
        this.unlocks = unlocks;
    }
}

export { TransactionPayload };
