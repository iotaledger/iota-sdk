// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { WalletEventType } from './types/wallet';
import { Event } from './types/wallet';
import type { WalletMethodHandler } from './wallet/wallet-method-handler';
import { __UtilsMethods__ } from './types/utils';
import type { SecretManagerMethodHandler } from './secret_manager/secret-manager-method-handler';
import type { ClientMethodHandler } from './client/client-method-handler';

// @ts-ignore: path is set to match runtime transpiled js path
import addon = require('../build/Release/index.node');

const {
    callUtilsMethodRust,
    callSecretManagerMethod,
    createSecretManager,
    initLogger,
    callClientMethod,
    createClient,
    destroyClient,
    listenMqtt,
    callWalletMethod,
    createWallet,
    listenWallet,
    destroyWallet,
    getClientFromWallet,
    getSecretManagerFromWallet,
    migrateStrongholdSnapshotV2ToV3,
    migrateDbChrysalisToStardust,
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
    eventTypes: WalletEventType[],
    callback: (error: Error, event: Event) => void,
    handler: WalletMethodHandler,
): Promise<void> => {
    listenWallet(
        eventTypes,
        function (err: any, data: string) {
            const parsed = JSON.parse(data);
            callback(err, new Event(parsed.accountIndex, parsed.event));
        },
        handler,
    );
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
    destroyClient,
    createSecretManager,
    createWallet,
    callClientMethodAsync,
    callSecretManagerMethodAsync,
    callUtilsMethod,
    callWalletMethodAsync,
    destroyWallet,
    listenWalletAsync,
    getClientFromWallet,
    getSecretManagerFromWallet,
    listenMqtt,
    migrateStrongholdSnapshotV2ToV3,
    migrateDbChrysalisToStardust,
};
