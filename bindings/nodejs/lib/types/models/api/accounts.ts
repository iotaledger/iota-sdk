// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SlotIndex } from '../../block/slot';
import { u64 } from '../../utils';

/**
 * Provides the cost and readiness to issue estimates.
 */
export class CongestionResponse {
    /**
     * Slot for which the estimate is provided.
     */
    slot!: SlotIndex;
    /**
     * Indicates if a node is ready to schedule a block issued by the specified account, or if the issuer should wait.
     */
    ready!: boolean;
    /**
     * Mana cost a user needs to burn to issue a block in the slot.
     */
    referenceManaCost!: u64;
    /**
     * BIC of the account in the slot. This balance needs to be non-negative, otherwise account is locked.
     */
    blockIssuanceCredits!: bigint;
}
