// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    callWalletMethodAsync,
    createWallet,
    listenTo,
    destroyWallet,
    getClient,
} from './bindings';
import type {
    EventType,
    WalletOptions,
    __Method__,
    __AccountMethod__,
    AccountId,
} from '../../types/wallet';
import { Client } from '../client';

// The WalletMethodHandler class interacts with messages with the rust bindings.
export class WalletMethodHandler {
    methodHandler: any;

    constructor(options?: WalletOptions) {
        const messageOptions = {
            storagePath: options?.storagePath,
            clientOptions: options?.clientOptions,
            coinType: options?.coinType,
            secretManager: options?.secretManager,
        };

        this.methodHandler = createWallet(JSON.stringify(messageOptions));
    }

    async callMethod(method: __Method__): Promise<string> {
        return callWalletMethodAsync(
            JSON.stringify(method),
            this.methodHandler,
        ).catch((error: Error) => {
            try {
                error = JSON.parse(error.toString()).payload;
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
        eventTypes: EventType[],
        callback: (error: Error, result: string) => void,
    ): Promise<void> {
        return listenTo(eventTypes, callback, this.methodHandler);
    }

    async destroy(): Promise<void> {
        return destroyWallet(this.methodHandler);
    }

    async getClient(): Promise<Client> {
        return new Client(await getClient(this.methodHandler));
    }
}
