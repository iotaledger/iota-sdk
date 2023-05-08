// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { callSecretManagerMethodAsync, createSecretManager } from '../bindings';
import type {
    SecretManagerType,
    __SecretManagerMethods__,
} from '../../types/secretManager/';

/** The MethodHandler which sends the commands to the Rust side. */
export class SecretManagerMethodHandler {
    methodHandler: SecretManagerMethodHandler;

    constructor(secretManager: SecretManagerType) {
        this.methodHandler = createSecretManager(JSON.stringify(secretManager));
    }

    async callMethod(method: __SecretManagerMethods__): Promise<string> {
        return callSecretManagerMethodAsync(
            JSON.stringify(method),
            this.methodHandler,
        );
    }
}
