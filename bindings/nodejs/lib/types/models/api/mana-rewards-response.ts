// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { EpochIndex } from '../../block/slot';
import { u64 } from '../../utils';

/**
 * Returns the mana rewards of an account or delegation output.
 */
export class ManaRewardsResponse {
    /**
     * The starting epoch index for which the mana rewards are returned.
     */
    startEpoch!: EpochIndex;
    /**
     * The ending epoch index for which the mana rewards are returned, the decay is applied up to this point
     * included.
     */
    endEpoch!: EpochIndex;
    /**
     * The amount of totally available rewards the requested output may claim.
     */
    rewards!: u64;
}
