// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { plainToInstance } from 'class-transformer';
import { Payload, PayloadType } from './payload';
import { TaggedDataPayload } from './tagged';
import { SignedTransactionPayload } from './signed_transaction';
import { CandidacyAnnouncementPayload } from './candidacy_announcement';

export * from './tagged';
export * from './signed_transaction';
export * from './payload';
export * from './candidacy_announcement';

export const PayloadDiscriminator = {
    property: 'type',
    subTypes: [
        { value: TaggedDataPayload, name: PayloadType.TaggedData as any },
        {
            value: SignedTransactionPayload,
            name: PayloadType.SignedTransaction as any,
        },
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
    } else if (data.type == PayloadType.SignedTransaction) {
        return plainToInstance(
            SignedTransactionPayload,
            data,
        ) as any as SignedTransactionPayload;
    } else if (data.type == PayloadType.CandidacyAnnouncement) {
        return plainToInstance(
            CandidacyAnnouncementPayload,
            data,
        ) as any as CandidacyAnnouncementPayload;
    }
    throw new Error('Invalid JSON');
}
