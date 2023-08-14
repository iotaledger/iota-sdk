// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * All of the milestone option types.
 */
enum MilestoneOptionType {
    /** The Receipt milestone option. */
    Receipt = 0,
    /** The ProtocolParams milestone option. */
    ProtocolParams = 1,
}

abstract class MilestoneOption {
    readonly type: MilestoneOptionType;

    /**
     * @param type The type of milestone option.
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
