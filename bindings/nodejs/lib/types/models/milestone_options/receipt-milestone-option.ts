// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Type } from 'class-transformer';
import { TreasuryTransactionPayload } from '../../block/payload/treasury/treasury';
import { MigratedFunds } from '../migrated-funds';
import { MilestoneOption, MilestoneOptionType } from './milestone-options';

/**
 * Receipt milestone option.
 */
export class ReceiptMilestoneOption extends MilestoneOption {
    /**
     * The milestone index at which the funds were migrated in the legacy network.
     */
    migratedAt: number;
    /**
     * Whether this Receipt is the final one for a given migrated at index.
     */
    final: boolean;
    /**
     * The index data.
     */
    @Type(() => MigratedFunds)
    funds: MigratedFunds[];
    /**
     * The TreasuryTransaction used to fund the funds.
     */
    @Type(() => TreasuryTransactionPayload)
    transaction: TreasuryTransactionPayload;

    constructor(
        migratedAt: number,
        final: boolean,
        funds: MigratedFunds[],
        transaction: TreasuryTransactionPayload,
    ) {
        super(MilestoneOptionType.Receipt);
        this.migratedAt = migratedAt;
        this.final = final;
        this.funds = funds;
        this.transaction = transaction;
    }
}
