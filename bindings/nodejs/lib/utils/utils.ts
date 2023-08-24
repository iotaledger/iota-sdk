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
    hexToBigInt,
    u64,
} from '../types';
import { AccountId, BlockId, FoundryId, NftId, TokenId } from '../types/block/id';
import { SlotCommitment, SlotCommitmentId } from '../types/block/slot';

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
     * Computes the account id for the given account output id.
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
     * Computes the foundry id.
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
     * Computes the NFT id for the given NFT output id.
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
     * @param rent Rent cost of objects which take node resources.
     * @returns The required storage deposit.
     */
    static computeStorageDeposit(output: Output, rent: IRent): u64 {
        const depositHex = callUtilsMethod({
            name: 'computeStorageDeposit',
            data: {
                output,
                rent,
            },
        });
        return hexToBigInt(depositHex);
    }

    /**
     * Computes a tokenId from the accountId, serial number and token scheme type.
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
     * Transforms an account id to a bech32 encoded address.
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
