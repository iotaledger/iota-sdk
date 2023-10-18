// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { plainToInstance } from 'class-transformer';
import { Payload, PayloadType } from './payload';
import { TaggedDataPayload } from './tagged';
import { TransactionPayload } from './transaction';
import { CandidacyAnnouncementPayload } from './candidacy_announcement';

export * from './tagged';
export * from './transaction';
export * from './payload';
export * from './candidacy_announcement';

export const PayloadDiscriminator = {
    property: 'type',
    subTypes: [
        { value: TaggedDataPayload, name: PayloadType.TaggedData as any },
        { value: TransactionPayload, name: PayloadType.Transaction as any },
        {
            value: CandidacyAnnouncementPayload,
            name: PayloadType.CandidacyAnnouncement as any,
        },
    ],
};

export function parsePayload(data: any): Payload {
    if (data.type == PayloadType.TaggedData) {
        return plainToInstance(
            TaggedDataPayload,
            data,
        ) as any as TaggedDataPayload;
    } else if (data.type == PayloadType.Transaction) {
        return plainToInstance(
            TransactionPayload,
            data,
        ) as any as TransactionPayload;
    } else if (data.type == PayloadType.CandidacyAnnouncement) {
        return plainToInstance(
            CandidacyAnnouncementPayload,
            data,
        ) as any as CandidacyAnnouncementPayload;
    }
    throw new Error('Invalid JSON');
}
