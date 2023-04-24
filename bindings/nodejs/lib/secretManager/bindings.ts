// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { SecretManagerMethodHandler } from './SecretManagerMethodHandler';

// @ts-ignore: path is set to match runtime transpiled js path
import addon = require('../../../build/Release/index.node');

const { callSecretManagerMethod, createSecretManager } = addon;

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

export { callSecretManagerMethodAsync, createSecretManager };
