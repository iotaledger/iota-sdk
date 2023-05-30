// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { MilestoneOptionType } from './milestoneOptions';
import { ProtocolParamsMilestoneOption } from './ProtocolParamsMilestoneOption';
import { ReceiptMilestoneOption } from './ReceiptMilestoneOption';

export * from './ProtocolParamsMilestoneOption';
export * from './ReceiptMilestoneOption';
export * from './milestoneOptions';

export const MilestoneOptionDiscriminator = {
    property: 'type',
    subTypes: [
        {
            value: ReceiptMilestoneOption,
            name: MilestoneOptionType.Receipt as any,
        },
        {
            value: ProtocolParamsMilestoneOption,
            name: MilestoneOptionType.ProtocolParams as any,
        },
    ],
};