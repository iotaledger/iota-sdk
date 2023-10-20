// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Type } from 'class-transformer';
import { HexEncodedString } from '../../../utils';
import { ContextInput, ContextInputDiscriminator } from '../../context_input';
import { Input, InputDiscriminator } from '../../input';
import { ManaAllotment } from '../../mana-allotment';
import { Output, OutputDiscriminator } from '../../output';
import { SlotIndex } from '../../slot';
import { Payload, PayloadType } from '../payload';
import { TaggedDataPayload } from '../tagged/tagged';

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
    /**
     * The type of essence.
     */
    readonly type: TransactionEssenceType;

    /**
     * @param type The type of transaction essence.
     */
    constructor(type: TransactionEssenceType) {
        this.type = type;
    }
}

/**
 * PayloadDiscriminator for payloads inside of a TransactionEssence.
 */
const PayloadDiscriminator = {
    property: 'type',
    subTypes: [
        { value: TaggedDataPayload, name: PayloadType.TaggedData as any },
    ],
};

/**
 * RegularTransactionEssence transaction essence.
 */
class RegularTransactionEssence extends TransactionEssence {
    /**
     * The unique value denoting whether the block was meant for mainnet, testnet, or a private network.
     */
    readonly networkId: string;

    readonly creationSlot: SlotIndex;

    @Type(() => Input, {
        discriminator: ContextInputDiscriminator,
    })
    readonly contextInputs: ContextInput[];

    @Type(() => Input, {
        discriminator: InputDiscriminator,
    })
    readonly inputs: Input[];

    readonly inputsCommitment: HexEncodedString;

    @Type(() => Output, {
        discriminator: OutputDiscriminator,
    })
    readonly outputs: Output[];

    readonly allotments: ManaAllotment[];

    @Type(() => Payload, {
        discriminator: PayloadDiscriminator,
    })
    readonly payload?: Payload;

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
        creationSlot: SlotIndex,
        contextInputs: ContextInput[],
        inputs: Input[],
        inputsCommitment: HexEncodedString,
        outputs: Output[],
        allotments: ManaAllotment[],
        payload?: Payload,
    ) {
        super(TransactionEssenceType.Regular);
        this.networkId = networkId;
        this.creationSlot = creationSlot;
        this.contextInputs = contextInputs;
        this.inputs = inputs;
        this.inputsCommitment = inputsCommitment;
        this.outputs = outputs;
        this.allotments = allotments;
        this.payload = payload;
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
