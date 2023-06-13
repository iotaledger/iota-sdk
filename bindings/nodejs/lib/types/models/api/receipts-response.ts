// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Type } from 'class-transformer';
import { ReceiptMilestoneOption } from '../milestone_options';
/**
 * Receipts response details.
 */
export class ReceiptsResponse {
    /**
     * The receipts.
     */
    @Type(() => MilestoneReceipt)
    receipts!: MilestoneReceipt[];
}

export class MilestoneReceipt {
    /**
     * The milestone index.
     */
    milestoneIndex!: number;
    /**
     * The receipt.
     */
    @Type(() => ReceiptMilestoneOption)
    receipt!: ReceiptMilestoneOption;
}
