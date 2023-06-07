// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { callSecretManagerMethodAsync, createSecretManager } from '../bindings';
import {
    SecretManagerType,
    __SecretManagerMethods__,
} from '../types/secretManager';

/** The MethodHandler which sends the commands to the Rust side. */
export class SecretManagerMethodHandler {
    methodHandler: SecretManagerMethodHandler;

    constructor(options: SecretManagerType | SecretManagerMethodHandler) {
        // The rust secret manager object is not extensible
        if (Object.isExtensible(options)) {
            this.methodHandler = createSecretManager(JSON.stringify(options));
        } else {
            this.methodHandler = options as SecretManagerMethodHandler;
        }
    }

    async callMethod(method: __SecretManagerMethods__): Promise<string> {
        return callSecretManagerMethodAsync(
            JSON.stringify(method),
            this.methodHandler,
        );
    }
}
