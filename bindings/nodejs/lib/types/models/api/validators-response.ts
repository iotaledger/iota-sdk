// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Bech32Address, EpochIndex } from '../../block';
import { u64 } from '../../utils';
import type { HexEncodedString } from '../../utils/hex-encoding';

/**
 * Information of a validator.
 */
export interface ValidatorResponse {
    /**
     * Account address of the validator.
     */
    address: Bech32Address;
    /**
     * The epoch index until which the validator registered to stake.
     */
    stakingEndEpoch: EpochIndex;
    /**
     * The total stake of the pool, including delegators.
     */
    poolStake: u64;
    /**
     * The stake of a validator.
     */
    validatorStake: u64;
    /**
     * The fixed cost of the validator, which it receives as part of its Mana rewards.
     */
    fixedCost: u64;
    /**
     * Shows whether the validator was active recently.
     */
    active: boolean;
    /**
     * The latest protocol version the validator supported.
     */
    latestSupportedProtocolVersion: number;
    /**
     * The latest protocol version the validator supported.
     */
    latestSupportedProtocolHash: HexEncodedString;
}

/**
 * A paginated list of all registered validators ready for the next epoch and indicates if they were active recently
 * (are eligible for committee selection).
 */
export interface ValidatorsResponse {
    /**
     * List of registered validators ready for the next epoch.
     */
    validators: ValidatorResponse[];
    /**
     * The number of validators returned per one API request with pagination.
     */
    pageSize: number;
    /**
     * The cursor that needs to be provided as cursor query parameter to request the next page. If empty, this was the
     * last page.
     */
    cursor?: string;
}
