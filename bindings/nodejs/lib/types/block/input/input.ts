// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { HexEncodedString } from '../../utils';

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
     * The input index.
     */
    transactionInputIndex: number;

    constructor(
        transactionId: HexEncodedString,
        transactionInputIndex: number,
    ) {
        super(InputType.UTXO);
        this.transactionId = transactionId;
        this.transactionInputIndex = transactionInputIndex;
    }
}

const InputDiscriminator = {
    property: 'type',
    subTypes: [{ value: UTXOInput, name: InputType.UTXO as any }],
};

export { InputDiscriminator, InputType, Input, UTXOInput };
