// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { ReceiptMilestoneOption } from '../milestoneOptions';
/**
 * Receipts response details.
 */
export interface IReceiptsResponse {
    /**
     * The receipts.
     */
    receipts: {
        /**
         * The milestone index.
         */
        milestoneIndex: number;
        /**
         * The receipt.
         */
        receipt: ReceiptMilestoneOption;
    }[];
}
