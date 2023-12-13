// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { errorHandle } from '..';
import {
    callClientMethod,
    createClient,
    listenMqtt,
    destroyClient,
} from '../bindings';
import type { IClientOptions, __ClientMethods__ } from '../types/client';

/**
 * The MethodHandler which sends the commands to the Rust side.
 */
export class ClientMethodHandler {
    methodHandler: any;

    /**
     * @param methodHandler The Rust method handler created in `ClientMethodHandler.create()`.
     */
    constructor(methodHandler: any) {
        this.methodHandler = methodHandler;
    }

    /**
     * @param options The client options.
     */
    static async create(options: IClientOptions): Promise<ClientMethodHandler> {
        try {
            const methodHandler = await createClient(JSON.stringify(options));
            return new ClientMethodHandler(methodHandler);
        } catch (error: any) {
            throw errorHandle(error);
        }
    }

    async destroy(): Promise<void> {
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
        return callClientMethod(
            this.methodHandler,
            JSON.stringify(method),
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
    async listenMqtt(
        topics: string[],
        callback: (error: Error, result: string) => void,
    ): Promise<void> {
        try {
            listenMqtt(this.methodHandler, topics, callback);
        } catch (error: any) {
            throw errorHandle(error);
        }
    }
}
