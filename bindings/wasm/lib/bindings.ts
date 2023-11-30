// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// This file overwrites the `bindings.ts` file from `bindings/nodejs/lib`, to link the Wasm `MethodHandler` interface.
// The rest of the TypeScript definitions are copied as-is to the `out` directory before being compiled.

import { __UtilsMethods__ } from './utils';

// Import needs to be in a single line, otherwise it breaks
// prettier-ignore
// @ts-ignore: path is set to match runtime transpiled js path when bundled.
import { initLogger, getClient, createClient, destroyClient, createSecretManager, createWallet, callClientMethod, callSecretManagerMethod, callUtilsMethodRust, callWalletMethod, destroyWallet, listenWallet, getSecretManager, listenMqtt, migrateStrongholdSnapshotV2ToV3 } from '../wasm/iota_sdk_wasm';

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
    createWallet,
    createSecretManager,
    callClientMethod,
    callSecretManagerMethod,
    callUtilsMethod,
    callWalletMethod,
    listenWallet,
    destroyWallet,
    destroyClient,
    getClient,
    getSecretManager,
    listenMqtt,
    migrateStrongholdSnapshotV2ToV3,
};
