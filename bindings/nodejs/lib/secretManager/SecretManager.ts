// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { SecretManagerMethodHandler } from './SecretManagerMethodHandler';
import type {
    IGenerateAddressesOptions,
    IPreparedTransactionData,
    LedgerNanoStatus,
    IBip32Chain,
} from '../../types/client/';
import type {
    PayloadTypes,
    UnlockTypes,
    HexEncodedString,
    IEd25519Signature,
} from '@iota/types';
import { SecretManagerType } from '../../types/secretManager/';

/** The SecretManager to interact with nodes. */
export class SecretManager {
    private methodHandler: SecretManagerMethodHandler;

    constructor(options: SecretManagerType) {
        this.methodHandler = new SecretManagerMethodHandler(options);
    }

    /** Generate addresses */
    async generateAddresses(
        generateAddressesOptions: IGenerateAddressesOptions,
    ): Promise<string[]> {
        const response = await this.methodHandler.callMethod({
            name: 'generateAddresses',
            data: {
                options: generateAddressesOptions,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Store a mnemonic in the Stronghold vault
     */
    async storeMnemonic(mnemonic: string): Promise<void> {
        const response = await this.methodHandler.callMethod({
            name: 'storeMnemonic',
            data: {
                mnemonic,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Sign a transaction
     */
    async signTransaction(
        preparedTransactionData: IPreparedTransactionData,
    ): Promise<PayloadTypes> {
        const response = await this.methodHandler.callMethod({
            name: 'signTransaction',
            data: {
                preparedTransactionData,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Create a signature unlock using the provided `secretManager`.
     */
    async signatureUnlock(
        // Uses `Array<number>` instead of `Uint8Array` because the latter serializes
        // as an object rather than an array, which results in errors with serde.
        transactionEssenceHash: Array<number>,
        chain: IBip32Chain,
    ): Promise<UnlockTypes> {
        const response = await this.methodHandler.callMethod({
            name: 'signatureUnlock',
            data: {
                transactionEssenceHash,
                chain,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Signs a message with an Ed25519 private key.
     */
    async signEd25519(
        message: HexEncodedString,
        chain: IBip32Chain,
    ): Promise<IEd25519Signature> {
        const response = await this.methodHandler.callMethod({
            name: 'signEd25519',
            data: {
                message,
                chain,
            },
        });
        return JSON.parse(response).payload;
    }

    /**
     * Get the status of a Ledger Nano
     */
    async getLedgerNanoStatus(): Promise<LedgerNanoStatus> {
        const response = await this.methodHandler.callMethod({
            name: 'getLedgerNanoStatus',
        });

        return JSON.parse(response).payload;
    }
}
