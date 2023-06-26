// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    callClientMethodAsync,
    createClient,
    listenMqtt,
    destroyClient,
} from '../bindings';
import type { IClientOptions, __ClientMethods__ } from '../types/client';

/** The MethodHandler which sends the commands to the Rust side. */
export class ClientMethodHandler {
    methodHandler: ClientMethodHandler;

    constructor(options: IClientOptions | ClientMethodHandler) {
        // The rust client object is not extensible
        if (Object.isExtensible(options)) {
            this.methodHandler = createClient(JSON.stringify(options));
        } else {
            this.methodHandler = options as ClientMethodHandler;
        }
    }

    async destroy(): Promise<void> {
        return destroyClient(this.methodHandler);
    }

    async callMethod(method: __ClientMethods__): Promise<string> {
        return callClientMethodAsync(
            JSON.stringify(method),
            this.methodHandler,
        );
    }

    // MQTT
    async listen(
        topics: string[],
        callback: (error: Error, result: string) => void,
    ): Promise<void> {
        return listenMqtt(topics, callback, this.methodHandler);
    }
}
