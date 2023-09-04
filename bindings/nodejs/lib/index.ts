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

export type Result = {
    // "ok" | "error" | "panic"
    type: string;
    payload: {
        // All method names from types/bridge/__name__.name
        // Or all variants of rust Error type
        type: string;
        // If "ok", json payload
        payload?: string;
        // If !"ok", error
        error?: string;
    };
};

function errorHandle(error: any): Error {
    if (error instanceof TypeError) {
        // neon or other bindings lib related error
        throw error;
    } else if (error instanceof Error) {
        // rust Err(Error)
        let err: Result = JSON.parse(error.message);
        if (err.type == 'panic') {
            return Error(err.payload.toString());
        } else {
            return Error(err.payload.error);
        }
    } else {
        // Something bad happened!
        return Error(error);
    }
}

export { errorHandle };
