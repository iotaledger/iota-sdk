// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { plainToInstance } from 'class-transformer';
import {
    MilestonePayload,
    TaggedDataPayload,
    TransactionPayload,
    TreasuryTransactionPayload,
} from './internal';

/**
 * All of the block payload types.
 */
enum PayloadType {
    /** A milestone payload. */
    Milestone = 7,
    /** A tagged data payload. */
    TaggedData = 5,
    /** A transaction payload. */
    Transaction = 6,
    /** A treasury transaction payload. */
    TreasuryTransaction = 4,
}

/**
 * The base class for block payloads.
 */
abstract class Payload {
    readonly type: PayloadType;

    /**
     * @param type The type of payload.
     */
    constructor(type: PayloadType) {
        this.type = type;
    }

    /**
     * Get the type of payload.
     */
    getType(): PayloadType {
        return this.type;
    }
}

export const PayloadDiscriminator = {
    property: 'type',
    subTypes: [
        { value: MilestonePayload, name: PayloadType.Milestone as any },
        { value: TaggedDataPayload, name: PayloadType.TaggedData as any },
        { value: TransactionPayload, name: PayloadType.Transaction as any },
        {
            value: TreasuryTransactionPayload,
            name: PayloadType.TreasuryTransaction as any,
        },
    ],
};

export function parsePayload(data: any): Payload {
    if (data.type == PayloadType.Milestone) {
        return plainToInstance(
            MilestonePayload,
            data,
        ) as any as MilestonePayload;
    } else if (data.type == PayloadType.TaggedData) {
        return plainToInstance(
            TaggedDataPayload,
            data,
        ) as any as TaggedDataPayload;
    } else if (data.type == PayloadType.Transaction) {
        return plainToInstance(
            TransactionPayload,
            data,
        ) as any as TransactionPayload;
    } else if (data.type == PayloadType.TreasuryTransaction) {
        return plainToInstance(
            TreasuryTransactionPayload,
            data,
        ) as any as TreasuryTransactionPayload;
    }
    throw new Error('Invalid JSON');
}

export { PayloadType, Payload };
