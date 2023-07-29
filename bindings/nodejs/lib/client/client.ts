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
} from '../types/block';
import { HexEncodedString } from '../utils';
import {
    IBlockMetadata,
    INodeInfo,
    INodeInfoProtocol,
    IPeer,
    UTXOInput,
    Response,
    OutputId,
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

    /** TODO  */
    constructor(options: IClientOptions | ClientMethodHandler) {
        this.methodHandler = new ClientMethodHandler(options);
    }

    /** TODO  */
    async destroy() {
        return this.methodHandler.destroy();
    }

    /**
     * Returns the node information together with the url of the used node
     * @returns { Promise<INodeInfoWrapper> }.
     */
    async getInfo(): Promise<INodeInfoWrapper> {
        const response = await this.methodHandler.callMethod({
            name: 'getInfo',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Gets the network related information such as network_id and min_pow_score
     * @param TODO TODO.
     * @returns TODO.
     */
    async getNetworkInfo(): Promise<INetworkInfo> {
        const response = await this.methodHandler.callMethod({
            name: 'getNetworkInfo',
        });

        return JSON.parse(response).payload;
    }

    /** 
     * Fetch basic output IDs based on query parameters
     * @param TODO TODO.
     * @returns TODO.
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

    /** Get output from a known outputID 
     * @param TODO TODO.
     * @returns TODO.
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

    /** Fetch OutputResponse from provided OutputIds (requests are sent in parallel)
     * @param TODO TODO.
     * @returns TODO.
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

    /** Build and post a block
     * @param TODO TODO.
     * @returns TODO.
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
     * Returns tips that are ideal for attaching a block.
     * The tips can be considered as non-lazy and are therefore ideal for attaching a block.
     * @returns TODO.
     */
    async getTips(): Promise<BlockId[]> {
        const response = await this.methodHandler.callMethod({
            name: 'getTips',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Post block in JSON format, returns the block ID.
     * @param TODO TODO.
     * @returns TODO.
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
     * Get block as JSON.
     * @param TODO TODO.
     * @returns TODO.
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
     * Get block metadata.
     * @param TODO TODO.
     * @returns TODO.
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
     * Find inputs from addresses for a provided amount (useful for offline signing)
     * @param TODO TODO.
     * @returns TODO.
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
     * Prepare a transaction for signing
     * @param TODO TODO.
     * @returns TODO.
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
     * Sign a transaction
     * @param TODO TODO.
     * @returns TODO.
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
     * Create a signature unlock using the provided `secretManager`.
     * @param TODO TODO.
     * @returns TODO.
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
     * Submit a payload in a block
     * @param TODO TODO.
     * @returns TODO.
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
     * @param TODO TODO.
     * @returns TODO.
     */
    async getNode(): Promise<INode> {
        const response = await this.methodHandler.callMethod({
            name: 'getNode',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the network id of the node we're connecting to.
     * @param TODO TODO.
     * @returns TODO.
     */
    async getNetworkId(): Promise<number> {
        const response = await this.methodHandler.callMethod({
            name: 'getNetworkId',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns the bech32_hrp.
     * @param TODO TODO.
     * @returns TODO.
     */
    async getBech32Hrp(): Promise<string> {
        const response = await this.methodHandler.callMethod({
            name: 'getBech32Hrp',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns the min PoW score.
     * @param TODO TODO.
     * @returns TODO.
     */
    async getMinPowScore(): Promise<number> {
        const response = await this.methodHandler.callMethod({
            name: 'getMinPowScore',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns the tips interval.
     * @param TODO TODO.
     * @returns TODO.
     */
    async getTipsInterval(): Promise<number> {
        const response = await this.methodHandler.callMethod({
            name: 'getTipsInterval',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns the token supply.
     * @param TODO TODO.
     * @returns TODO.
     */
    async getTokenSupply(): Promise<string> {
        return (await this.getProtocolParameters()).tokenSupply;
    }

    /**
     * Returns the protocol parameters.
     * @param TODO TODO.
     * @returns TODO.
     */
    async getProtocolParameters(): Promise<INodeInfoProtocol> {
        const response = await this.methodHandler.callMethod({
            name: 'getProtocolParameters',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns if local pow should be used or not.
     * @param TODO TODO.
     * @returns TODO.
     */
    async getLocalPow(): Promise<boolean> {
        const response = await this.methodHandler.callMethod({
            name: 'getLocalPow',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get fallback to local proof of work timeout.
     * @param TODO TODO.
     * @returns TODO.
     */
    async getFallbackToLocalPow(): Promise<boolean> {
        const response = await this.methodHandler.callMethod({
            name: 'getFallbackToLocalPow',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get health of node by input url.
     * @param TODO TODO.
     * @returns TODO.
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
     * Get info of node with input url.
     * @param TODO TODO.
     * @returns TODO.
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
     * Get peers.
     * @param TODO TODO.
     * @returns TODO.
     */
    async getPeers(): Promise<IPeer[]> {
        const response = await this.methodHandler.callMethod({
            name: 'getPeers',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Post block as raw bytes, returns the block ID.
     * @param TODO TODO.
     * @returns TODO.
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
     * @param TODO TODO.
     * @returns TODO.
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
     * Look up a milestone by a given milestone index.
     * @param TODO TODO.
     * @returns TODO.
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
     * Returns all UTXO changes that happened at a specific milestone.
     * @param TODO TODO.
     * @returns TODO.
     */
    async getUtxoChangesById(
        milestoneId: string,
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
     * Look up a milestone by a given milestone index.
     * @param TODO TODO.
     * @returns TODO.
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
     * Returns all UTXO changes that happened at a specific milestone.
     * @param TODO TODO.
     * @returns TODO.
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
     * Get receipts.
     * @param TODO TODO.
     * @returns TODO.
     */
    async getReceipts(): Promise<ReceiptsResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getReceipts',
        });
        const parsed = JSON.parse(response) as Response<ReceiptsResponse>;
        return plainToInstance(ReceiptsResponse, parsed.payload);
    }

    /**
     * Get the receipts by the given milestone index.
     * @param TODO TODO.
     * @returns TODO.
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
     * @param TODO TODO.
     * @returns TODO.
     */
    async getTreasury(): Promise<TreasuryOutput> {
        const response = await this.methodHandler.callMethod({
            name: 'getTreasury',
        });

        return Output.parse(JSON.parse(response).payload) as TreasuryOutput;
    }

    /**
     * Returns the included block of the transaction.
     * @param TODO TODO.
     * @returns TODO.
     */
    async getIncludedBlock(transactionId: string): Promise<Block> {
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
     * Returns the metadata of the included block of the transaction.
     * @param TODO TODO.
     * @returns TODO.
     */
    async getIncludedBlockMetadata(transactionId: string): Promise<Block> {
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
     * Transforms a hex encoded address to a bech32 encoded address.
     * @param TODO TODO.
     * @returns TODO.
     */
    async hexToBech32(hex: string, bech32Hrp?: string): Promise<string> {
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
     * Transforms an alias id to a bech32 encoded address.
     * @param TODO TODO.
     * @returns TODO.
     */
    async aliasIdToBech32(
        aliasId: string,
        bech32Hrp?: string,
    ): Promise<string> {
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
     * Transforms an nft id to a bech32 encoded address.
     * @param TODO TODO.
     * @returns TODO.
     */
    async nftIdToBech32(nftId: string, bech32Hrp?: string): Promise<string> {
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
     * Transforms a hex encoded public key to a bech32 encoded address.
     * @param TODO TODO.
     * @returns TODO.
     */
    async hexPublicKeyToBech32Address(
        hex: string,
        bech32Hrp?: string,
    ): Promise<string> {
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
     * Fetch alias output IDs
     * @param TODO TODO.
     * @returns TODO.
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
     * Fetch alias output ID
     * @param TODO TODO.
     * @returns TODO.
     */
    async aliasOutputId(aliasId: string): Promise<string> {
        const response = await this.methodHandler.callMethod({
            name: 'aliasOutputId',
            data: {
                aliasId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Fetch NFT output IDs
     * @param TODO TODO.
     * @returns TODO.
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
     * Fetch NFT output ID
     * @param TODO TODO.
     * @returns TODO.
     */
    async nftOutputId(nftId: string): Promise<string> {
        const response = await this.methodHandler.callMethod({
            name: 'nftOutputId',
            data: {
                nftId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Fetch Foundry Output IDs
     * @param TODO TODO.
     * @returns TODO.
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
     * Fetch Foundry Output ID
     * @param TODO TODO.
     * @returns TODO.
     */
    async foundryOutputId(foundryId: string): Promise<string> {
        const response = await this.methodHandler.callMethod({
            name: 'foundryOutputId',
            data: {
                foundryId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Try to get OutputResponse from provided OutputIds (requests are sent
     * in parallel and errors are ignored, can be useful for spent outputs)
     * @param TODO TODO.
     * @returns TODO.
     */
    async getOutputsIgnoreErrors(
        outputIds: string[],
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
     * Find all blocks by provided block IDs.
     * @param TODO TODO.
     * @returns TODO.
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
     * Retries (promotes or reattaches) a block for provided block id. Block should be
     * retried only if they are valid and haven't been confirmed for a while.
     * @param TODO TODO.
     * @returns TODO.
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
     * Retries (promotes or reattaches) a block for provided block id until it's included (referenced by a
     * milestone). Default interval is 5 seconds and max attempts is 40. Returns the included block at first
     * position and additional reattached blocks
     * @param TODO TODO.
     * @returns TODO.
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
     * Function to consolidate all funds from a range of addresses to the address with the lowest index in that range
     * Returns the address to which the funds got consolidated, if any were available
     * @param TODO TODO.
     * @returns TODO.
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
     * Reattaches blocks for provided block id. Blocks can be reattached only if they are valid and haven't been
     * confirmed for a while.
     * @param TODO TODO.
     * @returns TODO.
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
     * Reattach a block without checking if it should be reattached
     * @param TODO TODO.
     * @returns TODO.
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
     * Promotes a block. The method should validate if a promotion is necessary through get_block. If not, the
     * method should error out and should not allow unnecessary promotions.
     * @param TODO TODO.
     * @returns TODO.
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
     * Promote a block without checking if it should be promoted
     * @param TODO TODO.
     * @returns TODO.
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
     * Returns the unhealthy nodes.
     * @param TODO TODO.
     * @returns TODO.
     */
    async unhealthyNodes(): Promise<Set<INode>> {
        const response = await this.methodHandler.callMethod({
            name: 'unhealthyNodes',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Build a Basic Output.
     * @param TODO TODO.
     * @returns TODO.
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
     * Build an Alias Output.
     * @param TODO TODO.
     * @returns TODO.
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
     * Build a Foundry Output.
     * @param TODO TODO.
     * @returns TODO.
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
     * Build an Nft Output.
     * @param TODO TODO.
     * @returns TODO.
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
     * Listen to MQTT topics.
     * @param TODO TODO.
     * @returns TODO.
     */
    async listenMqtt(
        topics: string[],
        callback: (error: Error, result: string) => void,
    ): Promise<void> {
        return this.methodHandler.listen(topics, callback);
    }

    /**
     * Stop listening for provided MQTT topics.
     * @param TODO TODO.
     * @returns TODO.
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
     * @param output output to calculate the deposit amount for.
     * @returns The amount.
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
     * Request funds from a faucet, for example `https://faucet.testnet.shimmer.network/api/enqueue` or `http://localhost:8091/api/enqueue`.
     * @param TODO TODO.
     * @returns TODO.
     */
    async requestFundsFromFaucet(
        url: string,
        address: string,
    ): Promise<string> {
        const response = await this.methodHandler.callMethod({
            name: 'requestFundsFromFaucet',
            data: { url, address },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Extension method which provides request methods for plugins.
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
