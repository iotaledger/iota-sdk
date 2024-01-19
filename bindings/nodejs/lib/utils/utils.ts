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
    Bech32Address,
    InputSigningData,
    INodeInfoProtocol,
} from '../types';
import { AliasId, BlockId, FoundryId, NftId, TokenId } from '../types/block/id';

/** Utils class for utils. */
export class Utils {
    /**
     * Generate a new mnemonic.
     */
    static generateMnemonic(): string {
        return callUtilsMethod({
            name: 'generateMnemonic',
        });
    }

    /**
     * Convert a mnemonic to a hex encoded seed.
     *
     * @param mnemonic A mnemonic string.
     * @returns The seed as hex-encoded string.
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
     * Compute the alias ID from a given Alias output ID.
     *
     * @param outputId The output ID as hex-encoded string.
     * @returns The Alias ID.
     */
    static computeAliasId(outputId: OutputId): AliasId {
        return callUtilsMethod({
            name: 'computeAliasId',
            data: {
                outputId,
            },
        });
    }

    /**
     * Compute the Foundry ID.
     *
     * @param aliasId The Alias ID associated with the Foundry.
     * @param serialNumber The serial number of the Foundry.
     * @param tokenSchemeType The Token scheme type. Currently only a simple scheme is supported.
     * @returns The Foundry ID.
     */
    static computeFoundryId(
        aliasId: AliasId,
        serialNumber: number,
        tokenSchemeType: number,
    ): FoundryId {
        return callUtilsMethod({
            name: 'computeFoundryId',
            data: {
                aliasId,
                serialNumber,
                tokenSchemeType,
            },
        });
    }

    /**
     * Compute the NFT ID from the given NFT output ID.
     *
     * @param outputId The output ID as hex-encoded string.
     * @returns The NFT ID.
     */
    static computeNftId(outputId: OutputId): NftId {
        return callUtilsMethod({
            name: 'computeNftId',
            data: {
                outputId,
            },
        });
    }

    /**
     * Compute the input commitment from the output objects that are used as inputs to fund the transaction.
     *
     * @param inputs The output objects used as inputs for the transaction.
     * @returns The inputs commitment hash as hex-encoded string.
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
     * Compute the output ID from transaction ID and output index.
     *
     * @param transactionId The ID of the transaction.
     * @param outputIndex The index of the output.
     * @returns The output ID.
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
     * Compute the required storage deposit of an output.
     *
     * @param output The output.
     * @param rent Rent cost of objects which take node resources.
     * @returns The required storage deposit.
     */
    static computeStorageDeposit(output: Output, rent: IRent): bigint {
        const minStorageDepositAmount = callUtilsMethod({
            name: 'computeStorageDeposit',
            data: {
                output,
                rent,
            },
        });
        return BigInt(minStorageDepositAmount);
    }

    /**
     * Compute a Token iD from the aliasId, serial number and token scheme type.
     *
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
     * Parse a Bech32 address from a string.
     *
     * @param address A Bech32 address as string.
     * @returns A Bech32 address.
     */
    static parseBech32Address(address: Bech32Address): Address {
        const addr = callUtilsMethod({
            name: 'parseBech32Address',
            data: {
                address,
            },
        });
        return Address.parse(addr);
    }

    /**
     * Compute the block ID (Blake2b256 hash of the block bytes) of a block.
     *
     * @param block A block.
     * @returns The corresponding block ID.
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
     * Compute the milestone ID (Blake2b256 hash of the milestone essence) of a milestone payload.
     *
     * @param payload A milestone payload.
     * @returns The milestone ID.
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
     * Compute the transaction ID (Blake2b256 hash of the provided transaction payload) of a transaction payload.
     *
     * @param payload A transaction payload.
     * @returns The transaction ID.
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
     * Convert a Bech32 address to a hex-encoded string.
     *
     * @param bech32 A Bech32 address.
     * @returns The hex-encoded string.
     */
    static bech32ToHex(bech32: Bech32Address): HexEncodedString {
        return callUtilsMethod({
            name: 'bech32ToHex',
            data: {
                bech32,
            },
        });
    }

    /**
     * Convert a hex-encoded address string to a Bech32-encoded address string.
     *
     * @param hex A hex-encoded address string.
     * @param bech32Hrp The Bech32 HRP (human readable part) to use.
     * @returns The Bech32-encoded address string.
     */
    static hexToBech32(
        hex: HexEncodedString,
        bech32Hrp: string,
    ): Bech32Address {
        return callUtilsMethod({
            name: 'hexToBech32',
            data: {
                hex,
                bech32Hrp,
            },
        });
    }

