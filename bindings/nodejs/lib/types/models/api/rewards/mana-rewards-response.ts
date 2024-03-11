// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { EpochIndex } from '../../../block/slot';
import { u64 } from '../../../utils';

/**
 * The mana rewards of an account or delegation output.
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
    /**
     * The rewards of the latest committed epoch of the staking pool to which this validator or delegator belongs.
     * The ratio of this value and the maximally possible rewards for the latest committed epoch can be used to
     * determine how well the validator of this staking pool performed in that epoch. Note that if the pool was not
     * part of the committee in the latest committed epoch, this value is 0.
     */
    latestCommittedEpochPoolRewards!: u64;
}
