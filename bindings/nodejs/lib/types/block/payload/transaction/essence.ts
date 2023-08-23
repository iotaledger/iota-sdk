// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Type } from 'class-transformer';
import { PayloadDiscriminator } from '..';
import { HexEncodedString, u64 } from '../../../utils';
import { ContextInput, ContextInputDiscriminator } from '../../context_input';
import { Input, InputDiscriminator } from '../../input';
import { ManaAllotment } from '../../mana-allotment';
import { Output, OutputDiscriminator } from '../../output';
import { SlotIndex } from '../../slot';
import { Payload } from '../payload';

/**
 * All of the essence types.
 */
enum TransactionEssenceType {
    Regular = 1,
}

abstract class TransactionEssence {
    readonly type: TransactionEssenceType;

    constructor(type: TransactionEssenceType) {
        this.type = type;
    }
}

/**
 * RegularTransactionEssence transaction essence.
 */
class RegularTransactionEssence extends TransactionEssence {
    /**
     * The unique value denoting whether the block was meant for mainnet, testnet, or a private network.
     */
    networkId: u64;

    @Type(() => Input, {
        discriminator: ContextInputDiscriminator,
    })
    contextInput: ContextInput[];

    @Type(() => Input, {
        discriminator: InputDiscriminator,
    })
    inputs: Input[];

    inputsCommitment: HexEncodedString;

    @Type(() => Output, {
        discriminator: OutputDiscriminator,
    })
    outputs: Output[];

    allotments: ManaAllotment[];

    @Type(() => Payload, {
        discriminator: PayloadDiscriminator,
    })
    payload?: Payload;

    creationSlot?: SlotIndex;

    constructor(
        networkId: u64,
        contextInput: ContextInput[],
        inputs: Input[],
        inputsCommitment: HexEncodedString,
        outputs: Output[],
        allotments: ManaAllotment[],
        payload?: Payload,
        creationSlot?: SlotIndex,
    ) {
        super(TransactionEssenceType.Regular);
        this.networkId = networkId;
        this.contextInput = contextInput;
        this.inputs = inputs;
        this.inputsCommitment = inputsCommitment;
        this.outputs = outputs;
        this.allotments = allotments;
        this.payload = payload;
        this.creationSlot = creationSlot;
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
