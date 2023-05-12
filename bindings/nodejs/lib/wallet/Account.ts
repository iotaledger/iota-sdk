// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { WalletMethodHandler } from './WalletMethodHandler';
import type {
    AccountBalance,
    AccountMetadata,
    SyncOptions,
    AccountMeta,
    AccountAddress,
    SendAmountParams,
    SendNativeTokensParams,
    SendNftParams,
    AddressWithUnspentOutputs,
    AliasOutputParams,
    FilterOptions,
    GenerateAddressOptions,
    MintTokenTransaction,
    MintNativeTokenParams,
    MintNftParams,
    OutputData,
    OutputParams,
    OutputsToClaim,
    PreparedTransactionData,
    Transaction,
    TransactionOptions,
    ParticipationOverview,
    ParticipationEventId,
    ParticipationEventStatus,
    ParticipationEventType,
    ParticipationEventWithNodes,
    ParticipationEventRegistrationOptions,
    ParticipationEventMap,
} from '../../types/wallet';
import type { SignedTransactionEssence } from '../../types/wallet/signedTransactionEssence';
import type {
    BuildAliasOutputData,
    BuildBasicOutputData,
    BuildFoundryOutputData,
    BuildNftOutputData,
} from '../../types/wallet/buildOutputData';
import type {
    HexEncodedAmount,
    IAliasOutput,
    IBasicOutput,
    IFoundryOutput,
    INftOutput,
    OutputTypes,
} from '@iota/types';
import { INode } from '../client';

/** The Account class. */
export class Account {
    // private because the data isn't updated
    private meta: AccountMeta;
    private methodHandler: WalletMethodHandler;

    constructor(accountMeta: AccountMeta, methodHandler: WalletMethodHandler) {
        this.meta = accountMeta;
        this.methodHandler = methodHandler;
    }

