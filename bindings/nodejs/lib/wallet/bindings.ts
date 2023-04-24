// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { EventType } from '../../types/wallet';
import type { WalletMethodHandler } from './WalletMethodHandler';

// @ts-ignore: path is set to match runtime transpiled js path
import addon = require('../../../build/Release/index.node');

const {
    callWalletMethod,
    createWallet,
    listenWallet,
    destroyWallet,
    getClient,
} = addon;

const listenTo = (
    eventTypes: EventType[],
    callback: (error: Error, result: string) => void,
    handler: WalletMethodHandler,
): Promise<void> => {
    listenWallet(eventTypes, callback, handler);
    return Promise.resolve();
};

const callWalletMethodAsync = (
    message: string,
    handler: WalletMethodHandler,
): Promise<string> =>
    new Promise((resolve, reject) => {
        callWalletMethod(message, handler, (error: Error, result: string) => {
            if (error) {
                reject(error);
            } else {
                resolve(result);
            }
        });
    });

export {
    callWalletMethodAsync,
    createWallet,
    listenTo,
    destroyWallet,
    getClient,
};
