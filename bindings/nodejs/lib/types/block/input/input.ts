// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { callUtilsMethod } from '../../../bindings';
import { HexEncodedString } from '../../utils';
import { OutputId } from '../output';

/**
 * All of the input types.
 */
enum InputType {
    UTXO = 0,
}

abstract class Input {
    private type: InputType;

    constructor(type: InputType) {
        this.type = type;
    }

    /**
     * The type of input.
     */
    getType(): InputType {
        return this.type;
    }
}

/**
 * UTXO Transaction Input.
 */
class UTXOInput extends Input {
    /**
     * The transaction Id.
     */
    transactionId: HexEncodedString;
    /**
     * The output index.
     */
    transactionOutputIndex: number;

    constructor(
        transactionId: HexEncodedString,
        transactionOutputIndex: number,
    ) {
        super(InputType.UTXO);
        this.transactionId = transactionId;
        this.transactionOutputIndex = transactionOutputIndex;
    }

    /**
     * Creates a `UTXOInput` from an output id.
     */
    static fromOutputId(outputId: OutputId): UTXOInput {
        const input = callUtilsMethod({
            name: 'outputIdToUtxoInput',
            data: {
                outputId,
            },
        });
        return new UTXOInput(input.transactionId, input.transactionOutputIndex);
    }
}

const InputDiscriminator = {
    property: 'type',
    subTypes: [{ value: UTXOInput, name: InputType.UTXO as any }],
};

export { InputDiscriminator, InputType, Input, UTXOInput };
