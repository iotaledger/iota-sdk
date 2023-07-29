// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * All of the milestone option types.
 */
enum MilestoneOptionType {
    Receipt = 0,
    ProtocolParams = 1,
}

abstract class MilestoneOption {
    readonly type: MilestoneOptionType;

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

export { MilestoneOptionType, MilestoneOption };
