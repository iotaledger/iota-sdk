// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SecretManagerMethodHandler } from './secret-manager-method-handler';
import type {
    IGenerateAddressesOptions,
    PreparedTransactionData,
    LedgerNanoStatus,
    IBip32Chain,
} from '../types/client';
import { EvmSignature, SecretManagerType } from '../types/secret_manager';
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
        chain: IBip32Chain,
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
        chain: IBip32Chain,
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
     * Signs a message with an Evm private key.
     */
    async signEvm(
        message: HexEncodedString,
        chain: IBip32Chain,
    ): Promise<EvmSignature> {
        const response = await this.methodHandler.callMethod({
            name: 'signEvm',
            data: {
                message,
                chain,
            },
        });
        return JSON.parse(response).payload;
    }

    /**
     * Verifies an Ed25519 Signature with a message.
     */
    async verifyEd25519(
        signature: Ed25519Signature,
        message: HexEncodedString,
    ): Promise<Ed25519Signature> {
        const response = await this.methodHandler.callMethod({
            name: 'verifyEd25519',
            data: {
                signature,
                message,
            },
        });
        return JSON.parse(response).payload;
    }

    /**
     * Verifies an Evm Signature with a message.
     */
    async verifyEvm(
        public_key: HexEncodedString,
        signature: HexEncodedString,
        message: HexEncodedString,
    ): Promise<EvmSignature> {
        const response = await this.methodHandler.callMethod({
            name: 'verifyEvm',
            data: {
                public_key,
                signature,
                message,
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
