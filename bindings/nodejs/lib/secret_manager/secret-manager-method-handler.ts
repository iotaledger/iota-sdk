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

/** The MethodHandler which sends the commands to the Rust side. */
export class SecretManagerMethodHandler {
    methodHandler: SecretManagerMethodHandler;

    /** TODO. */
    constructor(options: SecretManagerType | SecretManagerMethodHandler) {
        // The rust secret manager object is not extensible
        if (Object.isExtensible(options)) {
            this.methodHandler = createSecretManager(JSON.stringify(options));
        } else {
            this.methodHandler = options as SecretManagerMethodHandler;
        }
    }

    /** TODO.
     * @param TODO TODO.
     * @returns TODO.
     */
    async callMethod(method: __SecretManagerMethods__): Promise<string> {
        return callSecretManagerMethodAsync(
            JSON.stringify(method),
            this.methodHandler,
        );
    }
}

export { migrateStrongholdSnapshotV2ToV3 };