    /**
     * Convert an Alias ID to a Bech32-encoded address string.
     *
     * @param aliasId An Alias ID.
     * @param bech32Hrp The Bech32 HRP (human readable part) to use.
     * @returns The Bech32-encoded address string.
     */
    static aliasIdToBech32(aliasId: AliasId, bech32Hrp: string): Bech32Address {
        return callUtilsMethod({
            name: 'aliasIdToBech32',
            data: {
                aliasId,
                bech32Hrp,
            },
        });
    }

    /**
     * Convert an NFT ID to a Bech32-encoded address string.
     *
     * @param nftId An NFT ID.
     * @param bech32Hrp The Bech32 HRP (human readable part) to use.
     * @returns The Bech32-encoded address string.
     */
    static nftIdToBech32(nftId: NftId, bech32Hrp: string): Bech32Address {
        return callUtilsMethod({
            name: 'nftIdToBech32',
            data: {
                nftId,
                bech32Hrp,
            },
        });
    }

    /**
     * Convert a hex-encoded public key to a Bech32-encoded address string.
     *
     * @param hex A hex-encoded public key.
     * @param bech32Hrp The Bech32 HRP (human readable part) to use.
     * @returns The Bech32-encoded address string.
     */
    static hexPublicKeyToBech32Address(
        hex: HexEncodedString,
        bech32Hrp: string,
    ): Bech32Address {
        return callUtilsMethod({
            name: 'hexPublicKeyToBech32Address',
            data: {
                hex,
                bech32Hrp,
            },
        });
    }

    /**
     * Checks whether an address string is a valid Bech32-encoded address.
     *
     * @param address An address string.
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
     *
     * @param essence A transaction essence.
     * @returns The hash of the transaction essence as a hex-encoded string.
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
     * Verify an Ed25519 signature against a message.
     *
     * @param signature An Ed25519 signature.
     * @param message A hex-encoded message.
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
     * Verify a Secp256k1Ecdsa signature against a message.
     * @param publicKey A hex-encoded public key.
     * @param signature A hex-encoded signature.
     * @param message A hex-encoded message.
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
     *
     * @param mnemonic A mnemonic string.
     */
    static verifyMnemonic(mnemonic: string): void {
        return callUtilsMethod({
            name: 'verifyMnemonic',
            data: { mnemonic },
        });
    }

    /**
     * Returns the hex representation of the serialized output bytes.
     *
     * @param output The output.
     * @returns The hex representation of the serialized output bytes.
     */
    static outputHexBytes(output: Output): HexEncodedString {
        const hexBytes = callUtilsMethod({
            name: 'outputHexBytes',
            data: {
                output,
            },
        });
        return hexBytes;
    }

    /**
     * Verifies the semantic of a transaction.
     *
     * @param inputs The inputs data.
     * @param transaction The transaction payload.
     * @param time The unix time for which to do the validation, should be roughly the one of the milestone that will reference the transaction.
     * @returns The conflict reason.
     */
    static verifyTransactionSemantic(
        inputs: InputSigningData[],
        transaction: TransactionPayload,
        time: number,
    ): string {
        const conflictReason = callUtilsMethod({
            name: 'verifyTransactionSemantic',
            data: {
                inputs,
                transaction,
                time,
            },
        });
        return conflictReason;
    }

    /**
     * Verifies the syntax of a transaction.
     *
     * @param transaction The transaction payload.
     * @param protocolParameters The protocol parameters used for the validation.
     * @returns void.
     */
    static verifyTransactionSyntax(
        transaction: TransactionPayload,
        protocolParameters: INodeInfoProtocol,
    ): void {
        return callUtilsMethod({
            name: 'verifyTransactionSyntax',
            data: {
                transaction,
                protocolParameters,
            },
        });
    }

    /**
     * Returns the serialized bytes of a block.
     *
     * @param block The block.
     * @returns The block bytes.
     */
    static blockBytes(block: Block): Uint8Array {
        const blockBytes = callUtilsMethod({
            name: 'blockBytes',
            data: {
                block,
            },
        });
        return new Uint8Array(blockBytes);
    }

    /**
     * Returns a block hash (Blake2b256 hash of block bytes without nonce) from a block for PoW.
     *
     * @param block The block.
     * @returns The Blake2b256 hash of the block bytes without nonce.
     */
    static blockHashWithoutNonce(block: Block): HexEncodedString {
        const hash = callUtilsMethod({
            name: 'blockHashWithoutNonce',
            data: {
                block,
            },
        });
        return hash;
    }
}
