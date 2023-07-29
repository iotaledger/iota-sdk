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
import {
    Ed25519Signature,
    HexEncodedString,
    TransactionPayload,
    Unlock,
    Response,
} from '../types';

import { plainToInstance } from 'class-transformer';

/** The SecretManager to interact with nodes. */
export class SecretManager {
    private methodHandler: SecretManagerMethodHandler;

    /** TODO. */
    constructor(options: SecretManagerType | SecretManagerMethodHandler) {
        this.methodHandler = new SecretManagerMethodHandler(options);
    }

    /** Generate ed25519 addresses.
     * @param TODO TODO.
     * @returns TODO.
     */
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

    /** Generate EVM addresses.
     * @param TODO TODO.
     * @returns TODO.
     */
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
     * Store a mnemonic in the Stronghold vault.
     * @param TODO TODO.
     * @returns TODO.
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
     * Sign a transaction.
     * @param TODO TODO.
     * @returns TODO.
     */
    async signTransaction(
        preparedTransactionData: PreparedTransactionData,
    ): Promise<TransactionPayload> {
        const response = await this.methodHandler.callMethod({
            name: 'signTransaction',
            data: {
                preparedTransactionData,
            },
        });

        const parsed = JSON.parse(response) as Response<TransactionPayload>;
        return plainToInstance(TransactionPayload, parsed.payload);
    }

    /**
     * Create a signature unlock using the provided `secretManager`.
     * @param TODO TODO.
     * @returns TODO.
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
     * @param TODO TODO.
     * @returns TODO.
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
     * @param TODO TODO.
     * @returns TODO.
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
     * Get the status of a Ledger Nano.
     * @param TODO TODO.
     * @returns TODO.
     */
    async getLedgerNanoStatus(): Promise<LedgerNanoStatus> {
        const response = await this.methodHandler.callMethod({
            name: 'getLedgerNanoStatus',
        });

        return JSON.parse(response).payload;
    }
}
