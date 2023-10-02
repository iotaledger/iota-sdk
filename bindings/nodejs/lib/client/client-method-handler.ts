// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

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
        // The rust client object is not extensible
        if (Object.isExtensible(options)) {
            this.methodHandler = createClient(JSON.stringify(options));
        } else {
            this.methodHandler = options as ClientMethodHandler;
        }
    }

    async destroy() {
        return destroyClient(this.methodHandler);
    }

    /**
     * Call a client method.
     *
     * @param method The client method.
     * @returns A promise that resolves to a JSON string response holding the result of the client method.
     */
    async callMethod(method: __ClientMethods__): Promise<string> {
        return callClientMethodAsync(
            JSON.stringify(method),
            this.methodHandler,
        );
    }

    /**
     * Listen to MQTT events.
     *
     * @param topics The topics to listen to.
     * @param callback The callback to be called when an MQTT event is received.
     */
    async listen(
        topics: string[],
        callback: (error: Error, result: string) => void,
    ): Promise<void> {
        return listenMqtt(topics, callback, this.methodHandler);
    }
}
