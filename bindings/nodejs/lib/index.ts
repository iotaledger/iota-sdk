// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// Needed for class-transformer json deserialisation
import 'reflect-metadata';
import { callUtilsMethod } from './bindings';
import { OutputId, UTXOInput } from './types';
import { bigIntToHex } from './utils';

// Allow bigint to be serialized as hex string.
//
// Note:
// Serializing `bigint` to a different format, e.g. to decimal number string
// must be done manually.
(BigInt.prototype as any).toJSON = function () {
    return bigIntToHex(this);
};

// Assign the util method on UTXOInput here,
// to prevent loading bindings (callUtilsMethod) when importing UTXOInput just for typing.
Object.assign(UTXOInput, {
    /**
     * Creates a `UTXOInput` from an output id.
     */
    fromOutputId(outputId: OutputId): UTXOInput {
        const input = callUtilsMethod({
            name: 'outputIdToUtxoInput',
            data: {
                outputId,
            },
        });
        return new UTXOInput(input.transactionId, input.transactionOutputIndex);
    },
});

export * from './client';
export * from './secret_manager';
export * from './types';
export * from './utils';
export * from './wallet';
export * from './logger';
