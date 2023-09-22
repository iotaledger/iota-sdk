// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Type } from 'class-transformer';
import { parsePayload } from '../../../..';
import { HexEncodedString } from '../../../utils';
import { Input, InputDiscriminator } from '../../input';
import { Output, OutputDiscriminator } from '../../output';
import { MilestonePayload } from '../milestone/milestone';
import { Payload } from '../payload';
import { TaggedDataPayload } from '../tagged/tagged';
import { TreasuryTransactionPayload } from '../treasury';
import { TransactionPayload } from './transaction';

/**
 * All of the essence types.
 */
enum TransactionEssenceType {
    /**
     * A regular transaction essence.
     */
    Regular = 1,
}

/**
 * The base class for transaction essences.
 */
abstract class TransactionEssence {
    readonly type: TransactionEssenceType;

    /**
     * @param type The type of transaction essence.
     */
    constructor(type: TransactionEssenceType) {
        this.type = type;
    }

    /**
     * Get the type of essence.
     */
    getType(): TransactionEssenceType {
        return this.type;
    }
}

/**
 * RegularTransactionEssence transaction essence.
 */
class RegularTransactionEssence extends TransactionEssence {
    /// The unique value denoting whether the block was meant for mainnet, testnet, or a private network.
    networkId: string;
    inputsCommitment: HexEncodedString;

    @Type(() => Input, {
        discriminator: InputDiscriminator,
    })
    inputs: Input[];

    @Type(() => Output, {
        discriminator: OutputDiscriminator,
    })
    outputs: Output[];

    payload?: unknown;

    /**
     * @param networkId The ID of the network the transaction was issued to.
     * @param inputsCommitment The hash of all inputs.
     * @param inputs The inputs of the transaction.
     * @param outputs The outputs of the transaction.
     * @param payload An optional Tagged Data payload.
     *
     */
    constructor(
        networkId: string,
        inputsCommitment: HexEncodedString,
        inputs: Input[],
        outputs: Output[],
        payload: Payload | undefined,
    ) {
        super(TransactionEssenceType.Regular);
        this.networkId = networkId;
        this.inputsCommitment = inputsCommitment;
        this.inputs = inputs;
        this.outputs = outputs;
        this.payload = payload;
    }

    getParsedPayload():
        | TransactionPayload
        | MilestonePayload
        | TaggedDataPayload
        | TreasuryTransactionPayload
        | undefined {
        return this.payload ? (parsePayload(this.payload) as any) : undefined;
    }
}

const TransactionEssenceDiscriminator = {
    property: 'type',
    subTypes: [
        {
            value: RegularTransactionEssence,
            name: TransactionEssenceType.Regular as any,
        },
    ],
};

export {
    TransactionEssenceDiscriminator,
    TransactionEssence,
    TransactionEssenceType,
    RegularTransactionEssence,
};
