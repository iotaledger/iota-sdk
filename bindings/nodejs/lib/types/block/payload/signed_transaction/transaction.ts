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
 * PayloadDiscriminator for payloads inside of a Transaction.
 */
const PayloadDiscriminator = {
    property: 'type',
    subTypes: [
        { value: TaggedDataPayload, name: PayloadType.TaggedData as any },
    ],
};

/**
 * A transaction.
 */
class Transaction {
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
        outputs: Output[],
        allotments: ManaAllotment[],
        payload?: Payload,
    ) {
        this.networkId = networkId;
        this.creationSlot = creationSlot;
        this.contextInputs = contextInputs;
        this.inputs = inputs;
        this.outputs = outputs;
        this.allotments = allotments;
        this.payload = payload;
    }
}

export { Transaction };
