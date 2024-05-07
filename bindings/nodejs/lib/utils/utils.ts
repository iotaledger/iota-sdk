// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { callUtilsMethod } from '../bindings';
import {
    Address,
    HexEncodedString,
    Ed25519Signature,
    Transaction,
    SignedTransactionPayload,
    TransactionId,
    TokenSchemeType,
    Output,
    StorageScoreParameters,
    OutputId,
    u64,
    Block,
    ProtocolParameters,
    Bech32Address,
    InputSigningData,
    Unlock,
    DecayedMana,
    NumericString,
    Ed25519Address,
    WorkScoreParameters,
} from '../types';
import {
    AccountId,
    BlockId,
    DelegationId,
    FoundryId,
    NftId,
    TokenId,
} from '../types/block/id';
import {
    SlotCommitment,
    SlotCommitmentId,
    SlotIndex,
} from '../types/block/slot';

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
    static computeAccountId(outputId: OutputId): AccountId {
        return callUtilsMethod({
            name: 'blake2b256Hash',
            data: {
                bytes: outputId,
            },
        });
    }

    /**
     * Compute the delegation ID from a given delegation output ID.
     *
     * @param outputId The output ID as hex-encoded string.
     * @returns The delegation ID.
     */
    static computeDelegationId(outputId: OutputId): DelegationId {
        return callUtilsMethod({
            name: 'blake2b256Hash',
            data: {
                bytes: outputId,
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
    static computeNftId(outputId: OutputId): NftId {
        return callUtilsMethod({
            name: 'blake2b256Hash',
            data: {
                bytes: outputId,
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
     * Compute the transaction ID from an output ID.
     *
     * @param outputId The output ID.
     * @returns The transaction ID of the transaction which created the output.
     */
    static transactionIdFromOutputId(outputId: OutputId): TransactionId {
        return outputId.slice(0, 74);
    }

    /**
     * Compute the output index from an output ID.
     *
     * @param outputId The output ID.
     * @returns The output index.
     */
    static outputIndexFromOutputId(outputId: OutputId): number {
        const numberString = outputId.slice(-4);
        const chunks = [];
        for (
            let i = 0, charsLength = numberString.length;
            i < charsLength;
            i += 2
        ) {
            chunks.push(numberString.substring(i, i + 2));
        }
        const separated = chunks.map((n) => parseInt(n, 16));
        const buf = Uint8Array.from(separated).buffer;
        const view = new DataView(buf);

        return view.getUint16(0, true);
    }

    /**
     * Compute the required storage deposit of an output.
     *
     * @param output The output.
     * @param storageScoreParameters Storage score of objects which take node resources.
     * @returns The required storage deposit.
     */
    static computeMinimumOutputAmount(
        output: Output,
        storageScoreParameters: StorageScoreParameters,
    ): u64 {
        const minStorageDepositAmount = callUtilsMethod({
            name: 'computeMinimumOutputAmount',
            data: {
                output,
                storageScoreParameters,
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
     * @param protocolParameters The network protocol parameters.
     * @returns The corresponding block ID.
     */
    static blockId(
        block: Block,
        protocolParameters: ProtocolParameters,
    ): BlockId {
        return callUtilsMethod({
            name: 'blockId',
            data: {
                block,
                protocolParameters,
            },
        });
    }

    /**
     * Compute the transaction ID (Blake2b256 hash of the provided transaction payload) of a transaction payload.
     *
     * @param payload A transaction payload.
     * @returns The transaction ID.
     */
    static transactionId(payload: SignedTransactionPayload): TransactionId {
        return callUtilsMethod({
            name: 'transactionId',
            data: {
                payload,
            },
        });
    }

    /**
     * Converts an address to its bech32 representation.
     *
     * @param address An address.
     * @param bech32Hrp The Bech32 HRP (human readable part) to use.
     * @returns The Bech32-encoded address string.
     */
    static addressToBech32(address: Address, bech32Hrp: string): Bech32Address {
        return callUtilsMethod({
            name: 'addressToBech32',
            data: {
                address,
                bech32Hrp,
            },
        });
    }

    /**
     * Hashes a hex encoded public key with Blake2b256.
     *
     * @param hex The hexadecimal string representation of a public key.
     * @returns The Ed25519 address with the hashed public key.
     */
    static publicKeyHash(hex: HexEncodedString): Ed25519Address {
        return new Ed25519Address(
            callUtilsMethod({
                name: 'blake2b256Hash',
                data: {
                    bytes: hex,
                },
            }),
        );
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
     * Compute the hash of an instance of ProtocolParameters.
     *
     * @param protocolParameters A ProtocolParameters instance.
     * @returns The hash of the protocol parameters as a hex-encoded string.
     */
    static protocolParametersHash(
        protocolParameters: ProtocolParameters,
    ): HexEncodedString {
        return callUtilsMethod({
            name: 'protocolParametersHash',
            data: {
                protocolParameters,
            },
        });
    }

    /**
     * Compute the signing hash of a transaction.
     *
     * @param transaction A transaction.
     * @returns The signing hash of the transaction as a hex-encoded string.
     */
    static transactionSigningHash(transaction: Transaction): HexEncodedString {
        return callUtilsMethod({
            name: 'transactionSigningHash',
            data: {
                transaction,
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
     * Computes a slotIndex from a block, transaction or slotCommitment Id.
     * @param id The block, transaction or slotCommitment Id.
     * @returns The slotIndex.
     */
    static computeSlotIndex(
        id: BlockId | SlotCommitmentId | TransactionId,
    ): SlotIndex {
        const numberString = id.slice(-8);
        const chunks = [];

        for (
            let i = 0, charsLength = numberString.length;
            i < charsLength;
            i += 2
        ) {
            chunks.push(numberString.substring(i, i + 2));
        }
        const separated = chunks.map((n) => parseInt(n, 16));
        const buf = Uint8Array.from(separated).buffer;
        const view = new DataView(buf);

        return view.getUint32(0, true);
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
                    protocolVersion: slotCommitment.protocolVersion,
                    slot: slotCommitment.slot,
                    previousCommitmentId: slotCommitment.previousCommitmentId,
                    rootsId: slotCommitment.rootsId,
                    cumulativeWeight:
                        slotCommitment.cumulativeWeight.toString(10),
                    referenceManaCost:
                        slotCommitment.referenceManaCost.toString(10),
                },
            },
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
     * @param transaction The transaction payload.
     * @param inputs The inputs data.
     * @param protocolParameters The protocol parameters.
     * @param unlocks The unlocks.
     * @param manaRewards The total mana rewards claimed in the transaction.
     *
     * @returns void.
     */
    static verifyTransactionSemantic(
        transaction: SignedTransactionPayload,
        inputs: InputSigningData[],
        protocolParameters: ProtocolParameters,
        unlocks?: Unlock[],
        manaRewards?: { [outputId: HexEncodedString]: NumericString },
    ): void {
        return callUtilsMethod({
            name: 'verifyTransactionSemantic',
            data: {
                transaction,
                inputs,
                protocolParameters,
                unlocks,
                manaRewards,
            },
        });
    }

    /**
     * Applies mana decay to the given mana.
     *
     * @param mana The mana to decay.
     * @param slotIndexCreated The slot index at which the provided mana starts to decay.
     * @param slotIndexTarget The slot index at which the provided mana stops to decay.
     * @param protocolParameters The ProtocolParameters.
     *
     * @returns The decayed mana.
     */
    static manaWithDecay(
        mana: u64,
        slotIndexCreated: number,
        slotIndexTarget: number,
        protocolParameters: ProtocolParameters,
    ): u64 {
        const decayedMana = callUtilsMethod({
            name: 'manaWithDecay',
            data: {
                mana: mana.toString(10),
                slotIndexCreated,
                slotIndexTarget,
                protocolParameters,
            },
        });
        return BigInt(decayedMana);
    }

    /**
     * Calculates the potential mana that is generated by holding `amount` tokens from `slotIndexCreated` to
     * `slotIndexTarget` and applies the decay to the result.
     *
     * @param amount The amount that generates mana.
     * @param slotIndexCreated The slot index at which the provided amount starts to generate mana.
     * @param slotIndexTarget The slot index at which the provided mana stops to generate mana.
     * @param protocolParameters The ProtocolParameters.
     *
     * @returns The decayed potential mana.
     */
    static generateManaWithDecay(
        amount: u64,
        slotIndexCreated: number,
        slotIndexTarget: number,
        protocolParameters: ProtocolParameters,
    ): u64 {
        const decayedMana = callUtilsMethod({
            name: 'generateManaWithDecay',
            data: {
                amount: amount.toString(10),
                slotIndexCreated,
                slotIndexTarget,
                protocolParameters,
            },
        });
        return BigInt(decayedMana);
    }

    /**
     * Applies mana decay to the output mana and calculates the potential mana that is generated.
     *
     * @param output The output.
     * @param slotIndexCreated The slot index at which the provided output was created.
     * @param slotIndexTarget The slot index for which to calculate the mana values.
     * @param protocolParameters The ProtocolParameters.
     *
     * @returns The decayed stored and potential mana.
     */
    static outputManaWithDecay(
        output: Output,
        slotIndexCreated: number,
        slotIndexTarget: number,
        protocolParameters: ProtocolParameters,
    ): DecayedMana {
        const decayedMana = callUtilsMethod({
            name: 'outputManaWithDecay',
            data: {
                output,
                slotIndexCreated,
                slotIndexTarget,
                protocolParameters,
            },
        });
        return {
            stored: BigInt(decayedMana.stored),
            potential: BigInt(decayedMana.potential),
        };
    }

    /**
     * Verifies the syntax of a transaction.
     *
     * @param transaction The transaction payload.
     * @param protocolParameters The protocol parameters used for the validation.
     * @returns void.
     */
    static verifyTransactionSyntax(
        transaction: SignedTransactionPayload,
        protocolParameters: ProtocolParameters,
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

    static iotaMainnetProtocolParameters(): ProtocolParameters {
        const params = callUtilsMethod({
            name: 'iotaMainnetProtocolParameters',
        });
        return params;
    }

    static shimmerMainnetProtocolParameters(): ProtocolParameters {
        const params = callUtilsMethod({
            name: 'shimmerMainnetProtocolParameters',
        });
        return params;
    }

    /**
     * Returns the work score of a block.
     *
     * @param block The block.
     * @param workScoreParameters The WorkScoreParameters.
     * @returns The work score of the block.
     */
    static blockWorkScore(
        block: Block,
        workScoreParameters: WorkScoreParameters,
    ): number {
        return callUtilsMethod({
            name: 'blockWorkScore',
            data: {
                block,
                workScoreParameters,
            },
        });
    }
}
