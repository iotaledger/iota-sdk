// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { BlockId } from '../../types/client/';
import type {
    AddressTypes,
    IBlock,
    ITransactionEssence,
    HexEncodedString,
    IEd25519Signature,
    IEd25519Address,
} from '@iota/types';

import { callUtilsMethodJson } from './bindings';

/** Utils class for utils. */
export class Utils {
    /**
     * Generates a new mnemonic.
     */
    static generateMnemonic(): string {
        return callUtilsMethodJson({
            name: 'generateMnemonic',
        });
    }

    /**
     * Returns a hex encoded seed for a mnemonic.
     */
    static mnemonicToHexSeed(mnemonic: string): string {
        return callUtilsMethodJson({
            name: 'mnemonicToHexSeed',
            data: {
                mnemonic,
            },
        });
    }

    /**
     * Computes the alias id for the given alias output id.
     */
    static computeAliasId(outputId: string): string {
        return callUtilsMethodJson({
            name: 'computeAliasId',
            data: {
                outputId,
            },
        });
    }

    /**
     * Computes the NFT id for the given NFT output id.
     */
    static computeNftId(outputId: string): string {
        return callUtilsMethodJson({
            name: 'computeNftId',
            data: {
                outputId,
            },
        });
    }

    /**
     * Computes the foundry id.
     */
    static computeFoundryId(
        aliasAddress: string,
        serialNumber: number,
        tokenSchemeKind: number,
    ): string {
        return callUtilsMethodJson({
            name: 'computeFoundryId',
            data: {
                aliasAddress,
                serialNumber,
                tokenSchemeKind,
            },
        });
    }

    /**
     * Returns a valid Address parsed from a String.
     */
    static parseBech32Address(address: string): AddressTypes {
        return callUtilsMethodJson({
            name: 'parseBech32Address',
            data: {
                address,
            },
        });
    }

    /**
     * Returns a block ID (Blake2b256 hash of the block bytes)
     */
    static blockId(block: IBlock): BlockId {
        return callUtilsMethodJson({
            name: 'blockId',
            data: {
                block,
            },
        });
    }

    /**
     * Transforms bech32 to hex.
     */
    static bech32ToHex(bech32: string): string {
        return callUtilsMethodJson({
            name: 'bech32ToHex',
            data: {
                bech32,
            },
        });
    }

    /**
     * Transforms a hex encoded address to a bech32 encoded address.
     */
    static hexToBech32(hex: string, bech32Hrp: string): string {
        return callUtilsMethodJson({
            name: 'hexToBech32',
            data: {
                hex,
                bech32Hrp,
            },
        });
    }

    /**
     * Transforms an alias id to a bech32 encoded address.
     */
    static aliasIdToBech32(aliasId: string, bech32Hrp: string): string {
        return callUtilsMethodJson({
            name: 'aliasIdToBech32',
            data: {
                aliasId,
                bech32Hrp,
            },
        });
    }

    /**
     * Transforms an nft id to a bech32 encoded address.
     */
    static nftIdToBech32(nftId: string, bech32Hrp: string): string {
        return callUtilsMethodJson({
            name: 'nftIdToBech32',
            data: {
                nftId,
                bech32Hrp,
            },
        });
    }

    /**
     * Transforms a hex encoded public key to a bech32 encoded address.
     */
    static hexPublicKeyToBech32Address(hex: string, bech32Hrp: string): string {
        return callUtilsMethodJson({
            name: 'hexPublicKeyToBech32Address',
            data: {
                hex,
                bech32Hrp,
            },
        });
    }

    /**
     * Checks if a String is a valid bech32 encoded address.
     */
    static isAddressValid(address: string): boolean {
        return callUtilsMethodJson({
            name: 'isAddressValid',
            data: {
                address,
            },
        });
    }

    /**
     * Compute the hash of a transaction essence.
     */
    static hashTransactionEssence(essence: ITransactionEssence): string {
        return callUtilsMethodJson({
            name: 'hashTransactionEssence',
            data: {
                essence,
            },
        });
    }

    /**
     * Verifies the Ed25519Signature for a message against an Ed25519Address.
     */
    static verifyEd25519Signature(
        signature: IEd25519Signature,
        message: HexEncodedString,
        address: IEd25519Address,
    ): boolean {
        return callUtilsMethodJson({
            name: 'verifyEd25519Signature',
            data: {
                signature,
                message,
                address,
            },
        });
    }
    /**
     * Verify if a mnemonic is a valid BIP39 mnemonic.
     */
    static verifyMnemonic(mnemonic: string): void {
        return callUtilsMethodJson({
            name: 'verifyMnemonic',
            data: { mnemonic },
        });
    }
}
