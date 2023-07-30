// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Type } from 'class-transformer';
import { TreasuryTransactionPayload } from '../../block/payload/treasury/treasury';
import { MigratedFunds } from '../migrated-funds';
import { MilestoneOption, MilestoneOptionType } from './milestone-options';

/**
 * A Receipt milestone option.
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
     * The funds which were migrated.
     */
    @Type(() => MigratedFunds)
    funds: MigratedFunds[];
    /**
     * The Treasury Transaction used to provide the funds.
     */
    @Type(() => TreasuryTransactionPayload)
    transaction: TreasuryTransactionPayload;

    /**
     * @param migratedAt The milestone index at which the funds were migrated in the legacy network.
     * @param final Whether this Receipt is the final one for a given migrated at index.
     * @param funds The funds which were migrated.
     * @param transaction The Treasury Transaction used to provide the funds.
     */
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
