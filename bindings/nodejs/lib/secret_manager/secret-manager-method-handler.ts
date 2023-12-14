// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { errorHandle } from '..';
import {
    callSecretManagerMethod,
    createSecretManager,
    migrateStrongholdSnapshotV2ToV3,
} from '../bindings';
import {
    SecretManagerType,
    __SecretManagerMethods__,
} from '../types/secret_manager';

/** The MethodHandler which sends the commands to the Rust backend. */
export class SecretManagerMethodHandler {
    methodHandler: any;

    /**
     * @param methodHandler The Rust method handler created in `SecretManagerMethodHandler.create()`.
     */
    constructor(methodHandler: any) {
        this.methodHandler = methodHandler;
    }

    /**
     * @param options A secret manager type or a secret manager method handler.
     */
    static create(options: SecretManagerType): SecretManagerMethodHandler {
        try {
            const methodHandler = createSecretManager(JSON.stringify(options));
            return new SecretManagerMethodHandler(methodHandler);
        } catch (error: any) {
            throw errorHandle(error);
        }
    }

    /**
     * Call a secret manager method.
     *
     * @param method One of the supported secret manager methods.
     * @returns The JSON response of the method.
     */
    async callMethod(method: __SecretManagerMethods__): Promise<string> {
        return callSecretManagerMethod(
            this.methodHandler,
            JSON.stringify(method),
        ).catch((error: any) => {
            throw errorHandle(error);
        });
    }
}

export { migrateStrongholdSnapshotV2ToV3 };
