// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { HexEncodedString } from '../../utils';
import { OutputId } from '../output';

/**
 * All of the transaction input types.
 */
enum InputType {
    /** A UTXO input. */
    UTXO = 0,
}

/**
 * The base class for transaction inputs.
 */
abstract class Input {
    readonly type: InputType;

    /**
     * @param type The type of input.
     */
    constructor(type: InputType) {
        this.type = type;
    }

    /**
     * Get the type of input.
     */
    getType(): InputType {
        return this.type;
    }
}

/**
 * A UTXO transaction input.
 */
class UTXOInput extends Input {
    /**
     * The transaction ID.
     */
    transactionId: HexEncodedString;
    /**
     * The output index.
     */
    transactionOutputIndex: number;

    /**
     * @param transactionId The ID of the transaction it is an input of.
     * @param transactionOutputIndex The index of the input within the transaction.
     */
    constructor(
        transactionId: HexEncodedString,
        transactionOutputIndex: number,
    ) {
        super(InputType.UTXO);
        this.transactionId = transactionId;
        this.transactionOutputIndex = transactionOutputIndex;
    }

    /**
     * Create a `UTXOInput` from a given output ID.
     */
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    static fromOutputId(outputId: OutputId): UTXOInput {
        // Implementation injected in lib/index.ts, as it uses bindings.
        return null as unknown as UTXOInput;
    }
}

const InputDiscriminator = {
    property: 'type',
    subTypes: [{ value: UTXOInput, name: InputType.UTXO as any }],
};

export { InputDiscriminator, InputType, Input, UTXOInput };
