// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Type } from 'class-transformer';
import { PayloadDiscriminator } from '..';
import { HexEncodedString } from '../../../utils';
import { Input, InputDiscriminator } from '../../input';
import { Output, OutputDiscriminator } from '../../output';
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

    /**
     * The type of essence.
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
    readonly networkId: number;
    readonly inputsCommitment: HexEncodedString;

    @Type(() => Input, {
        discriminator: InputDiscriminator,
    })
    readonly inputs: [Input];

    @Type(() => Output, {
        discriminator: OutputDiscriminator,
    })
    readonly outputs: [Output];

    @Type(() => Payload, {
        discriminator: PayloadDiscriminator,
    })
    readonly payload: Payload | undefined;

    constructor(
        networkId: number,
        inputsCommitment: HexEncodedString,
        inputs: [Input],
        outputs: [Output],
        payload: Payload | undefined,
    ) {
        super(TransactionEssenceType.Regular);
        this.networkId = networkId;
        this.inputsCommitment = inputsCommitment;
        this.inputs = inputs;
        this.outputs = outputs;
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
