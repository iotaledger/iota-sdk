// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { ClientMethodHandler } from './client-method-handler';
import {
    IClientOptions,
    IGenerateAddressesOptions,
    IBuildBlockOptions,
    QueryParameter,
    PreparedTransactionData,
    INetworkInfo,
    INode,
    IAuth,
    BasicOutputBuilderParams,
    AliasOutputBuilderParams,
    FoundryOutputBuilderParams,
    NftOutputBuilderParams,
    FoundryQueryParameter,
    NftQueryParameter,
    AliasQueryParameter,
    GenericQueryParameter,
} from '../types/client';
import type { INodeInfoWrapper } from '../types/client/nodeInfo';
import {
    Bip44,
    SecretManagerType,
} from '../types/secret_manager/secret-manager';
import {
    AliasOutput,
    BasicOutput,
    FoundryOutput,
    NftOutput,
    Block,
    BlockId,
    UnlockCondition,
    Payload,
    TransactionPayload,
    MilestonePayload,
    TreasuryOutput,
    Output,
    MilestoneId,
    AliasId,
    NftId,
    FoundryId,
} from '../types/block';
import { HexEncodedString, NumericString } from '../utils';
import {
    IBlockMetadata,
    INodeInfo,
    INodeInfoProtocol,
    IPeer,
    UTXOInput,
    Response,
    OutputId,
    TransactionId,
    Bech32Address,
} from '../types';
import {
    IMilestoneUtxoChangesResponse,
    OutputResponse,
    ReceiptsResponse,
    IOutputsResponse,
} from '../types/models/api';

import { plainToInstance } from 'class-transformer';

/** The Client to interact with nodes. */
export class Client {
    private methodHandler: ClientMethodHandler;

    /**
     * @param options client options or a client method handler.
     */
    constructor(options: IClientOptions | ClientMethodHandler) {
        this.methodHandler = new ClientMethodHandler(options);
    }

    async destroy() {
        return this.methodHandler.destroy();
    }

