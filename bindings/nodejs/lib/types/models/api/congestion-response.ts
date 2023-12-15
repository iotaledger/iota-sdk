// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SlotIndex } from '../../block/slot';
import { u64 } from '../../utils';

/**
 * Provides the cost and readiness to issue estimates.
 */
export class CongestionResponse {
    /**
     * The slot index for which the congestion estimate is provided.
     */
    slot!: SlotIndex;
    /**
     * Indicates if a node is ready to issue a block in a current congestion or should wait.
     */
    ready!: boolean;
    /**
     * The cost in mana for issuing a block in a current congestion estimated based on RMC and slot index.
     */
    referenceManaCost!: u64;
    /**
     * The Block Issuance Credits of the requested account.
     */
    blockIssuanceCredits!: u64;
}
