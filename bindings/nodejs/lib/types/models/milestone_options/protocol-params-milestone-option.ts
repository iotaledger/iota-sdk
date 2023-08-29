// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { MilestoneOption, MilestoneOptionType } from './milestone-options';

/**
 * A Protocol Parameters Milestone Option.
 */
export class ProtocolParamsMilestoneOption extends MilestoneOption {
    /**
     * The milestone index at which these protocol parameters become active.
     */
    targetMilestoneIndex: number;
    /**
     * The to be applied protocol version.
     */
    protocolVersion: number;
    /**
     * The protocol parameters in binary form. Hex-encoded with 0x prefix.
     */
    params: string;

    /**
     * @param targetMilestoneIndex The milestone index at which these protocol parameters become active.
     * @param protocolVersion The to be applied protocol version.
     * @param params The protocol parameters in binary form. Hex-encoded with 0x prefix.
     */
    constructor(
        targetMilestoneIndex: number,
        protocolVersion: number,
        params: string,
    ) {
        super(MilestoneOptionType.Receipt);
        this.targetMilestoneIndex = targetMilestoneIndex;
        this.protocolVersion = protocolVersion;
        this.params = params;
    }
}
