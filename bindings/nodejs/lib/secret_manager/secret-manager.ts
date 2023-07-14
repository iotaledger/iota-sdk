// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SecretManagerMethodHandler } from './secret-manager-method-handler';
import type {
    IGenerateAddressesOptions,
    PreparedTransactionData,
    LedgerNanoStatus,
} from '../types/client';
import {
    Bip44,
    Secp256k1EcdsaSignature,
    SecretManagerType,
} from '../types/secret_manager';
import { Ed25519Signature, HexEncodedString, Payload, Unlock } from '../types';

/** The SecretManager to interact with nodes. */
export class SecretManager {
    private methodHandler: SecretManagerMethodHandler;

    constructor(options: SecretManagerType | SecretManagerMethodHandler) {
        this.methodHandler = new SecretManagerMethodHandler(options);
    }

    /** Generate ed25519 addresses */
    async generateEd25519Addresses(
        generateAddressesOptions: IGenerateAddressesOptions,
    ): Promise<string[]> {
        const response = await this.methodHandler.callMethod({
            name: 'generateEd25519Addresses',
            data: {
                options: generateAddressesOptions,
            },
        });

        return JSON.parse(response).payload;
    }

    /** Generate EVM addresses */
    async generateEvmAddresses(
        generateAddressesOptions: IGenerateAddressesOptions,
    ): Promise<string[]> {
        const response = await this.methodHandler.callMethod({
            name: 'generateEvmAddresses',
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
        preparedTransactionData: PreparedTransactionData,
    ): Promise<Payload> {
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
        transactionEssenceHash: HexEncodedString,
        chain: Bip44,
    ): Promise<Unlock> {
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
        chain: Bip44,
    ): Promise<Ed25519Signature> {
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
     * Signs a message with an Secp256k1Ecdsa private key.
     */
    async signSecp256k1Ecdsa(
        message: HexEncodedString,
        chain: Bip44,
    ): Promise<Secp256k1EcdsaSignature> {
        const response = await this.methodHandler.callMethod({
            name: 'signSecp256k1Ecdsa',
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
