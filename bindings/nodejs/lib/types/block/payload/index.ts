// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { plainToInstance } from 'class-transformer';
import { MilestonePayload } from './milestone';
import { Payload, PayloadType } from './payload';
import { TaggedDataPayload } from './tagged';
import { TransactionPayload } from './transaction';

export * from './milestone';
export * from './tagged';
export * from './transaction';
export * from './payload';

export const PayloadDiscriminator = {
    property: 'type',
    subTypes: [
        { value: MilestonePayload, name: PayloadType.Milestone as any },
        { value: TaggedDataPayload, name: PayloadType.TaggedData as any },
        { value: TransactionPayload, name: PayloadType.Transaction as any },
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
    }
    throw new Error('Invalid JSON');
}
