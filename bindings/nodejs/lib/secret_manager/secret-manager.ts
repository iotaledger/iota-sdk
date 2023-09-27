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

    /**
     * @param options A secret manager type or a secret manager method handler.
     */
    constructor(options: SecretManagerType | SecretManagerMethodHandler) {
        this.methodHandler = new SecretManagerMethodHandler(options);
    }

    /**
     * Generate Ed25519 addresses.
     *
     * @param generateAddressesOptions Options to generate addresses.
     * @returns An array of generated addresses.
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

    /**
     * Generate EVM addresses.
     *
     * @param generateAddressesOptions Options to generate addresses.
     * @returns An array of generated addresses.
     */
    async generateEvmAddresses(
        generateAddressesOptions: IGenerateAddressesOptions,
    ): Promise<HexEncodedString[]> {
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
     *
     * @param mnemonic The mnemonic to store.
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
     *
     * @param preparedTransactionData An instance of `PreparedTransactionData`.
     * @returns The corresponding transaction payload.
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
     *
     * @param transactionEssenceHash The hash of the transaction essence.
     * @param chain A BIP44 chain.
     * @returns The corresponding unlock.
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
     *
     * @param message The message to sign.
     * @param chain A BIP44 chain.
     * @returns The corresponding signature.
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
     *
     * @param message The message to sign.
     * @param chain A BIP44 chain.
     * @returns The corresponding signature.
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
     */
    async getLedgerNanoStatus(): Promise<LedgerNanoStatus> {
        const response = await this.methodHandler.callMethod({
            name: 'getLedgerNanoStatus',
        });

        return JSON.parse(response).payload;
    }
}
