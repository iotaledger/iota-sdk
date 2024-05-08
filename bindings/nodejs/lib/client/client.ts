// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { plainToInstance } from 'class-transformer';
import { ClientMethodHandler } from './client-method-handler';
import { Response } from '../types';
import {
    ClientOptions,
    Node,
    Auth,
    AccountOutputBuilderParams,
    BasicOutputBuilderParams,
    FoundryOutputBuilderParams,
    NftOutputBuilderParams,
    AccountOutputQueryParameters,
    AnchorOutputQueryParameters,
    BasicOutputQueryParameters,
    DelegationOutputQueryParameters,
    FoundryOutputQueryParameters,
    NftOutputQueryParameters,
    OutputQueryParameters,
} from '../types/client';
import {
    AccountOutput,
    BasicOutput,
    FoundryOutput,
    NftOutput,
    Output,
    BlockId,
    Payload,
    parseBlock,
    Block,
    AccountId,
    AnchorId,
    NftId,
    FoundryId,
    DelegationId,
    UnsignedBlock,
    parseUnsignedBlock,
    SlotIndex,
    SlotCommitmentId,
    SlotCommitment,
    EpochIndex,
    Address,
} from '../types/block';
import type { NodeInfoResponse } from '../types/client/nodeInfo';
import {
    UTXOInput,
    OutputId,
    ProtocolParameters,
    u64,
    TransactionId,
    Bech32Address,
} from '../types';
import {
    BlockMetadataResponse,
    BlockWithMetadataResponse,
    InfoResponse,
    IssuanceBlockHeaderResponse,
    CommitteeResponse,
    CongestionResponse,
    ManaRewardsResponse,
    NetworkMetricsResponse,
    OutputMetadataResponse,
    OutputResponse,
    OutputsResponse,
    OutputWithMetadataResponse,
    RoutesResponse,
    TransactionMetadataResponse,
    UtxoChangesFullResponse,
    UtxoChangesResponse,
    ValidatorResponse,
    ValidatorsResponse,
} from '../types/models/api';

/** The Client to interact with nodes. */
export class Client {
    private methodHandler: ClientMethodHandler;

    /**
     * @param methodHandler The Rust method handler created in `ClientMethodHandler.create()`.
     */
    constructor(methodHandler: ClientMethodHandler) {
        this.methodHandler = methodHandler;
    }

    /**
     * @param options The client options.
     */
    static async create(options: ClientOptions): Promise<Client> {
        return new Client(await ClientMethodHandler.create(options));
    }
    async destroy(): Promise<void> {
        return this.methodHandler.destroy();
    }

    // Node routes.

