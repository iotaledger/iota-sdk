// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    callSecretManagerMethodAsync,
    createSecretManager,
    migrateStrongholdSnapshotV2ToV3,
} from '../bindings';
import {
    SecretManagerType,
    __SecretManagerMethods__,
} from '../types/secret_manager';

/** The MethodHandler which sends the commands to the Rust backend. */
export class SecretManagerMethodHandler {
    methodHandler: SecretManagerMethodHandler;

    /**
     * @param options A secret manager type or a secret manager method handler.
     */
    constructor(options: SecretManagerType | SecretManagerMethodHandler) {
        // The rust secret manager object is not extensible
        if (Object.isExtensible(options)) {
            this.methodHandler = createSecretManager(JSON.stringify(options));
        } else {
            this.methodHandler = options as SecretManagerMethodHandler;
        }
    }

    /**
     * Call a secret manager method.
     *
     * @param method One of the supported secret manager methods.
     * @returns The JSON response of the method.
     */
    async callMethod(method: __SecretManagerMethods__): Promise<string> {
        return callSecretManagerMethodAsync(
            JSON.stringify(method),
            this.methodHandler,
        );
    }
}

export { migrateStrongholdSnapshotV2ToV3 };
