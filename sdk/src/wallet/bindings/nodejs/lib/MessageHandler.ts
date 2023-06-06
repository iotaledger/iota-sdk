// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    sendMessageAsync,
    messageHandlerNew,
    listenWallet,
    destroy,
} from './bindings';
import type {
    WalletEventType,
    AccountManagerOptions,
    __Message__,
    __AccountMethod__,
    AccountId,
} from '../types';
import { Event } from '../types';

// The MessageHandler class interacts with messages with the rust bindings.
export class MessageHandler {
    messageHandler: any;

    constructor(options?: AccountManagerOptions) {
        const messageOptions = {
            storagePath: options?.storagePath,
            clientOptions: options?.clientOptions,
            coinType: options?.coinType,
            secretManager: options?.secretManager,
        };

        this.messageHandler = messageHandlerNew(JSON.stringify(messageOptions));
    }

    async sendMessage(message: __Message__): Promise<string> {
        return sendMessageAsync(
            JSON.stringify(message),
            this.messageHandler,
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
        return this.sendMessage({
            cmd: 'callAccountMethod',
            payload: {
                accountId: accountIndex,
                method,
            },
        });
    }

    async listen(
        eventTypes: WalletEventType[],
        callback: (error: Error, result: Event) => void,
    ): Promise<void> {
        return listenWallet(
            eventTypes,
            function (err: any, data: string) {
                const parsed = JSON.parse(data);
                callback(err, new Event(parsed.accountIndex, parsed.event));
            },
            this.messageHandler,
        );
    }

    async destroy(): Promise<void> {
        return destroy(this.messageHandler);
    }
}
