// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SecretManagerMethodHandler } from './secret-manager-method-handler';
import type {
    GenerateAddressesOptions,
    PreparedTransactionData,
    LedgerNanoStatus,
    CoinType,
    GenerateAddressOptions,
} from '../types/client';
import {
    Bip44,
    Secp256k1EcdsaSignature,
    SecretManagerType,
} from '../types/secret_manager';
import {
    Ed25519Signature,
    HexEncodedString,
    SignedTransactionPayload,
    Unlock,
    Response,
    UnsignedBlock,
    Block,
    parseBlock,
    Bech32Address,
} from '../types';

import { plainToInstance } from 'class-transformer';

/** The SecretManager to interact with nodes. */
export class SecretManager {
    private methodHandler: SecretManagerMethodHandler;

    /**
     * @param options A secret manager type or a secret manager method handler.
     */
    constructor(methodHandler: SecretManagerMethodHandler) {
        this.methodHandler = methodHandler;
    }

    /**
     * @param options The secret manager options.
     */
    static create(options: SecretManagerType): SecretManager {
        return new SecretManager(SecretManagerMethodHandler.create(options));
    }

    /**
     * Generate a single Ed25519 address.
     *
     * @returns The generated Bech32 address.
     */
    async generateEd25519Address(
        coinType: CoinType,
        bech32Hrp: string,
        accountIndex?: number,
        addressIndex?: number,
        address_options?: GenerateAddressOptions,
    ): Promise<Bech32Address> {
        const options = {
            coinType,
            bech32Hrp,
            accountIndex,
            addressIndex,
            options: address_options,
        };
        const response = await this.methodHandler.callMethod({
            name: 'generateEd25519Addresses',
            data: {
                options,
            },
        });

        return JSON.parse(response[0]).payload;
    }

    /**
     * Generate multiple Ed25519 addresses at once.
     *
     * @param generateAddressesOptions Options to generate addresses.
     * @returns An array of generated addresses.
     */
    async generateEd25519Addresses(
        generateAddressesOptions: GenerateAddressesOptions,
    ): Promise<Bech32Address[]> {
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
        generateAddressesOptions: GenerateAddressesOptions,
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
    ): Promise<SignedTransactionPayload> {
        const response = await this.methodHandler.callMethod({
            name: 'signTransaction',
            data: {
                preparedTransactionData,
            },
        });

        const parsed = JSON.parse(
            response,
        ) as Response<SignedTransactionPayload>;
        return plainToInstance(SignedTransactionPayload, parsed.payload);
    }

    /**
     * Sign a block.
     *
     * @param unsignedBlock An unsigned block.
     * @param chain A BIP44 chain.
     * @returns The signed block.
     */
    async signBlock(
        unsignedBlock: UnsignedBlock,
        chain: Bip44,
    ): Promise<Block> {
        const response = await this.methodHandler.callMethod({
            name: 'signBlock',
            data: {
                unsignedBlock,
                chain,
            },
        });

        const parsed = JSON.parse(response) as Response<Block>;
        return parseBlock(parsed.payload);
    }

    /**
     * Create a signature unlock using the provided `secretManager`.
     *
     * @param transactionSigningHash The signing hash of the transaction.
     * @param chain A BIP44 chain.
     * @returns The corresponding unlock.
     */
    async signatureUnlock(
        transactionSigningHash: HexEncodedString,
        chain: Bip44,
    ): Promise<Unlock> {
        const response = await this.methodHandler.callMethod({
            name: 'signatureUnlock',
            data: {
                transactionSigningHash,
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

    /**
     * Set the Stronghold password.
     */
    async setStrongholdPassword(password: string): Promise<void> {
        await this.methodHandler.callMethod({
            name: 'setStrongholdPassword',
            data: { password },
        });
    }

    /**
     * Change the Stronghold password.
     */
    async changeStrongholdPassword(password: string): Promise<void> {
        await this.methodHandler.callMethod({
            name: 'changeStrongholdPassword',
            data: { password },
        });
    }

    /**
     * Clear the Stronghold password.
     */
    async clearStrongholdPassword(): Promise<void> {
        await this.methodHandler.callMethod({
            name: 'clearStrongholdPassword',
        });
    }
}
