// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Bech32Address, EpochIndex } from '../../block';
import { u64 } from '../../utils';

/**
 * Returns information of a validator (committee member).
 */
export interface CommitteeMember {
    /**
     * Account address of the validator.
     */
    address: Bech32Address;
    /**
     * The total stake of the pool, including delegators.
     */
    pool_stake: u64;
    /**
     * The stake of a validator.
     */
    validator_stake: u64;
    /**
     * The fixed cost of the validator, which it receives as part of its Mana rewards.
     */
    fixed_cost: u64;
}

/**
 * Returns the validator information of the committee.
 */
export interface CommitteeResponse {
    /**
     * The epoch index of the committee.
     */
    epoch: EpochIndex;
    /**
     * The total amount of delegated and staked IOTA tokens in the selected committee.
     */
    totalStake: u64;
    /**
     * The total amount of staked IOTA tokens in the selected committee.
     */
    totalValidatorStake: u64;
    /**
     * The validators of the committee.
     */
    committee: CommitteeMember[];
}