    /**
     * Returns the health of the node.
     * GET /health
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
     * Returns the available API route groups of the node.
     * GET /api/routes
     */
    async getRoutes(): Promise<RoutesResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getRoutes',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the node information together with the url of the used node.
     */
    async getNodeInfo(): Promise<NodeInfoResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getNodeInfo',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns general information about the node.
     * GET /api/core/v3/info
     *
     * @param url The URL of the node.
     * @param auth An authentication object (e.g. JWT).
     */
    async getInfo(url: string, auth?: Auth): Promise<InfoResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getInfo',
            data: {
                url,
                auth,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the network metrics.
     */
    async getNetworkMetrics(): Promise<NetworkMetricsResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getNetworkMetrics',
        });

        return JSON.parse(response).payload;
    }

    // Accounts routes.

    /**
     * Checks if the account is ready to issue a block.
     * GET /api/core/v3/accounts/{bech32Address}/congestion
     */
    async getAccountCongestion(
        accountId: AccountId,
        workScore?: number,
    ): Promise<CongestionResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getAccountCongestion',
            data: {
                accountId,
                workScore,
            },
        });

        return JSON.parse(response).payload;
    }

    // Rewards routes.

    /**
     * Returns the total available Mana rewards of an account or delegation output decayed up to `epochEnd` index
     * provided in the response.
     * Note that rewards for an epoch only become available at the beginning of the next epoch. If the end epoch of a
     * staking feature is equal or greater than the current epoch, the rewards response will not include the potential
     * future rewards for those epochs. `epochStart` and `epochEnd` indicates the actual range for which reward value
     * is returned and decayed for.
     * GET /api/core/v3/rewards/{outputId}
     */
    async getOutputManaRewards(
        outputId: OutputId,
        slot?: SlotIndex,
    ): Promise<ManaRewardsResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getOutputManaRewards',
            data: {
                outputId,
                slot,
            },
        });

        return JSON.parse(response).payload;
    }

    // Validators routes.

    /**
     * Returns information of all registered validators and if they are active, ordered by their holding stake.
     * GET /api/core/v3/validators
     */
    async getValidators(
        pageSize?: number,
        cursor?: string,
    ): Promise<ValidatorsResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getValidators',
            data: {
                pageSize,
                cursor,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Return information about a staker (registered validator).
     * GET /api/core/v3/validators/{bech32Address}
     */
    async getValidator(accountId: AccountId): Promise<ValidatorResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getValidator',
            data: {
                accountId,
            },
        });

        return JSON.parse(response).payload;
    }

    // Committee routes.

    /**
     * Returns the information of committee members at the given epoch index. If epoch index is not provided, the
     * current committee members are returned.
     * GET /api/core/v3/committee/?epoch
     */
    async getCommittee(epoch?: EpochIndex): Promise<CommitteeResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getCommittee',
            data: {
                epoch,
            },
        });

        return JSON.parse(response).payload;
    }

    // Blocks routes.

    /**
     * Returns information that is ideal for attaching a block in the network.
     * GET /api/core/v3/blocks/issuance
     */
    async getIssuance(): Promise<IssuanceBlockHeaderResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getIssuance',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns the BlockId of the submitted block.
     * POST /api/core/v3/blocks
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
     * Returns the BlockId of the submitted block.
     * POST /api/core/v3/blocks
     *
     * @param blockBytes The block as raw bytes.
     * @returns The ID of the posted block.
     */
    async postBlockRaw(blockBytes: Uint8Array): Promise<BlockId> {
        const response = await this.methodHandler.callMethod({
            name: 'postBlockRaw',
            data: {
                blockBytes,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Finds a block by its ID and returns it as object.
     * GET /api/core/v3/blocks/{blockId}
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
        return parseBlock(parsed.payload);
    }

    /**
     * Finds a block by its ID and returns it as raw bytes.
     * GET /api/core/v3/blocks/{blockId}
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
     * Returns the metadata of a block.
     * GET /api/core/v3/blocks/{blockId}/metadata
     *
     * @param blockId The corresponding block ID of the requested block metadata.
     * @returns The requested block metadata.
     */
    async getBlockMetadata(blockId: BlockId): Promise<BlockMetadataResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getBlockMetadata',
            data: {
                blockId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns a block with its metadata.
     * GET /api/core/v2/blocks/{blockId}/full
     *
     * @param blockId The corresponding block ID of the requested block.
     * @returns The requested block with its metadata.
     */
    async getBlockWithMetadata(
        blockId: BlockId,
    ): Promise<BlockWithMetadataResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getBlockWithMetadata',
            data: {
                blockId,
            },
        });

        return JSON.parse(response).payload;
    }

    // UTXO routes.

    /**
     * Finds an output by its ID and returns it as object.
     * GET /api/core/v3/outputs/{outputId}
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
     * Finds an output by its ID and returns it as raw bytes.
     * GET /api/core/v3/outputs/{outputId}
     */
    async getOutputRaw(outputId: OutputId): Promise<Uint8Array> {
        const response = await this.methodHandler.callMethod({
            name: 'getOutputRaw',
            data: {
                outputId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Finds output metadata by output ID.
     * GET /api/core/v3/outputs/{outputId}/metadata
     */
    async getOutputMetadata(
        outputId: OutputId,
    ): Promise<OutputMetadataResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getOutputMetadata',
            data: {
                outputId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Finds an output with its metadata by output ID.
     * GET /api/core/v3/outputs/{outputId}/full
     */
    async getOutputWithMetadata(
        outputId: OutputId,
    ): Promise<OutputWithMetadataResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getOutputWithMetadata',
            data: {
                outputId,
            },
        });

        const parsed = JSON.parse(
            response,
        ) as Response<OutputWithMetadataResponse>;
        return plainToInstance(OutputWithMetadataResponse, parsed.payload);
    }

    /**
     * Returns the earliest confirmed block containing the transaction with the given ID.
     * GET /api/core/v3/transactions/{transactionId}/included-block
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
        return parseBlock(parsed.payload);
    }

    /**
     * Returns the earliest confirmed block containing the transaction with the given ID, as raw bytes.
     * GET /api/core/v3/transactions/{transactionId}/included-block
     *
     * @param transactionId The ID of the transaction.
     * @returns The included block that contained the transaction.
     */
    async getIncludedBlockRaw(
        transactionId: TransactionId,
    ): Promise<Uint8Array> {
        const response = await this.methodHandler.callMethod({
            name: 'getIncludedBlockRaw',
            data: {
                transactionId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Returns the metadata of the earliest block containing the tx that was confirmed.
     * GET /api/core/v3/transactions/{transactionId}/included-block/metadata
     *
     * @param transactionId The ID of the transaction.
     * @returns The included block that contained the transaction.
     */
    async getIncludedBlockMetadata(
        transactionId: TransactionId,
    ): Promise<BlockMetadataResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getIncludedBlockMetadata',
            data: {
                transactionId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Finds the metadata of a transaction.
     * GET /api/core/v3/transactions/{transactionId}/metadata
     *
     * @param transactionId The ID of the transaction.
     * @returns The transaction metadata.
     */
    async getTransactionMetadata(
        transactionId: TransactionId,
    ): Promise<TransactionMetadataResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getTransactionMetadata',
            data: {
                transactionId,
            },
        });

        return JSON.parse(response).payload;
    }

    // Commitments routes.

    /**
     * Finds a slot commitment by its ID and returns it as object.
     * GET /api/core/v3/commitments/{commitmentId}
     *
     * @param commitmentId Commitment ID of the commitment to look up.
     * @returns The commitment.
     */
    async getCommitment(
        commitmentId: SlotCommitmentId,
    ): Promise<SlotCommitment> {
        const response = await this.methodHandler.callMethod({
            name: 'getCommitment',
            data: {
                commitmentId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Finds a slot commitment by its ID and returns it as raw bytes.
     * GET /api/core/v3/commitments/{commitmentId}
     *
     * @param commitmentId Commitment ID of the commitment to look up.
     * @returns The commitment as raw bytes.
     */
    async getCommitmentRaw(
        commitmentId: SlotCommitmentId,
    ): Promise<Uint8Array> {
        const response = await this.methodHandler.callMethod({
            name: 'getCommitmentRaw',
            data: {
                commitmentId,
            },
        });

        return JSON.parse(response).payload;
    }
    /**
     * Get all UTXO changes of a given slot by slot commitment ID.
     * GET /api/core/v3/commitments/{commitmentId}/utxo-changes
     *
     * @param commitmentId Commitment ID of the commitment to look up.
     * @returns The UTXO changes.
     */
    async getUtxoChanges(
        commitmentId: SlotCommitmentId,
    ): Promise<UtxoChangesResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getUtxoChanges',
            data: {
                commitmentId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get all full UTXO changes of a given slot by slot commitment ID.
     * GET /api/core/v3/commitments/{commitmentId}/utxo-changes/full
     *
     * @param commitmentId Commitment ID of the commitment to look up.
     * @returns The UTXO changes.
     */
    async getUtxoChangesFull(
        commitmentId: SlotCommitmentId,
    ): Promise<UtxoChangesFullResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getUtxoChangesFull',
            data: {
                commitmentId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Finds a slot commitment by slot index and returns it as object.
     * GET /api/core/v3/commitments/by-slot/{slot}
     *
     * @param slot Index of the commitment to look up.
     * @returns The commitment.
     */
    async getCommitmentBySlot(slot: SlotIndex): Promise<SlotCommitment> {
        const response = await this.methodHandler.callMethod({
            name: 'getCommitmentBySlot',
            data: {
                slot,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Finds a slot commitment by slot index and returns it as raw bytes.
     * GET /api/core/v3/commitments/by-slot/{slot}
     *
     * @param slot Index of the commitment to look up.
     * @returns The commitment as raw bytes.
     */
    async getCommitmentBySlotRaw(slot: SlotIndex): Promise<Uint8Array> {
        const response = await this.methodHandler.callMethod({
            name: 'getCommitmentBySlotRaw',
            data: {
                slot,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get all UTXO changes of a given slot by its index.
     * GET /api/core/v3/commitments/by-slot/{slot}/utxo-changes
     *
     * @param slot Index of the commitment to look up.
     * @returns The UTXO changes.
     */
    async getUtxoChangesBySlot(slot: SlotIndex): Promise<UtxoChangesResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getUtxoChangesBySlot',
            data: {
                slot,
            },
        });
        return JSON.parse(response).payload;
    }

    /**
     * Get all full UTXO changes of a given slot by its index.
     * GET /api/core/v3/commitments/by-slot/{slot}/utxo-changes/full
     *
     * @param slot Index of the commitment to look up.
     * @returns The UTXO changes.
     */
    async getUtxoChangesFullBySlot(
        slot: SlotIndex,
    ): Promise<UtxoChangesFullResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'getUtxoChangesFullBySlot',
            data: {
                slot,
            },
        });
        return JSON.parse(response).payload;
    }

    // High level API routes.

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
     * Get outputs from provided output IDs (requests are sent
     * in parallel and errors are ignored, can be useful for spent outputs)
     *
     * @param outputIds An array of output IDs.
     * @returns An array of corresponding output responses.
     */
    async getOutputsIgnoreNotFound(
        outputIds: OutputId[],
    ): Promise<OutputResponse[]> {
        const response = await this.methodHandler.callMethod({
            name: 'getOutputsIgnoreNotFound',
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
        return parsed.payload.map((p) => parseBlock(p));
    }

    /**
     * Find inputs from addresses for a given amount (useful for offline signing).
     *
     * @param addresses A list of included addresses.
     * @param amount The amount to find inputs for.
     * @returns An array of UTXO inputs.
     */
    async findInputs(addresses: string[], amount: u64): Promise<UTXOInput[]> {
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

    // Other routes.

    /**
     * Build an unsigned block.
     *
     * @param issuerId The identifier of the block issuer account.
     * @param payload The payload to post.
     * @returns The block ID followed by the block containing the payload.
     */
    async buildBasicBlock(
        issuerId: AccountId,
        payload?: Payload,
    ): Promise<UnsignedBlock> {
        const response = await this.methodHandler.callMethod({
            name: 'buildBasicBlock',
            data: {
                issuerId,
                payload,
            },
        });

        const parsed = JSON.parse(response) as Response<UnsignedBlock>;
        return parseUnsignedBlock(parsed.payload);
    }

    /**
     * Get a node candidate from the healthy node pool.
     */
    async getNode(): Promise<Node> {
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
     * Get the token supply.
     */
    async getTokenSupply(): Promise<u64> {
        return BigInt((await this.getProtocolParameters()).tokenSupply);
    }

    /**
     * Get the protocol parameters.
     */
    async getProtocolParameters(): Promise<ProtocolParameters> {
        const response = await this.methodHandler.callMethod({
            name: 'getProtocolParameters',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Converts an address to its bech32 representation
     *
     * @param address An address.
     * @param bech32Hrp The Bech32 HRP (human readable part) to be used.
     * @returns The corresponding Bech32 address.
     */
    async addressToBech32(
        address: Address,
        bech32Hrp?: string,
    ): Promise<Bech32Address> {
        const response = await this.methodHandler.callMethod({
            name: 'addressToBech32',
            data: {
                address,
                bech32Hrp,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Return the unhealthy nodes.
     */
    async unhealthyNodes(): Promise<Set<Node>> {
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
     * Build an account output.
     *
     * @param params An instance of `AccountOutputBuilderParams`.
     */
    async buildAccountOutput(
        params: AccountOutputBuilderParams,
    ): Promise<AccountOutput> {
        if (params.amount && typeof params.amount === 'bigint') {
            params.amount = params.amount.toString(10);
        }
        const response = await this.methodHandler.callMethod({
            name: 'buildAccountOutput',
            data: params,
        });

        const parsed = JSON.parse(response) as Response<AccountOutput>;
        return plainToInstance(AccountOutput, parsed.payload);
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
        return this.methodHandler.listenMqtt(topics, callback);
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
     * Calculate the minimum required amount for an output.
     *
     * @param output The output to calculate the minimum amount for.
     * @returns The minimum required amount.
     */
    async computeMinimumOutputAmount(output: Output): Promise<number> {
        const response = await this.methodHandler.callMethod({
            name: 'computeMinimumOutputAmount',
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
     * @param basePluginPath The base path for the plugin eg indexer/v2/ .
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

    // inx-indexer routes

    /**
     * Fetch account/anchor/basic/delegation/NFT/foundry output IDs based on the given query parameters.
     */
    async outputIds(
        queryParameters: OutputQueryParameters,
    ): Promise<OutputsResponse> {
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
        queryParameters: BasicOutputQueryParameters,
    ): Promise<OutputsResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'basicOutputIds',
            data: {
                queryParameters,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the corresponding output IDs given a list of account query parameters.
     *
     * @param outputQueryParameters `AccountOutputQueryParameters`.
     * @returns A paginated query response of corresponding output IDs.
     */
    async accountOutputIds(
        queryParameters: AccountOutputQueryParameters,
    ): Promise<OutputsResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'accountOutputIds',
            data: {
                queryParameters,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the corresponding output ID from an account ID.
     *
     * @param accountId An account ID.
     * @returns The corresponding output ID.
     */
    async accountOutputId(accountId: AccountId): Promise<OutputId> {
        const response = await this.methodHandler.callMethod({
            name: 'accountOutputId',
            data: {
                accountId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the corresponding output IDs given a list of anchor query parameters.
     *
     * @param outputQueryParameters `AnchorOutputQueryParameters`.
     * @returns A paginated query response of corresponding output IDs.
     */
    async anchorOutputIds(
        queryParameters: AnchorOutputQueryParameters,
    ): Promise<OutputsResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'anchorOutputIds',
            data: {
                queryParameters,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the corresponding output ID from an anchor ID.
     *
     * @param anchorId An anchor ID.
     * @returns The corresponding output ID.
     */
    async anchorOutputId(anchorId: AnchorId): Promise<OutputId> {
        const response = await this.methodHandler.callMethod({
            name: 'anchorOutputId',
            data: {
                anchorId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the corresponding output IDs given a list of delegation query parameters.
     *
     * @param outputQueryParameters `DelegationOutputQueryParameters`.
     * @returns A paginated query response of corresponding output IDs.
     */
    async delegationOutputIds(
        queryParameters: DelegationOutputQueryParameters,
    ): Promise<OutputsResponse> {
        const response = await this.methodHandler.callMethod({
            name: 'delegationOutputIds',
            data: {
                queryParameters,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the corresponding output ID from an delegation ID.
     *
     * @param delegationId A delegation ID.
     * @returns The corresponding output ID.
     */
    async delegationOutputId(delegationId: DelegationId): Promise<OutputId> {
        const response = await this.methodHandler.callMethod({
            name: 'delegationOutputId',
            data: {
                delegationId,
            },
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get the corresponding output IDs given a list of Foundry query parameters.
     *
     * @param outputQueryParameters `FoundryOutputQueryParameters`.
     * @returns A paginated query response of corresponding output IDs.
     */
    async foundryOutputIds(
        queryParameters: FoundryOutputQueryParameters,
    ): Promise<OutputsResponse> {
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
     * Get the corresponding output IDs given a list of NFT query parameters.
     *
     * @param outputQueryParameters `NftOutputQueryParameters`.
     * @returns A paginated query response of corresponding output IDs.
     */
    async nftOutputIds(
        queryParameters: NftOutputQueryParameters,
    ): Promise<OutputsResponse> {
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
     * Compute the block ID (Blake2b256 hash of the block bytes) of a block.
     *
     * @param block A block.
     * @returns The corresponding block ID.
     */
    async blockId(block: Block): Promise<BlockId> {
        const response = await this.methodHandler.callMethod({
            name: 'blockId',
            data: {
                block,
            },
        });

        return JSON.parse(response).payload;
    }
}
