// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { EventType } from '../types/wallet';
import type { WalletMethodHandler } from './wallet/WalletMethodHandler';
import { __UtilsMethods__ } from '../types/utils';
import type { SecretManagerMethodHandler } from './secretManager/SecretManagerMethodHandler';
import type { ClientMethodHandler } from './client/ClientMethodHandler';

// @ts-ignore: path is set to match runtime transpiled js path
import addon = require('../../../build/Release/index.node');

const {
    callUtilsMethodRust,
    callSecretManagerMethod,
    createSecretManager,
    initLogger,
    callClientMethod,
    createClient,
    listenMqtt,
    callWalletMethod,
    createWallet,
    listenWallet,
    destroyWallet,
    getClientFromWallet,
} = addon;

const callClientMethodAsync = (
    method: string,
    handler: ClientMethodHandler,
): Promise<string> =>
    new Promise((resolve, reject) => {
        callClientMethod(method, handler, (error: Error, result: string) => {
            if (error) {
                reject(error);
            } else {
                resolve(result);
            }
        });
    });

const callSecretManagerMethodAsync = (
    method: string,
    handler: SecretManagerMethodHandler,
): Promise<string> =>
    new Promise((resolve, reject) => {
        callSecretManagerMethod(
            method,
            handler,
            (error: Error, result: string) => {
                if (error) {
                    reject(error);
                } else {
                    resolve(result);
                }
            },
        );
    });

const callUtilsMethod = (method: __UtilsMethods__): any => {
    const response = JSON.parse(callUtilsMethodRust(JSON.stringify(method)));
    if (response.type == 'error' || response.type == 'panic') {
        throw response;
    } else {
        return response.payload;
    }
};

const listenWalletAsync = (
    eventTypes: EventType[],
    callback: (error: Error, result: string) => void,
    handler: WalletMethodHandler,
): Promise<void> => {
    listenWallet(eventTypes, callback, handler);
    return Promise.resolve();
};

const callWalletMethodAsync = (
    method: string,
    handler: WalletMethodHandler,
): Promise<string> =>
    new Promise((resolve, reject) => {
        callWalletMethod(method, handler, (error: Error, result: string) => {
            if (error) {
                reject(error);
            } else {
                resolve(result);
            }
        });
    });

export {
    initLogger,
    createClient,
    createSecretManager,
    createWallet,
    callClientMethodAsync,
    callSecretManagerMethodAsync,
    callUtilsMethod,
    callWalletMethodAsync,
    destroyWallet,
    listenWalletAsync,
    getClientFromWallet,
    listenMqtt,
};
