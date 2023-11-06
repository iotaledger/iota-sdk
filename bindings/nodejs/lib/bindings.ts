// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { __UtilsMethods__ } from './types/utils';

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
    implCustomDatabase,
    testCustomDatabase,
} = addon;

const callUtilsMethod = (method: __UtilsMethods__): any => {
    const response = JSON.parse(callUtilsMethodRust(JSON.stringify(method)));
    if (response.type == 'error' || response.type == 'panic') {
        throw response;
    } else {
        return response.payload;
    }
};

export {
    initLogger,
    createClient,
    destroyClient,
    createSecretManager,
    createWallet,
    callClientMethod,
    callSecretManagerMethod,
    callUtilsMethod,
    callWalletMethod,
    destroyWallet,
    listenWallet,
    getClientFromWallet,
    getSecretManagerFromWallet,
    listenMqtt,
    migrateStrongholdSnapshotV2ToV3,
    implCustomDatabase,
    testCustomDatabase,
};
