// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { HexEncodedString } from '../../utils';
import { OutputId } from '../output';

/**
 * All of the input types.
 */
enum InputType {
    UTXO = 0,
    Treasury = 1,
}

abstract class Input {
    readonly type: InputType;

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
 * Treasury Input.
 */
class TreasuryInput extends Input {
    /**
     * The milestone id of the input.
     */
    milestoneId: HexEncodedString;

    constructor(milestoneId: HexEncodedString) {
        super(InputType.Treasury);
        this.milestoneId = milestoneId;
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
        const source = outputId.startsWith("0x") ? outputId.substring(2) : outputId;
        const inputHexLe = source.slice(64);
        const chunks = [inputHexLe.substring(0, 2), inputHexLe.substring(2)];
        const separated = chunks.map(n => parseInt(n, 16))
        const buf = Uint8Array.from(separated).buffer;
        const view = new DataView(buf);

        const transactionId = source.substring(0, source.length - 4);
        const transactionOutputIndex = view.getUint16(0, true);

        return new UTXOInput(`0x${transactionId}`, transactionOutputIndex);
    }
}

const InputDiscriminator = {
    property: 'type',
    subTypes: [
        { value: TreasuryInput, name: InputType.Treasury as any },
        { value: UTXOInput, name: InputType.UTXO as any },
    ],
};

export { InputDiscriminator, InputType, Input, TreasuryInput, UTXOInput };
