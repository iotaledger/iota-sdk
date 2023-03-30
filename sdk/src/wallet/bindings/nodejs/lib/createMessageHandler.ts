// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    sendMessageAsync,
    messageHandlerNew,
    listen,
    destroy,
} from './bindings';
import type {
    EventType,
    AccountManagerOptions,
    __Message__,
    __AccountMethod__,
    AccountId,
    MessageHandler,
} from '../types';


// The factory function to create a MessageHandler class that interacts with the rust bindings.
export async function createMessageHandler(options?: AccountManagerOptions): Promise<MessageHandler> {
    const messageOptions = {
        storagePath: options?.storagePath,
        clientOptions: options?.clientOptions,
        coinType: options?.coinType,
        secretManager: options?.secretManager
    };
    const { messageHandler } = await messageHandlerNew(JSON.stringify(messageOptions));
    
    async function sendMessage(message: __Message__): Promise<string> {
        return sendMessageAsync(
            JSON.stringify(message),
            messageHandler,
        ).catch((error: Error) => {
            try {
                error = JSON.parse(error.toString()).payload;
            } catch (e) {}
            return Promise.reject(error);
        });
    }
    
    return {
        sendMessage,
    
        async callAccountMethod(
            accountIndex: AccountId,
            method: __AccountMethod__,
        ): Promise<string> {
            return sendMessage({
                cmd: 'callAccountMethod',
                payload: {
                    accountId: accountIndex,
                    method,
                },
            });
        },
    
        async listen(
            eventTypes: EventType[],
            callback: (error: Error, result: string) => void,
        ): Promise<void> {
            return listen(eventTypes, callback, messageHandler);
        },
    
        async destroy(): Promise<void> {
            return destroy(messageHandler);
        }
    }
}
