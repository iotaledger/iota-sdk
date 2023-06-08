// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { MilestoneOptionType } from './milestone-options';
import { ProtocolParamsMilestoneOption } from './protocol-params-milestone-option';
import { ReceiptMilestoneOption } from './receipt-milestone-option';

export * from './protocol-params-milestone-option';
export * from './receipt-milestone-option';
export * from './milestone-options';

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
