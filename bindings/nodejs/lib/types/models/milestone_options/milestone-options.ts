// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * All of the milestone option types.
 */
enum MilestoneOptionType {
    /** TODO. */
    Receipt = 0,
    /** TODO. */
    ProtocolParams = 1,
}

abstract class MilestoneOption {
    readonly type: MilestoneOptionType;

    /**
     * TODO.
     */
    constructor(type: MilestoneOptionType) {
        this.type = type;
    }

    /**
     * Get the type of milestone option.
     */
    getType(): MilestoneOptionType {
        return this.type;
    }
}

export { MilestoneOptionType, MilestoneOption };
