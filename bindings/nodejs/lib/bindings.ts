// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { __UtilsMethods__ } from './types/utils';

// @ts-ignore: path is set to match runtime transpiled js path
import addon = require('../build/Release/index.node');
import { errorHandle } from '.';

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
    getClient,
    getSecretManager,
    migrateStrongholdSnapshotV2ToV3,
} = addon;

const callUtilsMethod = (method: __UtilsMethods__): any => {
    try {
        const response = JSON.parse(
            callUtilsMethodRust(JSON.stringify(method)),
        );
        return response.payload;
    } catch (error: any) {
        throw errorHandle(error);
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
    getClient,
    getSecretManager,
    listenMqtt,
    migrateStrongholdSnapshotV2ToV3,
};
