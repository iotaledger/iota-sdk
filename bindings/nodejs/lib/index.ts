// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// Needed for class-transformer json deserialisation
import 'reflect-metadata';
import { callUtilsMethod } from './bindings';
import {
    BlockError,
    ClientError,
    ClientErrorName,
    OutputId,
    PrefixHexError,
    SerdeJsonError,
    UnpackError,
    UTXOInput,
    WalletError,
    WalletErrorName,
} from './types';
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

// For future reference to see what we return from rust as a serialized string
export type Result = OkResult | NotOkResult;

export type OkResult = {
    // "Response" enum name, we consider "ok".
    type: string;
    // The object below.
    payload: {
        // Ok: All method names from types/bridge/__name__.name
        type: string;
        // json payload
        payload: string;
    };
};

export type NotOkResult = PanicResult | ErrorResult;

export type ErrorResult = {
    type: 'error';
    payload: {
        // All variants of iota_sdk_bindings_core::Error type i.e block/client/wallet.
        // client and wallet have a full object, the others a string.
        type: string;
        // json error
        error:
            | {
                  // all variants of each sub-error type (i.e. healthyNodePoolEmpty )
                  type: string;
                  // Error message from the enum.to_string() generation
                  error: string;
              }
            | string;
    };
};

export type PanicResult = {
    type: 'panic';
    payload: string;
};

function errorHandle(error: any): Error {
    try {
        let err: Result = JSON.parse(error.message);
        if (!err.type || !(err.type == 'panic' || err.type == 'error')) {
            return error;
        }

        // Guaranteed error
        err = err as NotOkResult;
        if (err.type == 'panic') {
            // Panic example:
            // {"type":"panic","payload":"Client was destroyed"}
            return Error((err as PanicResult).payload);
        } else if (err.type == 'error') {
            err = err as ErrorResult;
            if (typeof err.payload.error === 'string') {
                // Error example:
                // {"type":"error","payload":{"type":"block","error":"too many commitment inputs"}}

                switch (err.payload.type) {
                    case 'block':
                        return new BlockError(err.payload.error);
                    case 'prefixHex':
                        return new PrefixHexError(err.payload.error);
                    case 'serdeJson':
                        return new SerdeJsonError(err.payload.error);
                    case 'unpack':
                        return new UnpackError(err.payload.error);
                }
            } else {
                // Error example:
                // {"type":"error","payload":{"type":"client","error":{"error":"no healthy node available","type":"healthyNodePoolEmpty"}}}

                switch (err.payload.type) {
                    case 'client':
                        return new ClientError({
                            name: err.payload.error.type as ClientErrorName,
                            message: err.payload.error.error,
                        });
                    case 'wallet':
                        return new WalletError({
                            name: err.payload.error.type as WalletErrorName,
                            message: err.payload.error.error,
                        });
                }
            }
        }
        return Error(
            'in ErrorHandle without a valid error object. Only call this in catch statements.',
        );
    } catch (err: any) {
        // json error, SyntaxError, we must have send a non-json error
        return error;
    }
}

export { errorHandle };
