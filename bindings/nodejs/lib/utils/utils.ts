// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { callUtilsMethod } from '../bindings';
import {
    Address,
    HexEncodedString,
    Block,
    Ed25519Signature,
    TransactionEssence,
    MilestonePayload,
    MilestoneId,
    TransactionPayload,
    TransactionId,
    TokenSchemeType,
    Output,
    IRent,
    OutputId,
    hexToBigInt,
} from '../types';
import { AliasId, BlockId, FoundryId, TokenId } from '../types/block/id';

/** Utils class for utils. */
export class Utils {
    /**
     * Generates a new mnemonic.
     */
    static generateMnemonic(): string {
        return callUtilsMethod({
            name: 'generateMnemonic',
        });
    }

    /**
     * Returns a hex encoded seed for a mnemonic.
     */
    static mnemonicToHexSeed(mnemonic: string): HexEncodedString {
        return callUtilsMethod({
            name: 'mnemonicToHexSeed',
            data: {
                mnemonic,
            },
        });
    }

    /**
     * Computes the alias id for the given alias output id.
     */
    static computeAliasId(outputId: string): AliasId {
        return callUtilsMethod({
            name: 'computeAliasId',
            data: {
                outputId,
            },
        });
    }

    /**
     * Computes the foundry id.
     */
    static computeFoundryId(
        aliasId: AliasId,
        serialNumber: number,
        tokenSchemeKind: number,
    ): FoundryId {
        return callUtilsMethod({
            name: 'computeFoundryId',
            data: {
                aliasId,
                serialNumber,
                tokenSchemeKind,
            },
        });
    }

    /**
     * Computes the NFT id for the given NFT output id.
     */
    static computeNftId(outputId: string): string {
        return callUtilsMethod({
            name: 'computeNftId',
            data: {
                outputId,
            },
        });
    }

    /**
     * Computes the inputCommitment from the output objects that are used as inputs to fund the transaction.
     * @param inputs The output objects used as inputs for the transaction.
     * @returns The inputs commitment.
     */
    static computeInputsCommitment(inputs: Output[]): HexEncodedString {
        return callUtilsMethod({
            name: 'computeInputsCommitment',
            data: {
                inputs,
            },
        });
    }

    /**
     * Computes the output ID from transaction id and output index.
     * @param transactionId The id of the transaction.
     * @param outputIndex The index of the output.
     * @returns The output id.
     */
    static computeOutputId(id: TransactionId, index: number): OutputId {
        return callUtilsMethod({
            name: 'computeOutputId',
            data: {
                id,
                index,
            },
        });
    }

    /**
     * Computes the required storage deposit of an output.
     * @param output The output.
     * @param rentStructure Rent cost of objects which take node resources.
     * @returns The required storage deposit.
     */
    static computeStorageDeposit(output: Output, rentStructure: IRent): bigint {
        const depositHex = callUtilsMethod({
            name: 'computeStorageDeposit',
            data: {
                output,
                rentStructure,
            },
        });
        return hexToBigInt(depositHex);
    }

    /**
     * Computes a tokenId from the aliasId, serial number and token scheme type.
     * @param aliasId The alias that controls the foundry.
     * @param serialNumber The serial number of the foundry.
     * @param tokenSchemeType The tokenSchemeType of the foundry.
     * @returns The tokenId.
     */
    static computeTokenId(
        aliasId: AliasId,
        serialNumber: number,
        tokenSchemeType: TokenSchemeType,
    ): TokenId {
        return callUtilsMethod({
            name: 'computeTokenId',
            data: {
                aliasId,
                serialNumber,
                tokenSchemeType,
            },
        });
    }

    /**
     * Returns a valid Address parsed from a String.
     */
    static parseBech32Address(address: string): Address {
        const addr = callUtilsMethod({
            name: 'parseBech32Address',
            data: {
                address,
            },
        });
        return Address.parse(addr);
    }

    /**
     * Returns a block ID (Blake2b256 hash of the block bytes)
     */
    static blockId(block: Block): BlockId {
        return callUtilsMethod({
            name: 'blockId',
            data: {
                block,
            },
        });
    }

    /**
     * Returns a Milestone ID (Blake2b256 hash of the milestone essence)
     */
    static milestoneId(payload: MilestonePayload): MilestoneId {
        return callUtilsMethod({
            name: 'milestoneId',
            data: {
                payload,
            },
        });
    }

    /**
     * Returns the transaction ID (Blake2b256 hash of the provided transaction payload)
     * @param payload The transaction payload.
     * @returns The transaction id.
     */
    static transactionId(payload: TransactionPayload): TransactionId {
        return callUtilsMethod({
            name: 'transactionId',
            data: {
                payload,
            },
        });
    }

    /**
     * Transforms bech32 to hex.
     */
    static bech32ToHex(bech32: string): string {
        return callUtilsMethod({
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
        return callUtilsMethod({
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
        return callUtilsMethod({
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
        return callUtilsMethod({
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
        return callUtilsMethod({
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
        return callUtilsMethod({
            name: 'isAddressValid',
            data: {
                address,
            },
        });
    }

    /**
     * Compute the hash of a transaction essence.
     */
    static hashTransactionEssence(
        essence: TransactionEssence,
    ): HexEncodedString {
        return callUtilsMethod({
            name: 'hashTransactionEssence',
            data: {
                essence,
            },
        });
    }

    /**
     * Verifies an ed25519 signature against a message.
     */
    static verifyEd25519Signature(
        signature: Ed25519Signature,
        message: HexEncodedString,
    ): boolean {
        return callUtilsMethod({
            name: 'verifyEd25519Signature',
            data: {
                signature,
                message,
            },
        });
    }

    /**
     * Verifies a Secp256k1Ecdsa signature against a message.
     */
    static verifySecp256k1EcdsaSignature(
        publicKey: HexEncodedString,
        signature: HexEncodedString,
        message: HexEncodedString,
    ): boolean {
        return callUtilsMethod({
            name: 'verifySecp256k1EcdsaSignature',
            data: {
                publicKey,
                signature,
                message,
            },
        });
    }

    /**
     * Verify if a mnemonic is a valid BIP39 mnemonic.
     */
    static verifyMnemonic(mnemonic: string): void {
        return callUtilsMethod({
            name: 'verifyMnemonic',
            data: { mnemonic },
        });
    }
}
