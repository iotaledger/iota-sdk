// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    callWalletMethod,
    createWallet,
    listenWallet,
    destroyWallet,
    getClientFromWallet,
    getSecretManagerFromWallet,
} from '../bindings';
import type {
    WalletEventType,
    WalletOptions,
    __WalletMethod__,
    Event,
} from '../types/wallet';
import { Client } from '../client';
import { SecretManager } from '../secret_manager';

// The WalletMethodHandler class interacts with methods with the rust bindings.
export class WalletMethodHandler {
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
        const methodHandler = await createWallet(JSON.stringify(options));
        return new WalletMethodHandler(methodHandler);
    }

    /**
     * Call a wallet method on the Rust backend.
     *
     * @param method The wallet method to call.
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
        ).catch((error: Error) => {
            try {
                if (error.message !== undefined) {
                    error = JSON.parse(error.message).payload;
                } else {
                    error = JSON.parse(error.toString()).payload;
                }
            } catch (e) {
                console.error(e);
            }
            return Promise.reject(error);
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
        return listenWallet(this.methodHandler, eventTypes, callback);
    }

    async destroy(): Promise<void> {
        return destroyWallet(this.methodHandler);
    }

    /**
     * Get the client associated with the wallet.
     */
    async getClient(): Promise<Client> {
        return new Promise((resolve, reject) => {
            getClientFromWallet(this.methodHandler).then((result: any) => {
                if (result.message !== undefined) {
                    reject(JSON.parse(result.message).payload);
                } else {
                    resolve(new Client(result));
                }
            });
        });
    }

    /**
     * Get the secret manager associated with the wallet.
     */
    async getSecretManager(): Promise<SecretManager> {
        return new Promise((resolve, reject) => {
            getSecretManagerFromWallet(this.methodHandler).then(
                (result: any) => {
                    if (result.message !== undefined) {
                        reject(JSON.parse(result.message).payload);
                    } else {
                        resolve(new SecretManager(result));
                    }
                },
            );
        });
    }
}
