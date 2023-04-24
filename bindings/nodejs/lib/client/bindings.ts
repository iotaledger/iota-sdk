// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { ClientMethodHandler } from './ClientMethodHandler';

// @ts-ignore: path is set to match runtime transpiled js path
import addon = require('../../../build/Release/index.node');

const { initLogger, callClientMethod, createClient, listenMqtt } = addon;

const callMethodAsync = (
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

export { initLogger, callMethodAsync, createClient, listenMqtt };
