// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { ProtocolParamsMilestoneOption } from './ProtocolParamsMilestoneOption';
import { ReceiptMilestoneOption } from './ReceiptMilestoneOption';

/**
 * All of the milestone option types.
 */
enum MilestoneOptionType {
    Receipt = 0,
    ProtocolParams = 1,
}

abstract class MilestoneOption {
    private type: MilestoneOptionType;

    constructor(type: MilestoneOptionType) {
        this.type = type;
    }

    /**
     * The type of milestone option.
     */
    getType(): MilestoneOptionType {
        return this.type;
    }
}

const MilestoneOptionDiscriminator = {
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

export { MilestoneOptionType, MilestoneOption, MilestoneOptionDiscriminator };
