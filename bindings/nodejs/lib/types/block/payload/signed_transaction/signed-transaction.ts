// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Type } from 'class-transformer';
import { Transaction, Unlock, UnlockDiscriminator } from '.';
import { Payload, PayloadType } from '../payload';

/**
 * Signed transaction payload.
 */
class SignedTransactionPayload extends Payload {
    /**
     * The transaction.
     */
    readonly transaction: Transaction;
    /**
     * The unlocks.
     */
    @Type(() => Unlock, {
        discriminator: UnlockDiscriminator,
    })
    readonly unlocks: Unlock[];

    /**
     * @param transaction The transaction.
     * @param unlocks The unlocks of the transaction.
     */
    constructor(transaction: Transaction, unlocks: Unlock[]) {
        super(PayloadType.SignedTransaction);
        this.transaction = transaction;
        this.unlocks = unlocks;
    }
}

export { SignedTransactionPayload };
