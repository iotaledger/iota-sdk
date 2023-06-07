// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    callWalletMethodAsync,
    createWallet,
    listenWalletAsync,
    destroyWallet,
    getClientFromWallet,
    getSecretManagerFromWallet,
} from '../bindings';
import type {
    WalletEventType,
    WalletOptions,
    __Method__,
    __AccountMethod__,
    AccountId,
    Event,
} from '../types/wallet';
import { Client } from '../client';
import { SecretManager } from '../secretManager';

// The WalletMethodHandler class interacts with methods with the rust bindings.
export class WalletMethodHandler {
    methodHandler: any;

    constructor(options?: WalletOptions) {
        const walletOptions = {
            storagePath: options?.storagePath,
            clientOptions: options?.clientOptions,
            coinType: options?.coinType,
            secretManager: options?.secretManager,
        };

        this.methodHandler = createWallet(JSON.stringify(walletOptions));
    }

    async callMethod(method: __Method__): Promise<string> {
        return callWalletMethodAsync(
            // mapToObject is required to convert maps to array since they otherwise get serialized as `[{}]` even if not empty
            JSON.stringify(method, function mapToObject(_key, value) {
                if (value instanceof Map) {
                    return Object.fromEntries(value);
                } else {
                    return value;
                }
            }),
            this.methodHandler,
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

    async callAccountMethod(
        accountIndex: AccountId,
        method: __AccountMethod__,
    ): Promise<string> {
        return this.callMethod({
            name: 'callAccountMethod',
            data: {
                accountId: accountIndex,
                method,
            },
        });
    }

    async listen(
        eventTypes: WalletEventType[],
        callback: (error: Error, event: Event) => void,
    ): Promise<void> {
        return listenWalletAsync(eventTypes, callback, this.methodHandler);
    }

    async destroy(): Promise<void> {
        return destroyWallet(this.methodHandler);
    }

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