    /**
     * Get the node information together with the url of the used node.
     */
    async getInfo(): Promise<INodeInfoWrapper> {
        const response = await this.methodHandler.callMethod({
            name: 'getInfo',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get network related information such as protocol parameters and minimum pow score.
     */
    async getNetworkInfo(): Promise<INetworkInfo> {
        const response = await this.methodHandler.callMethod({
            name: 'getNetworkInfo',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Fetch alias/basic/NFT/foundry output IDs based on the given query parameters.
     */
    async outputIds(
        queryParameters: GenericQueryParameter[],
    ): Promise<IOutputsResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'outputIds',
            data: {
                queryParameters,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Fetch basic output IDs based on the given query parameters.
     */
    async basicOutputIds(
        queryParameters: QueryParameter[],
    ): Promise<IOutputsResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'basicOutputIds',
            data: {
                queryParameters,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get output from a given output ID.
     */
    async getOutput(outputId: OutputId): Promise<OutputResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getOutput',
            data: {
                outputId,
            },
        });

        const parsed = JSON.parse(response) as Response<OutputResponse>;
        return plainToInstance(OutputResponse, parsed.payload);
    }

    /**
     * Fetch OutputResponse from given output IDs. Requests are sent in parallel.
     */
    async getOutputs(outputIds: OutputId[]): Promise<OutputResponse[]> {
        const response = await this.methodHandler.callMethod({
            name: 'getOutputs',
            data: {
                outputIds,
            },
        });

        const parsed = JSON.parse(response) as Response<OutputResponse[]>;
        return plainToInstance(OutputResponse, parsed.payload);
    }

    /**
     * Build and post a block.
     *
     * @param secretManager One of the supported secret managers.
     * @param options Options on how to build a block.
     * @returns The block ID and the posted block itself.
     */
    async buildAndPostBlock(
        secretManager?: SecretManagerType,
        options?: IBuildBlockOptions,
    ): Promise<[BlockId, Block]> {
        if (
            options &&
            options.output &&
            typeof options.output.amount === 'bigint'
        ) {
            options.output.amount = options.output.amount.toString(10);
        }
        if (
            options &&
            options.outputHex &&
            typeof options.outputHex.amount === 'bigint'
        ) {
            options.outputHex.amount = options.outputHex.amount.toString(10);
        }
        const response = await this.methodHandler.callMethod({
            name: 'buildAndPostBlock',
            data: {
                secretManager,
                options,
            },
        });
        const parsed = JSON.parse(response) as Response<[BlockId, Block]>;
        const block = plainToInstance(Block, parsed.payload[1]);
        return [parsed.payload[0], block];
    }

    /**
     * Request tips from the node.
     * The tips can be considered as non-lazy and are therefore ideal for attaching a block to the Tangle.
     * @returns An array of tips represented by their block IDs.
     */
    async getTips(): Promise<BlockId[]> {
        const response = await this.methodHandler.callMethod({
            name: 'getTips',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Post a block in JSON format.
     *
     * @param block The block to post.
     * @returns The block ID once the block has been posted.
     */
    async postBlock(block: Block): Promise<BlockId> {
        const response = await this.methodHandler.callMethod({
            name: 'postBlock',
            data: {
                block,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get a block in JSON format.
     *
     * @param blockId The corresponding block ID of the requested block.
     * @returns The requested block.
     */
    async getBlock(blockId: BlockId): Promise<Block> {
        const response = await this.methodHandler.callMethod({
            name: 'getBlock',
            data: {
                blockId,
            },
        });

        const parsed = JSON.parse(response) as Response<Block>;
        return plainToInstance(Block, parsed.payload);
    }

    /**
     * Get the metadata of a block.
     *
     * @param blockId The corresponding block ID of the requested block metadata.
     * @returns The requested block metadata.
     */
    async getBlockMetadata(blockId: BlockId): Promise<IBlockMetadata> {
        const response = await this.methodHandler.callMethod({
            name: 'getBlockMetadata',
            data: {
                blockId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Find inputs from addresses for a given amount (useful for offline signing).
     *
     * @param addresses A list of included addresses.
     * @param amount The amount to find inputs for.
     * @returns An array of UTXO inputs.
     */
    async findInputs(
        addresses: string[],
        amount: bigint,
    ): Promise<UTXOInput[]> {
        const response = await this.methodHandler.callMethod({
            name: 'findInputs',
            data: {
                addresses,
                amount: Number(amount),
            },
        });

        const parsed = JSON.parse(response) as Response<UTXOInput[]>;
        return plainToInstance(UTXOInput, parsed.payload);
    }

    /**
     * Prepare a transaction for signing.
     *
     * @param secretManager One of the supported secret managers.
     * @param options Options to build a block.
     * @returns An instance of `PreparedTransactionData`.
     */
    async prepareTransaction(
        secretManager?: SecretManagerType,
        options?: IBuildBlockOptions,
    ): Promise<PreparedTransactionData> {
        const response = await this.methodHandler.callMethod({
            name: 'prepareTransaction',
            data: {
                secretManager,
                options,
            },
        });

        const parsed = JSON.parse(
            response,
        ) as Response<PreparedTransactionData>;
        return plainToInstance(PreparedTransactionData, parsed.payload);
    }

    /**
     * Sign a transaction.
     *
     * @param secretManager One of the supported secret managers.
     * @param preparedTransactionData An instance of `PreparedTransactionData`.
     * @returns The corresponding transaction payload.
     */
    async signTransaction(
        secretManager: SecretManagerType,
        preparedTransactionData: PreparedTransactionData,
    ): Promise<TransactionPayload> {
        const response = await this.methodHandler.callMethod({
            name: 'signTransaction',
            data: {
                secretManager,
                preparedTransactionData,
            },
        });

        const parsed = JSON.parse(response) as Response<TransactionPayload>;
        return plainToInstance(TransactionPayload, parsed.payload);
    }

    /**
     * Create a signature unlock using the given secret manager.
     *
     * @param secretManager One of the supported secret managers.
     * @param transactionEssenceHash The hash of the transaction essence.
     * @param chain A BIP44 chain
     * @returns The corresponding unlock condition.
     */
    async signatureUnlock(
        secretManager: SecretManagerType,
        transactionEssenceHash: HexEncodedString,
        chain: Bip44,
    ): Promise<UnlockCondition> {
        const response = await this.methodHandler.callMethod({
            name: 'signatureUnlock',
            data: {
                secretManager,
                transactionEssenceHash,
                chain,
            },
        });

        return UnlockCondition.parse(JSON.parse(response).payload);
    }

    /**
     * Submit a payload in a block.
     *
     * @param payload The payload to post.
     * @returns The block ID followed by the block containing the payload.
     */
    async postBlockPayload(payload: Payload): Promise<[BlockId, Block]> {
        const response = await this.methodHandler.callMethod({
            name: 'postBlockPayload',
            data: {
                payload,
            },
        });
        const parsed = JSON.parse(response) as Response<[BlockId, Block]>;
        const block = plainToInstance(Block, parsed.payload[1]);
        return [parsed.payload[0], block];
    }

    /**
     * Get a node candidate from the healthy node pool.
     */
    async getNode(): Promise<INode> {
        const response = await this.methodHandler.callMethod({
            name: 'getNode',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the ID of the network the node is connected to.
     */
    async getNetworkId(): Promise<string> {
        const response = await this.methodHandler.callMethod({
            name: 'getNetworkId',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the Bech32 HRP (human readable part) of the network the node is connected to.
     */
    async getBech32Hrp(): Promise<string> {
        const response = await this.methodHandler.callMethod({
            name: 'getBech32Hrp',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the minimum PoW score.
     */
    async getMinPowScore(): Promise<number> {
        const response = await this.methodHandler.callMethod({
            name: 'getMinPowScore',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the tips interval.
     */
    async getTipsInterval(): Promise<number> {
        const response = await this.methodHandler.callMethod({
            name: 'getTipsInterval',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the token supply.
     */
    async getTokenSupply(): Promise<NumericString> {
        return (await this.getProtocolParameters()).tokenSupply;
    }

    /**
     * Get the protocol parameters.
     */
    async getProtocolParameters(): Promise<INodeInfoProtocol> {
        const response = await this.methodHandler.callMethod({
            name: 'getProtocolParameters',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Check whether local pow should be used or not.
     */
    async getLocalPow(): Promise<boolean> {
        const response = await this.methodHandler.callMethod({
            name: 'getLocalPow',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Check whether to fallback to local proof of work in case the node doesn't support remote PoW.
     */
    async getFallbackToLocalPow(): Promise<boolean> {
        const response = await this.methodHandler.callMethod({
            name: 'getFallbackToLocalPow',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the health of a node.
     *
     * @param url The URL of the node.
     */
    async getHealth(url: string): Promise<boolean> {
        const response = await this.methodHandler.callMethod({
            name: 'getHealth',
            data: {
                url,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the info about the node.
     *
     * @param url The URL of the node.
     * @param auth An authentication object (e.g. JWT).
     */
    async getNodeInfo(url: string, auth?: IAuth): Promise<INodeInfo> {
        const response = await this.methodHandler.callMethod({
            name: 'getNodeInfo',
            data: {
                url,
                auth,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the peers of the node.
     */
    async getPeers(): Promise<IPeer[]> {
        const response = await this.methodHandler.callMethod({
            name: 'getPeers',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Post block as raw bytes, returns the block ID.
     *
     * @param block The block.
     * @returns The ID of the posted block.
     */
    async postBlockRaw(block: Block): Promise<BlockId> {
        const response = await this.methodHandler.callMethod({
            name: 'postBlockRaw',
            data: {
                block,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get block as raw bytes.
     *
     * @param blockId The block ID of the requested block.
     * @returns The raw bytes of the requested block.
     */
    async getBlockRaw(blockId: BlockId): Promise<Uint8Array> {
        const response = await this.methodHandler.callMethod({
            name: 'getBlockRaw',
            data: {
                blockId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get a milestone payload by its ID.
     *
     * @param milestoneId The ID of the requested milestone.
     * @returns The corresponding milestone payload.
     */
    async getMilestoneById(milestoneId: string): Promise<MilestonePayload> {
        const response = await this.methodHandler.callMethod({
            name: 'getMilestoneById',
            data: {
                milestoneId,
            },
        });
        const parsed = JSON.parse(response) as Response<MilestonePayload>;
        return plainToInstance(MilestonePayload, parsed.payload);
    }

    /**
     * Get all UTXO changes of a milestone by its ID.
     *
     * @param milestoneId The ID of the milestone that applied the UTXO changes.
     * @returns A milestone UTXO changes response.
     */
    async getUtxoChangesById(
        milestoneId: MilestoneId,
    ): Promise<IMilestoneUtxoChangesResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getUtxoChangesById',
            data: {
                milestoneId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get a milestone payload by its index.
     *
     * @param index The index of the requested milestone.
     * @returns The corresponding milestone payload.
     */
    async getMilestoneByIndex(index: number): Promise<MilestonePayload> {
        const response = await this.methodHandler.callMethod({
            name: 'getMilestoneByIndex',
            data: {
                index,
            },
        });
        const parsed = JSON.parse(response) as Response<MilestonePayload>;
        return plainToInstance(MilestonePayload, parsed.payload);
    }

    /**
     * Get all UTXO changes of a milestone by its index.
     *
     * @param index The index of the milestone that applied the UTXO changes.
     * @returns A milestone UTXO changes response.
     */
    async getUtxoChangesByIndex(
        index: number,
    ): Promise<IMilestoneUtxoChangesResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getUtxoChangesByIndex',
            data: {
                index,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get all receipts.
     */
    async getReceipts(): Promise<ReceiptsResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getReceipts',
        });
        const parsed = JSON.parse(response) as Response<ReceiptsResponse>;
        return plainToInstance(ReceiptsResponse, parsed.payload);
    }

    /**
     * Get the receipts at a given milestone index.
     *
     * @param milestoneIndex The index of the milestone that migrated funds to the new network.
     */
    async getReceiptsMigratedAt(
        milestoneIndex: number,
    ): Promise<ReceiptsResponse[]> {
        const response = await this.methodHandler.callMethod({
            name: 'getReceiptsMigratedAt',
            data: {
                milestoneIndex,
            },
        });
        const parsed = JSON.parse(response) as Response<ReceiptsResponse[]>;
        return plainToInstance(ReceiptsResponse, parsed.payload);
    }

    /**
     * Get the treasury output.
     */
    async getTreasury(): Promise<TreasuryOutput> {
        const response = await this.methodHandler.callMethod({
            name: 'getTreasury',
        });

        return Output.parse(JSON.parse(response).payload) as TreasuryOutput;
    }

    /**
     * Get the included block of a given transaction.
     *
     * @param transactionId The ID of the transaction.
     * @returns The included block that contained the transaction.
     */
    async getIncludedBlock(transactionId: TransactionId): Promise<Block> {
        const response = await this.methodHandler.callMethod({
            name: 'getIncludedBlock',
            data: {
                transactionId,
            },
        });
        const parsed = JSON.parse(response) as Response<Block>;
        return plainToInstance(Block, parsed.payload);
    }

    /**
     * Get the metadata of the included block of a given transaction.
     *
     * @param transactionId The ID of the transaction.
     * @returns The included block that contained the transaction.
     */
    async getIncludedBlockMetadata(
        transactionId: TransactionId,
    ): Promise<Block> {
        const response = await this.methodHandler.callMethod({
            name: 'getIncludedBlockMetadata',
            data: {
                transactionId,
            },
        });
        const parsed = JSON.parse(response) as Response<Block>;
        return plainToInstance(Block, parsed.payload);
    }

    /**
     * Convert a hex encoded address to a Bech32 encoded address.
     *
     * @param hex The hexadecimal string representation of an address.
     * @param bech32Hrp The Bech32 HRP (human readable part) to be used.
     * @returns The corresponding Bech32 address.
     */
    async hexToBech32(
        hex: HexEncodedString,
        bech32Hrp?: string,
    ): Promise<Bech32Address> {
        const response = await this.methodHandler.callMethod({
            name: 'hexToBech32',
            data: {
                hex,
                bech32Hrp,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Convert an Alias ID to a Bech32 encoded address.
     *
     * @param aliasId An Alias ID.
     * @param bech32Hrp The Bech32 HRP (human readable part) to be used.
     * @returns The corresponding Bech32 address.
     */
    async aliasIdToBech32(
        aliasId: AliasId,
        bech32Hrp?: string,
    ): Promise<Bech32Address> {
        const response = await this.methodHandler.callMethod({
            name: 'aliasIdToBech32',
            data: {
                aliasId,
                bech32Hrp,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Convert an NFT ID to a Bech32 encoded address.
     *
     * @param nftId An NFT ID.
     * @param bech32Hrp The Bech32 HRP (human readable part) to be used.
     * @returns The corresponding Bech32 address.
     */
    async nftIdToBech32(
        nftId: NftId,
        bech32Hrp?: string,
    ): Promise<Bech32Address> {
        const response = await this.methodHandler.callMethod({
            name: 'nftIdToBech32',
            data: {
                nftId,
                bech32Hrp,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Convert a hex encoded public key to a Bech32 encoded address.
     *
     * @param hex The hexadecimal string representation of a public key.
     * @param bech32Hrp The Bech32 HRP (human readable part) to be used.
     * @returns The corresponding Bech32 address.
     */
    async hexPublicKeyToBech32Address(
        hex: HexEncodedString,
        bech32Hrp?: string,
    ): Promise<Bech32Address> {
        const response = await this.methodHandler.callMethod({
            name: 'hexPublicKeyToBech32Address',
            data: {
                hex,
                bech32Hrp,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the corresponding output IDs given a list of Alias query parameters.
     *
     * @param queryParameters An array of `AliasQueryParameter`s.
     * @returns A paginated query response of corresponding output IDs.
     */
    async aliasOutputIds(
        queryParameters: AliasQueryParameter[],
    ): Promise<IOutputsResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'aliasOutputIds',
            data: {
                queryParameters,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the corresponding output ID from an Alias ID.
     *
     * @param aliasId An Alias ID.
     * @returns The corresponding output ID.
     */
    async aliasOutputId(aliasId: AliasId): Promise<OutputId> {
        const response = await this.methodHandler.callMethod({
            name: 'aliasOutputId',
            data: {
                aliasId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the corresponding output IDs given a list of NFT query parameters.
     *
     * @param queryParameters An array of `NftQueryParameter`s.
     * @returns A paginated query response of corresponding output IDs.
     */
    async nftOutputIds(
        queryParameters: NftQueryParameter[],
    ): Promise<IOutputsResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'nftOutputIds',
            data: {
                queryParameters,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the corresponding output ID from an NFT ID.
     *
     * @param nftId An NFT ID.
     * @returns The corresponding output ID.
     */
    async nftOutputId(nftId: NftId): Promise<OutputId> {
        const response = await this.methodHandler.callMethod({
            name: 'nftOutputId',
            data: {
                nftId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the corresponding output IDs given a list of Foundry query parameters.
     *
     * @param queryParameters An array of `FoundryQueryParameter`s.
     * @returns A paginated query response of corresponding output IDs.
     */
    async foundryOutputIds(
        queryParameters: FoundryQueryParameter[],
    ): Promise<IOutputsResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'foundryOutputIds',
            data: {
                queryParameters,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the corresponding output ID from a Foundry ID.
     *
     * @param foundryId A Foundry ID.
     * @returns The corresponding output ID.
     */
    async foundryOutputId(foundryId: FoundryId): Promise<OutputId> {
        const response = await this.methodHandler.callMethod({
            name: 'foundryOutputId',
            data: {
                foundryId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get outputs from provided output IDs (requests are sent
     * in parallel and errors are ignored, can be useful for spent outputs)
     *
     * @param outputIds An array of output IDs.
     * @returns An array of corresponding output responses.
     */
    async getOutputsIgnoreErrors(
        outputIds: OutputId[],
    ): Promise<OutputResponse[]> {
        const response = await this.methodHandler.callMethod({
            name: 'getOutputsIgnoreErrors',
            data: {
                outputIds,
            },
        });
        const parsed = JSON.parse(response) as Response<OutputResponse[]>;
        return plainToInstance(OutputResponse, parsed.payload);
    }

    /**
     * Find blocks by their IDs.
     *
     * @param blockIds An array of `BlockId`s.
     * @returns An array of corresponding blocks.
     */
    async findBlocks(blockIds: BlockId[]): Promise<Block[]> {
        const response = await this.methodHandler.callMethod({
            name: 'findBlocks',
            data: {
                blockIds,
            },
        });
        const parsed = JSON.parse(response) as Response<Block[]>;
        return plainToInstance(Block, parsed.payload);
    }

    /**
     * Retry (promote or reattach) a block given its block ID.
     *
     * **Note**: Blocks should be retried only if they are valid and haven't been confirmed for some time.
     *
     * @param blockId The ID of the block to retry.
     * @returns The included block.
     */
    async retry(blockId: BlockId): Promise<[BlockId, Block]> {
        const response = await this.methodHandler.callMethod({
            name: 'retry',
            data: {
                blockId,
            },
        });
        const parsed = JSON.parse(response) as Response<[BlockId, Block]>;
        const block = plainToInstance(Block, parsed.payload[1]);
        return [parsed.payload[0], block];
    }

    /**
     * Retry (promote or reattach) a block given its block ID until it's included
     * (i.e. referenced by a milestone).
     *
     * @param blockId The ID of the block to retry.
     * @param interval A retry interval in seconds. Defaults to 5.
     * @param maxAttempts A maximum number of retries. Defaults to 40.
     * @returns The included block at first position and additional reattached blocks.
     */
    async retryUntilIncluded(
        blockId: BlockId,
        interval?: number,
        maxAttempts?: number,
    ): Promise<[BlockId, Block][]> {
        const response = await this.methodHandler.callMethod({
            name: 'retryUntilIncluded',
            data: {
                blockId,
                interval,
                maxAttempts,
            },
        });
        const parsed = JSON.parse(response) as Response<[BlockId, Block][]>;
        const arr: [BlockId, Block][] = [];
        parsed.payload.forEach((payload) => {
            arr.push([payload[0], plainToInstance(Block, payload[1])]);
        });

        return arr;
    }

    /**
     * Consolidate all funds from a range of addresses to the address with the lowest index in that range.
     *
     * @param secretManager One of supported secret managers.
     * @param generateAddressesOptions Options for generating addresses.
     * @returns The address to which the funds got consolidated, if any were available.
     */
    async consolidateFunds(
        secretManager: SecretManagerType,
        generateAddressesOptions: IGenerateAddressesOptions,
    ): Promise<string> {
        const response = await this.methodHandler.callMethod({
            name: 'consolidateFunds',
            data: {
                secretManager,
                generateAddressesOptions,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Reattach a block.
     *
     * **Note**: Blocks can be reattached only if they are valid and haven't been confirmed for some time.
     *
     * @param blockId The ID of the block to reattach.
     * @returns The included block.
     */
    async reattach(blockId: BlockId): Promise<[BlockId, Block]> {
        const response = await this.methodHandler.callMethod({
            name: 'reattach',
            data: {
                blockId,
            },
        });
        const parsed = JSON.parse(response) as Response<[BlockId, Block]>;
        const block = plainToInstance(Block, parsed.payload[1]);
        return [parsed.payload[0], block];
    }

    /**
     * Reattach a block without checking whether it should be reattached.
     *
     * @param blockId The ID of the block to reattach.
     * @returns The included block.
     */
    async reattachUnchecked(blockId: BlockId): Promise<[BlockId, Block]> {
        const response = await this.methodHandler.callMethod({
            name: 'reattachUnchecked',
            data: {
                blockId,
            },
        });
        const parsed = JSON.parse(response) as Response<[BlockId, Block]>;
        const block = plainToInstance(Block, parsed.payload[1]);
        return [parsed.payload[0], block];
    }

    /**
     * Promote a block.
     *
     * **NOTE**: The method validates whether a promotion is necessary through `get_block`. If not, the
     * method will error out and will not do unnecessary promotions.
     *
     * @param blockId The ID of the block to promote.
     * @returns The included block.
     */
    async promote(blockId: BlockId): Promise<[BlockId, Block]> {
        const response = await this.methodHandler.callMethod({
            name: 'promote',
            data: {
                blockId,
            },
        });
        const parsed = JSON.parse(response) as Response<[BlockId, Block]>;
        const block = plainToInstance(Block, parsed.payload[1]);
        return [parsed.payload[0], block];
    }
    /**
     * Promote a block without checking if it should be promoted.
     *
     * @param blockId The ID of the block to promote.
     * @returns The included block.
     */
    async promoteUnchecked(blockId: BlockId): Promise<[BlockId, Block]> {
        const response = await this.methodHandler.callMethod({
            name: 'promoteUnchecked',
            data: {
                blockId,
            },
        });
        const parsed = JSON.parse(response) as Response<[BlockId, Block]>;
        const block = plainToInstance(Block, parsed.payload[1]);
        return [parsed.payload[0], block];
    }

    /**
     * Return the unhealthy nodes.
     */
    async unhealthyNodes(): Promise<Set<INode>> {
        const response = await this.methodHandler.callMethod({
            name: 'unhealthyNodes',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Build a basic output.
     *
     * @param params An instance of `BasicOutputBuilderParams`.
     */
    async buildBasicOutput(
        params: BasicOutputBuilderParams,
    ): Promise<BasicOutput> {
        if (params.amount && typeof params.amount === 'bigint') {
            params.amount = params.amount.toString(10);
        }
        const response = await this.methodHandler.callMethod({
            name: 'buildBasicOutput',
            data: params,
        });

        const parsed = JSON.parse(response) as Response<BasicOutput>;
        return plainToInstance(BasicOutput, parsed.payload);
    }

    /**
     * Build an alias output.
     *
     * @param params An instance of `AliasOutputBuilderParams`.
     */
    async buildAliasOutput(
        params: AliasOutputBuilderParams,
    ): Promise<AliasOutput> {
        if (params.amount && typeof params.amount === 'bigint') {
            params.amount = params.amount.toString(10);
        }
        const response = await this.methodHandler.callMethod({
            name: 'buildAliasOutput',
            data: params,
        });

        const parsed = JSON.parse(response) as Response<AliasOutput>;
        return plainToInstance(AliasOutput, parsed.payload);
    }

    /**
     * Build a foundry output.
     *
     * @param params An instance of `FoundryOutputBuilderParams`.
     */
    async buildFoundryOutput(
        params: FoundryOutputBuilderParams,
    ): Promise<FoundryOutput> {
        if (params.amount && typeof params.amount === 'bigint') {
            params.amount = params.amount.toString(10);
        }
        const response = await this.methodHandler.callMethod({
            name: 'buildFoundryOutput',
            data: params,
        });

        const parsed = JSON.parse(response) as Response<FoundryOutput>;
        return plainToInstance(FoundryOutput, parsed.payload);
    }

    /**
     * Build an NFT output.
     *
     * @param params An instance of `NftOutputBuilderParams`.
     */
    async buildNftOutput(params: NftOutputBuilderParams): Promise<NftOutput> {
        if (params.amount && typeof params.amount === 'bigint') {
            params.amount = params.amount.toString(10);
        }
        const response = await this.methodHandler.callMethod({
            name: 'buildNftOutput',
            data: params,
        });

        const parsed = JSON.parse(response) as Response<NftOutput>;
        return plainToInstance(NftOutput, parsed.payload);
    }

    /**
     * Listen to MQTT events.
     *
     * @param topics An array of MQTT topics to listen to.
     */
    async listenMqtt(
        topics: string[],
        callback: (error: Error, result: string) => void,
    ): Promise<void> {
        return this.methodHandler.listen(topics, callback);
    }

    /**
     * Stop listening to certain MQTT events.
     *
     * @param topics An array of MQTT topics to stop listening to.
     */
    async clearMqttListeners(topics: string[]): Promise<void> {
        await this.methodHandler.callMethod({
            name: 'clearListeners',
            data: {
                topics,
            },
        });
    }

    /**
     * Calculate the minimum required storage deposit for an output.
     *
     * @param output The output to calculate the minimum deposit amount for.
     * @returns The minimum required amount.
     */
    async minimumRequiredStorageDeposit(output: Output): Promise<number> {
        const response = await this.methodHandler.callMethod({
            name: 'minimumRequiredStorageDeposit',
            data: {
                output,
            },
        });
        return JSON.parse(response).payload;
    }

    /**
     * Request funds from a faucet.
     *
     * Example URLs: `https://faucet.testnet.shimmer.network/api/enqueue` or `http://localhost:8091/api/enqueue`.
     *
     * @param url The URL of the faucet.
     * @param address The address to send the funds to.
     * @returns The faucet response.
     */
    async requestFundsFromFaucet(
        url: string,
        address: Bech32Address,
    ): Promise<string> {
        const response = await this.methodHandler.callMethod({
            name: 'requestFundsFromFaucet',
            data: { url, address },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Extension method which provides request methods for plugins.
     *
     * @param basePluginPath The base path for the plugin eg indexer/v1/ .
     * @param method The http method.
     * @param endpoint The path for the plugin request.
     * @param queryParams Additional query params for the request.
     * @param request The request object.
     * @returns The response json.
     */
    async callPluginRoute(
        basePluginPath: string,
        method: 'GET' | 'POST',
        endpoint: string,
        queryParams?: string[],
        request?: string,
    ): Promise<string> {
        const response = await this.methodHandler.callMethod({
            name: 'callPluginRoute',
            data: {
                basePluginPath,
                method,
                endpoint,
                queryParams: queryParams ?? [],
                request,
            },
        });

        return JSON.parse(response).payload;
    }
}
