// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { callUtilsMethod } from '../bindings';
import {
    Address,
    HexEncodedString,
    Block,
    Ed25519Signature,
    TransactionEssence,
    TransactionPayload,
    TransactionId,
    TokenSchemeType,
    Output,
    IRent,
    OutputId,
    u64,
} from '../types';
import {
    AccountId,
    BlockId,
    FoundryId,
    NftId,
    TokenId,
} from '../types/block/id';
import { SlotCommitment, SlotCommitmentId } from '../types/block/slot';

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
     * Compute the account ID from a given account output ID.
     *
     * @param outputId The output ID as hex-encoded string.
     * @returns The account ID.
     */
    static computeAccountId(outputId: string): AccountId {
        return callUtilsMethod({
            name: 'computeAccountId',
            data: {
                outputId,
            },
        });
    }

    /**
     * Compute the Foundry ID.
     *
     * @param accountId The account ID associated with the Foundry.
     * @param serialNumber The serial number of the Foundry.
     * @param tokenSchemeType The Token scheme type. Currently only a simple scheme is supported.
     * @returns The Foundry ID.
     */
    static computeFoundryId(
        accountId: AccountId,
        serialNumber: number,
        tokenSchemeType: number,
    ): FoundryId {
        return callUtilsMethod({
            name: 'computeFoundryId',
            data: {
                accountId,
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
    static computeNftId(outputId: string): NftId {
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
    static computeStorageDeposit(output: Output, rent: IRent): u64 {
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
     * Computes a tokenId from the account ID, serial number and token scheme type.
     * @param accountId The account that controls the foundry.
     * @param serialNumber The serial number of the foundry.
     * @param tokenSchemeType The tokenSchemeType of the foundry.
     * @returns The tokenId.
     */
    static computeTokenId(
        accountId: AccountId,
        serialNumber: number,
        tokenSchemeType: TokenSchemeType,
    ): TokenId {
        return callUtilsMethod({
            name: 'computeTokenId',
            data: {
                accountId,
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
    static bech32ToHex(bech32: string): string {
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
     * Transforms an account id to a bech32 encoded address.
     *
     * @param accountId An account ID.
     * @param bech32Hrp The Bech32 HRP (human readable part) to use.
     * @returns The Bech32-encoded address string.
     */
    static accountIdToBech32(accountId: string, bech32Hrp: string): string {
        return callUtilsMethod({
            name: 'accountIdToBech32',
            data: {
                accountId,
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
     * Convert a hex-encoded public key to a Bech32-encoded address string.
     *
     * @param hex A hex-encoded public key.
     * @param bech32Hrp The Bech32 HRP (human readable part) to use.
     * @returns The Bech32-encoded address string.
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
     * Derives the `SlotCommitmentId` of the `SlotCommitment`.
     */
    static computeSlotCommitmentId(
        slotCommitment: SlotCommitment,
    ): SlotCommitmentId {
        return callUtilsMethod({
            name: 'computeSlotCommitmentId',
            data: {
                slotCommitment: {
                    index: slotCommitment.index.toString(10),
                    prevId: slotCommitment.prevId,
                    rootsId: slotCommitment.rootsId,
                    cumulativeWeight:
                        slotCommitment.cumulativeWeight.toString(10),
                },
            },
        });
    }
}
