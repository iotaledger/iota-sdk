// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    callWalletMethod,
    createWallet,
    listenWallet,
    destroyWallet,
    getClient,
    getSecretManager,
} from '../bindings';
import {
    WalletEventType,
    WalletOptions,
    __WalletMethod__,
    Event,
} from '../types/wallet';
import { Client, ClientMethodHandler } from '../client';
import { SecretManager, SecretManagerMethodHandler } from '../secret_manager';
import { errorHandle } from '..';

// The WalletMethodHandler class interacts with methods with the rust bindings.
export class WalletMethodHandler {
    // External rust object
    methodHandler: any;

    /**
     * @param methodHandler The Rust method handler created in `WalletMethodHandler.create()`.
     */
    constructor(methodHandler: any) {
        this.methodHandler = methodHandler;
    }

    /**
     * @param options The wallet options.
     */
    static async create(options: WalletOptions): Promise<WalletMethodHandler> {
        try {
            const methodHandler = await createWallet(JSON.stringify(options));
            return new WalletMethodHandler(methodHandler);
        } catch (error: any) {
            throw errorHandle(error);
        }
    }

    /**
     * Call a wallet method on the Rust backend.
     *
     * @param method The wallet method to call.
     * @returns A promise that resolves to a JSON string response holding the result of the wallet method.
     */
    async callMethod(method: __WalletMethod__): Promise<string> {
        return callWalletMethod(
            this.methodHandler,
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
     * Listen to wallet events.
     *
     * @param eventTypes The wallet event types to listen for.
     * @param callback The callback function to call when an event is received.
     */
    async listen(
        eventTypes: WalletEventType[],
        callback: (error: Error, event: Event) => void,
    ): Promise<void> {
        return listenWallet(
            this.methodHandler,
            eventTypes,
            function (err: any, data: string) {
                const parsed = JSON.parse(data);
                callback(
                    // Send back raw error instead of parsing
                    err,
                    new Event(parsed.accountIndex, parsed.event),
                );
            },
        ).catch((error: any) => {
            throw errorHandle(error);
        });
    }

    async destroy(): Promise<void> {
        try {
            await destroyWallet(this.methodHandler);
        } catch (error: any) {
            throw errorHandle(error);
        }
    }

    async getClient(): Promise<Client> {
        try {
            const result = await getClient(this.methodHandler);
            return new Client(new ClientMethodHandler(result));
        } catch (error: any) {
            throw errorHandle(error);
        }
    }

    /**
     * Get the secret manager associated with the wallet.
     */
    async getSecretManager(): Promise<SecretManager> {
        try {
            const result = await getSecretManager(this.methodHandler);
            return new SecretManager(new SecretManagerMethodHandler(result));
        } catch (error: any) {
            throw errorHandle(error);
        }
    }
}