    /**
     * Build an `AliasOutput`.
     * @param data Options for building an `AliasOutput`.
     * @returns The built `AliasOutput`.
     */
    async buildAliasOutput(data: BuildAliasOutputData): Promise<IAliasOutput> {
        const resp = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'buildAliasOutput',
                data,
            },
        );
        return JSON.parse(resp).payload;
    }

    /**
     * Build a `BasicOutput`.
     * @param data Options for building a `BasicOutput`.
     * @returns The built `BasicOutput`.
     */
    async buildBasicOutput(data: BuildBasicOutputData): Promise<IBasicOutput> {
        const resp = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'buildBasicOutput',
                data,
            },
        );
        return JSON.parse(resp).payload;
    }

    /**
     * Build a `FoundryOutput`.
     * @param data Options for building a `FoundryOutput`.
     * @returns The built `FoundryOutput`.
     */
    async buildFoundryOutput(
        data: BuildFoundryOutputData,
    ): Promise<IFoundryOutput> {
        const resp = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'buildFoundryOutput',
                data,
            },
        );
        return JSON.parse(resp).payload;
    }

    /**
     * Build an `NftOutput`.
     * @param data Options for building an `NftOutput`.
     * @returns The built `NftOutput`.
     */
    async buildNftOutput(data: BuildNftOutputData): Promise<INftOutput> {
        const resp = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'buildNftOutput',
                data,
            },
        );
        return JSON.parse(resp).payload;
    }

    /**
     * Burn native tokens. This doesn't require the foundry output which minted them, but will not increase
     * the foundries `melted_tokens` field, which makes it impossible to destroy the foundry output. Therefore it's
     * recommended to use melting, if the foundry output is available.
     * @param tokenId The native token id.
     * @param burnAmount The to be burned amount.
     * @param transactionOptions The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns The transaction.
     */
    async prepareBurnNativeToken(
        tokenId: string,
        burnAmount: HexEncodedAmount,
        transactionOptions?: TransactionOptions,
    ): Promise<PreparedTransactionData> {
        const resp = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'prepareBurnNativeToken',
                data: {
                    tokenId,
                    burnAmount,
                    options: transactionOptions,
                },
            },
        );
        return JSON.parse(resp).payload;
    }

    /**
     * Burn an nft output. Outputs controlled by it will be sweeped before if they don't have a storage
     * deposit return, timelock or expiration unlock condition. This should be preferred over burning, because after
     * burning, the foundry can never be destroyed anymore.
     * @param nftId The NftId.
     * @param transactionOptions The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns The transaction.
     */
    async prepareBurnNft(
        nftId: string,
        transactionOptions?: TransactionOptions,
    ): Promise<PreparedTransactionData> {
        const resp = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'prepareBurnNft',
                data: {
                    nftId,
                    options: transactionOptions,
                },
            },
        );
        return JSON.parse(resp).payload;
    }

    /**
     * Claim basic or nft outputs that have additional unlock conditions
     * to their `AddressUnlockCondition` from the account.
     * @param outputIds The outputs to claim.
     * @returns The resulting transaction.
     */
    async claimOutputs(outputIds: string[]): Promise<Transaction> {
        const resp = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'claimOutputs',
                data: {
                    outputIdsToClaim: outputIds,
                },
            },
        );
        return JSON.parse(resp).payload;
    }

    /**
     * Consolidate basic outputs with only an `AddressUnlockCondition` from an account
     * by sending them to an own address again if the output amount is greater or
     * equal to the output consolidation threshold.
     * @param force Force consolidation on addresses where the threshold isn't met.
     * @param outputConsolidationThreshold A default threshold is used if this is omitted.
     * @returns The consolidation transaction.
     */
    async prepareConsolidateOutputs(
        force: boolean,
        outputConsolidationThreshold?: number,
    ): Promise<PreparedTransactionData> {
        const resp = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'prepareConsolidateOutputs',
                data: {
                    force,
                    outputConsolidationThreshold,
                },
            },
        );
        return JSON.parse(resp).payload;
    }

    /**
     * `createAliasOutput` creates an alias output
     * @param params The alias output options.
     * @param transactionOptions The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns A transaction object.
     */
    async prepareCreateAliasOutput(
        params?: AliasOutputParams,
        transactionOptions?: TransactionOptions,
    ): Promise<PreparedTransactionData> {
        const resp = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'prepareCreateAliasOutput',
                data: {
                    params,
                    options: transactionOptions,
                },
            },
        );
        return JSON.parse(resp).payload;
    }

    /**
     * Melt native tokens. This happens with the foundry output which minted them, by increasing its
     * `melted_tokens` field.
     * @param tokenId The native token id.
     * @param meltAmount To be melted amount.
     * @param transactionOptions The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns The transaction.
     */
    async prepareDecreaseNativeTokenSupply(
        tokenId: string,
        meltAmount: HexEncodedAmount,
        transactionOptions?: TransactionOptions,
    ): Promise<PreparedTransactionData> {
        const resp = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'prepareDecreaseNativeTokenSupply',
                data: {
                    tokenId,
                    meltAmount,
                    options: transactionOptions,
                },
            },
        );
        return JSON.parse(resp).payload;
    }

    async deregisterParticipationEvent(
        eventId: ParticipationEventId,
    ): Promise<void> {
        const resp = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'deregisterParticipationEvent',
                data: {
                    eventId,
                },
            },
        );
        return JSON.parse(resp).payload;
    }

    /**
     * Destroy an alias output. Outputs controlled by it will be sweeped before if they don't have a
     * storage deposit return, timelock or expiration unlock condition. The amount and possible native tokens will be
     * sent to the governor address.
     * @param aliasId The AliasId.
     * @param transactionOptions The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns The transaction.
     */
    async prepareDestroyAlias(
        aliasId: string,
        transactionOptions?: TransactionOptions,
    ): Promise<PreparedTransactionData> {
        const resp = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'prepareDestroyAlias',
                data: {
                    aliasId,
                    options: transactionOptions,
                },
            },
        );
        return JSON.parse(resp).payload;
    }

    /**
     * Function to destroy a foundry output with a circulating supply of 0.
     * Native tokens in the foundry (minted by other foundries) will be transactioned to the controlling alias.
     * @param foundryId The FoundryId.
     * @param transactionOptions The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns The transaction.
     */
    async prepareDestroyFoundry(
        foundryId: string,
        transactionOptions?: TransactionOptions,
    ): Promise<PreparedTransactionData> {
        const resp = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'prepareDestroyFoundry',
                data: {
                    foundryId,
                    options: transactionOptions,
                },
            },
        );
        return JSON.parse(resp).payload;
    }

    /**
     * Generate a new unused address.
     * @param options Options for address generation.
     * @returns The address.
     */
    async generateAddress(
        options?: GenerateAddressOptions,
    ): Promise<AccountAddress> {
        const addresses = await this.generateAddresses(1, options);
        return addresses[0];
    }

    /**
     * Generate new unused addresses.
     * @param amount The amount of addresses to generate.
     * @param options Options for address generation.
     * @returns The addresses.
     */
    async generateAddresses(
        amount: number,
        options?: GenerateAddressOptions,
    ): Promise<AccountAddress[]> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'generateAddresses',
                data: {
                    amount,
                    options,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Get the account balance.
     * @returns The account balance.
     */
    async getBalance(): Promise<AccountBalance> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'getBalance',
            },
        );

        return JSON.parse(response).payload;
    }

    /**
     * Get the data for an output.
     * @param outputId The output to get.
     * @returns The `OutputData`.
     */
    async getOutput(outputId: string): Promise<OutputData> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'getOutput',
                data: {
                    outputId,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    async getParticipationEvent(
        eventId: ParticipationEventId,
    ): Promise<ParticipationEventWithNodes> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'getParticipationEvent',
                data: {
                    eventId,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    async getParticipationEventIds(
        node: INode,
        eventType?: ParticipationEventType,
    ): Promise<ParticipationEventId[]> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'getParticipationEventIds',
                data: {
                    node,
                    eventType,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    async getParticipationEvents(): Promise<ParticipationEventMap> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'getParticipationEvents',
            },
        );
        return JSON.parse(response).payload;
    }

    async getParticipationEventStatus(
        eventId: ParticipationEventId,
    ): Promise<ParticipationEventStatus> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'getParticipationEventStatus',
                data: {
                    eventId,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Get a `FoundryOutput` by native token ID. It will try to get the foundry from
     * the account, if it isn't in the account it will try to get it from the node.
     * @param tokenId The native token ID to get the foundry for.
     * @returns The `FoundryOutput` that minted the token.
     */
    async getFoundryOutput(tokenId: string): Promise<IFoundryOutput> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'getFoundryOutput',
                data: {
                    tokenId,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Get outputs with additional unlock conditions.
     * @param outputs The type of outputs to claim.
     * @returns The output IDs of the unlockable outputs.
     */
    async getOutputsWithAdditionalUnlockConditions(
        outputs: OutputsToClaim,
    ): Promise<string[]> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'getOutputsWithAdditionalUnlockConditions',
                data: {
                    outputsToClaim: outputs,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Get a transaction stored in the account.
     * @param transactionId The ID of the transaction to get.
     * @returns The transaction.
     */
    async getTransaction(transactionId: string): Promise<Transaction> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'getTransaction',
                data: {
                    transactionId,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Get the transaction with inputs of an incoming transaction stored in the account
     * List might not be complete, if the node pruned the data already
     * @param transactionId The ID of the transaction to get.
     * @returns The transaction.
     */
    async getIncomingTransactionData(
        transactionId: string,
    ): Promise<Transaction> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'getIncomingTransactionData',
                data: {
                    transactionId,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * List all the addresses of the account.
     * @returns The addresses.
     */
    async addresses(): Promise<AccountAddress[]> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'addresses',
            },
        );

        return JSON.parse(response).payload;
    }

    /**
     * List the addresses of the account with unspent outputs.
     * @returns The addresses.
     */
    async addressesWithUnspentOutputs(): Promise<AddressWithUnspentOutputs[]> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'addressesWithUnspentOutputs',
            },
        );

        return JSON.parse(response).payload;
    }

    /**
     * List all outputs of the account.
     * @param filterOptions Options to filter the to be returned outputs.
     * @returns The outputs with metadata.
     */
    async outputs(filterOptions?: FilterOptions): Promise<OutputData[]> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'outputs',
                data: { filterOptions },
            },
        );

        return JSON.parse(response).payload;
    }

    /**
     * List all the pending transactions of the account.
     * @returns The transactions.
     */
    async pendingTransactions(): Promise<Transaction[]> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'pendingTransactions',
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * List all incoming transactions of the account.
     * @returns The incoming transactions with their inputs.
     */
    async incomingTransactions(): Promise<[string, Transaction][]> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'incomingTransactions',
            },
        );

        return JSON.parse(response).payload;
    }

    /**
     * List all the transactions of the account.
     * @returns The transactions.
     */
    async transactions(): Promise<Transaction[]> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'transactions',
            },
        );

        return JSON.parse(response).payload;
    }

    /**
     * List all the unspent outputs of the account.
     * @param filterOptions Options to filter the to be returned outputs.
     * @returns The outputs with metadata.
     */
    async unspentOutputs(filterOptions?: FilterOptions): Promise<OutputData[]> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'unspentOutputs',
                data: { filterOptions },
            },
        );

        return JSON.parse(response).payload;
    }

    /**
     * Get the accounts metadata.
     * @returns The accounts metadata.
     */
    getMetadata(): AccountMetadata {
        return {
            alias: this.meta.alias,
            coinType: this.meta.coinType,
            index: this.meta.index,
        };
    }

    /**
     * Calculate the minimum required storage deposit for an output.
     * @param output output to calculate the deposit amount for.
     * @returns The amount.
     */
    async minimumRequiredStorageDeposit(output: OutputTypes): Promise<string> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'minimumRequiredStorageDeposit',
                data: {
                    output,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Mint more native tokens.
     * @param tokenId The native token id.
     * @param mintAmount To be minted amount.
     * @param transactionOptions The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns The minting transaction and the token ID.
     */
    async prepareIncreaseNativeTokenSupply(
        tokenId: string,
        mintAmount: HexEncodedAmount,
        transactionOptions?: TransactionOptions,
    ): Promise<MintTokenTransaction> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'prepareIncreaseNativeTokenSupply',
                data: {
                    tokenId,
                    mintAmount,
                    options: transactionOptions,
                },
            },
        );

        return JSON.parse(response).payload;
    }

    /**
     * Mint native tokens.
     * @param params The options for minting tokens.
     * @param transactionOptions The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns The minting transaction and the token ID.
     */
    async prepareMintNativeToken(
        params: MintNativeTokenParams,
        transactionOptions?: TransactionOptions,
    ): Promise<MintTokenTransaction> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'prepareMintNativeToken',
                data: {
                    params,
                    options: transactionOptions,
                },
            },
        );

        return JSON.parse(response).payload;
    }

    /**
     * Mint nfts.
     * @param params The options for minting nfts.
     * @param transactionOptions The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns The minting transaction.
     */
    async prepareMintNfts(
        params: MintNftParams[],
        transactionOptions?: TransactionOptions,
    ): Promise<PreparedTransactionData> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'prepareMintNfts',
                data: {
                    params,
                    options: transactionOptions,
                },
            },
        );

        return JSON.parse(response).payload;
    }

    /**
     * Prepare an output for sending, useful for offline signing.
     * @param options The options for preparing an output. If the amount is
     * below the minimum required storage deposit, by default the remaining
     * amount will automatically be added with a `StorageDepositReturn` `UnlockCondition`,
     * when setting the `ReturnStrategy` to `gift`, the full minimum required
     * storage deposit will be sent to the recipient. When the assets contain
     * an nft id, the data from the existing `NftOutput` will be used, just with
     * the address unlock conditions replaced.
     * @param transactionOptions The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns The prepared output.
     */
    async prepareOutput(
        params: OutputParams,
        transactionOptions?: TransactionOptions,
    ): Promise<OutputTypes> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'prepareOutput',
                data: {
                    params,
                    transactionOptions,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Send an amount transaction
     * @param params Address with amounts to send.
     * @param options The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns The prepared transaction data.
     */
     async sendAmount(
        params: SendAmountParams[],
        options?: TransactionOptions,
    ): Promise<Transaction> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'sendAmount',
                data: {
                    params,
                    options,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Prepare a send amount transaction, useful for offline signing.
     * @param params Address with amounts to send.
     * @param options The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns The prepared transaction data.
     */
    async prepareSendAmount(
        params: SendAmountParams[],
        options?: TransactionOptions,
    ): Promise<PreparedTransactionData> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'prepareSendAmount',
                data: {
                    params,
                    options,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Prepare a transaction, useful for offline signing.
     * @param outputs Outputs to use in the transaction.
     * @param options The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns The prepared transaction data.
     */
    async prepareTransaction(
        outputs: OutputTypes[],
        options?: TransactionOptions,
    ): Promise<PreparedTransactionData> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'prepareTransaction',
                data: {
                    outputs,
                    options,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    async registerParticipationEvents(
        options: ParticipationEventRegistrationOptions,
    ): Promise<ParticipationEventMap> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'registerParticipationEvents',
                data: {
                    options,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Retries (promotes or reattaches) a transaction sent from the account for a provided transaction id until it's
     * included (referenced by a milestone). Returns the included block id.
     */
    async retryTransactionUntilIncluded(
        transactionId: string,
        interval?: number,
        maxAttempts?: number,
    ): Promise<PreparedTransactionData> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'retryTransactionUntilIncluded',
                data: {
                    transactionId,
                    interval,
                    maxAttempts,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Send native tokens.
     * @param params Addresses amounts and native tokens.
     * @param transactionOptions The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns The sent transaction.
     */
    async prepareSendNativeTokens(
        params: SendNativeTokensParams[],
        transactionOptions?: TransactionOptions,
    ): Promise<PreparedTransactionData> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'prepareSendNativeTokens',
                data: {
                    params,
                    options: transactionOptions,
                },
            },
        );

        return JSON.parse(response).payload;
    }

    /**
     * Send nft.
     * @param params Addresses and nft ids.
     * @param transactionOptions The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns The sent transaction.
     */
    async prepareSendNft(
        params: SendNftParams[],
        transactionOptions?: TransactionOptions,
    ): Promise<PreparedTransactionData> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'prepareSendNft',
                data: {
                    params,
                    options: transactionOptions,
                },
            },
        );

        return JSON.parse(response).payload;
    }

    /**
     * Send outputs in a transaction.
     * @param outputs The outputs to send.
     * @param transactionOptions The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns The sent transaction.
     */
    async sendOutputs(
        outputs: OutputTypes[],
        transactionOptions?: TransactionOptions,
    ): Promise<Transaction> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'sendOutputs',
                data: {
                    outputs,
                    options: transactionOptions,
                },
            },
        );

        return JSON.parse(response).payload;
    }

    /**
     * Set the alias for the account
     * @param alias The account alias to set.
     */
    async setAlias(alias: string): Promise<void> {
        await this.methodHandler.callAccountMethod(this.meta.index, {
            name: 'setAlias',
            data: {
                alias,
            },
        });
    }

    /**
     * Set the fallback SyncOptions for account syncing.
     * If storage is enabled, will persist during restarts.
     * @param options The sync options to set.
     */
    async setDefaultSyncOptions(options: SyncOptions): Promise<void> {
        await this.methodHandler.callAccountMethod(this.meta.index, {
            name: 'setDefaultSyncOptions',
            data: {
                options,
            },
        });
    }

    /**
     * Sign a prepared transaction, useful for offline signing.
     * @param preparedTransactionData The prepared transaction data to sign.
     * @returns The signed transaction essence.
     */
    async signTransactionEssence(
        preparedTransactionData: PreparedTransactionData,
    ): Promise<SignedTransactionEssence> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'signTransactionEssence',
                data: {
                    preparedTransactionData,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Sign a prepared transaction, and send it.
     * @param preparedTransactionData The prepared transaction data to sign and submit.
     * @returns The transaction.
     */
     async signAndSubmitTransaction(
        preparedTransactionData: PreparedTransactionData,
    ): Promise<Transaction> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'signAndSubmitTransaction',
                data: {
                    preparedTransactionData,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Validate the transaction, submit it to a node and store it in the account.
     * @param signedTransactionData A signed transaction to submit and store.
     * @returns The sent transaction.
     */
    async submitAndStoreTransaction(
        signedTransactionData: SignedTransactionEssence,
    ): Promise<Transaction> {
        const response = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'submitAndStoreTransaction',
                data: {
                    signedTransactionData,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Sync the account by fetching new information from the nodes.
     * Will also retry pending transactions if necessary.
     * A custom default can be set using setDefaultSyncOptions.
     *
     * @param options Optional synchronization options.
     * @returns The account balance.
     */
    async sync(options?: SyncOptions): Promise<AccountBalance> {
        const resp = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'sync',
                data: {
                    options,
                },
            },
        );
        return JSON.parse(resp).payload;
    }

    async prepareVote(
        eventId?: ParticipationEventId,
        answers?: number[],
    ): Promise<PreparedTransactionData> {
        const resp = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'prepareVote',
                data: {
                    eventId,
                    answers,
                },
            },
        );
        return JSON.parse(resp).payload;
    }

    async prepareStopParticipating(
        eventId: ParticipationEventId,
    ): Promise<PreparedTransactionData> {
        const resp = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'prepareStopParticipating',
                data: {
                    eventId,
                },
            },
        );
        return JSON.parse(resp).payload;
    }

    /**
     * Calculates the voting overview of an account.
     * @param eventIds Optional, filters participations only for provided events.
     * @returns ParticipationOverview
     */
    async getParticipationOverview(
        eventIds?: ParticipationEventId[],
    ): Promise<ParticipationOverview> {
        const resp = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'getParticipationOverview',
                data: {
                    eventIds,
                },
            },
        );
        return JSON.parse(resp).payload;
    }

    async prepareIncreaseVotingPower(amount: string): Promise<PreparedTransactionData> {
        const resp = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'prepareIncreaseVotingPower',
                data: {
                    amount,
                },
            },
        );
        return JSON.parse(resp).payload;
    }

    async prepareDecreaseVotingPower(amount: string): Promise<PreparedTransactionData> {
        const resp = await this.methodHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'prepareDecreaseVotingPower',
                data: {
                    amount,
                },
            },
        );
        return JSON.parse(resp).payload;
    }
}
