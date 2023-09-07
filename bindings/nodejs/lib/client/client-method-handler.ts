// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { errorHandle } from '..';
import {
    callClientMethodAsync,
    createClient,
    listenMqtt,
    destroyClient,
} from '../bindings';
import type { IClientOptions, __ClientMethods__ } from '../types/client';

/**
 * The MethodHandler which sends the commands to the Rust side.
 */
export class ClientMethodHandler {
    methodHandler: ClientMethodHandler;

    /**
     * @param options client options or a client method handler.
     */
    constructor(options: IClientOptions | ClientMethodHandler) {
        try {
            // The rust client object is not extensible
            if (Object.isExtensible(options)) {
                this.methodHandler = createClient(JSON.stringify(options));
            } else {
                this.methodHandler = options as ClientMethodHandler;
            }
        } catch (error: any) {
            throw errorHandle(error);
        }
    }

    destroy(): void {
        try {
            destroyClient(this.methodHandler);
        } catch (error: any) {
            throw errorHandle(error);
        }
    }

    /**
     * Call a client method.
     *
     * @param method The client method.
     * @returns A promise that resolves to a JSON string response holding the result of the client method.
     */
    async callMethod(method: __ClientMethods__): Promise<string> {
        return callClientMethodAsync(
            // mapToObject is required to convert maps to array since they otherwise get serialized as `[{}]` even if not empty
            JSON.stringify(method, function mapToObject(_key, value) {
                if (value instanceof Map) {
                    return Object.fromEntries(value);
                } else {
                    return value;
                }
            }),
            this.methodHandler,
        ).catch((error: any) => {
            throw errorHandle(error);
        });
    }

    /**
     * Listen to MQTT events.
     *
     * @param topics The topics to listen to.
     * @param callback The callback to be called when an MQTT event is received.
     */
    listen(
        topics: string[],
        callback: (error: Error, result: string) => void,
    ): Promise<void> {
        try {
            return listenMqtt(topics, callback, this.methodHandler).catch(
                (error: any) => {
                    throw errorHandle(error);
                },
            );
        } catch (error: any) {
            throw errorHandle(error);
        }
    }
}
