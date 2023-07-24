// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { MessageHandler } from './MessageHandler';
import type {
    Balance,
    AccountMetadata,
    SyncOptions,
    AccountMeta,
    Address,
    SendParams,
    SendNativeTokensParams,
    SendNftParams,
    AddressWithUnspentOutputs,
    AliasOutputParams,
    FilterOptions,
    GenerateAddressOptions,
    CreateNativeTokenTransaction,
    CreateNativeTokenParams,
    MintNftParams,
    Node,
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
    GenerateAddressesOptions,
    Secp256k1EcdsaSignature,
    Ed25519Signature,
    ConsolidationParams,
    Bip44,
} from '../types';
import type { SignedTransactionEssence } from '../types/signedTransactionEssence';
import type {
    BuildAliasOutputData,
    BuildBasicOutputData,
    BuildFoundryOutputData,
    BuildNftOutputData,
} from '../types/buildOutputData';
import type {
    HexEncodedAmount,
    HexEncodedString,
    IAliasOutput,
    IBasicOutput,
    IFoundryOutput,
    INftOutput,
    OutputTypes,
} from '@iota/types';

/** The Account class. */
export class Account {
    // private because the data isn't updated
    private meta: AccountMeta;
    private messageHandler: MessageHandler;

    constructor(accountMeta: AccountMeta, messageHandler: MessageHandler) {
        this.meta = accountMeta;
        this.messageHandler = messageHandler;
    }

    /**
     * Build an `AliasOutput`.
     * @param data Options for building an `AliasOutput`.
     * @returns The built `AliasOutput`.
     */
    async buildAliasOutput(data: BuildAliasOutputData): Promise<IAliasOutput> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'buildAliasOutput',
                data,
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Build a `BasicOutput`.
     * @param data Options for building a `BasicOutput`.
     * @returns The built `BasicOutput`.
     */
    async buildBasicOutput(data: BuildBasicOutputData): Promise<IBasicOutput> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'buildBasicOutput',
                data,
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Build a `FoundryOutput`.
     * @param data Options for building a `FoundryOutput`.
     * @returns The built `FoundryOutput`.
     */
    async buildFoundryOutput(
        data: BuildFoundryOutputData,
    ): Promise<IFoundryOutput> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'buildFoundryOutput',
                data,
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Build an `NftOutput`.
     * @param data Options for building an `NftOutput`.
     * @returns The built `NftOutput`.
     */
    async buildNftOutput(data: BuildNftOutputData): Promise<INftOutput> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'buildNftOutput',
                data,
            },
        );
        return JSON.parse(response).payload;
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
    async burnNativeToken(
        tokenId: string,
        burnAmount: HexEncodedAmount,
        transactionOptions?: TransactionOptions,
    ): Promise<Transaction> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'burnNativeToken',
                data: {
                    tokenId,
                    burnAmount,
                    options: transactionOptions,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Burn an nft output
     * @param nftId The NftId.
     * @param transactionOptions The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns The transaction.
     */
    async burnNft(
        nftId: string,
        transactionOptions?: TransactionOptions,
    ): Promise<Transaction> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'burnNft',
                data: {
                    nftId,
                    options: transactionOptions,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Claim basic or nft outputs that have additional unlock conditions
     * to their `AddressUnlockCondition` from the account.
     * @param outputIds The outputs to claim.
     * @returns The resulting transaction.
     */
    async claimOutputs(outputIds: string[]): Promise<Transaction> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'claimOutputs',
                data: {
                    outputIdsToClaim: outputIds,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Consolidate basic outputs with only an `AddressUnlockCondition` from an account
     * by sending them to an own address again if the output amount is greater or
     * equal to the output consolidation threshold.
     * @param params The consolidation parameters.
     * @returns The consolidation transaction.
     */
    async consolidateOutputs(
        params: ConsolidationParams,
    ): Promise<Transaction> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'consolidateOutputs',
                data: {
                    params,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * `createAliasOutput` creates an alias output
     * @param params The alias output options.
     * @param transactionOptions The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns A transaction object.
     */
    async createAliasOutput(
        params?: AliasOutputParams,
        transactionOptions?: TransactionOptions,
    ): Promise<Transaction> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'createAliasOutput',
                data: {
                    params,
                    options: transactionOptions,
                },
            },
        );
        return JSON.parse(response).payload;
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
    async meltNativeToken(
        tokenId: string,
        meltAmount: HexEncodedAmount,
        transactionOptions?: TransactionOptions,
    ): Promise<Transaction> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'meltNativeToken',
                data: {
                    tokenId,
                    meltAmount,
                    options: transactionOptions,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    async deregisterParticipationEvent(
        eventId: ParticipationEventId,
    ): Promise<void> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'deregisterParticipationEvent',
                data: {
                    eventId,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Destroy an alias output.
     * @param aliasId The AliasId.
     * @param transactionOptions The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns The transaction.
     */
    async destroyAlias(
        aliasId: string,
        transactionOptions?: TransactionOptions,
    ): Promise<Transaction> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'destroyAlias',
                data: {
                    aliasId,
                    options: transactionOptions,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Function to destroy a foundry output with a circulating supply of 0.
     * Native tokens in the foundry (minted by other foundries) will be transactioned to the controlling alias.
     * @param foundryId The FoundryId.
     * @param transactionOptions The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns The transaction.
     */
    async destroyFoundry(
        foundryId: string,
        transactionOptions?: TransactionOptions,
    ): Promise<Transaction> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'destroyFoundry',
                data: {
                    foundryId,
                    options: transactionOptions,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Generate a new unused address.
     * @param options Options for address generation.
     * @returns The address.
     */
    async generateEd25519Address(
        options?: GenerateAddressOptions,
    ): Promise<Address> {
        const addresses = await this.generateEd25519Addresses(1, options);
        return addresses[0];
    }

    /**
     * Generate new unused addresses.
     * @param amount The amount of addresses to generate.
     * @param options Options for address generation.
     * @returns The addresses.
     */
    async generateEd25519Addresses(
        amount: number,
        options?: GenerateAddressOptions,
    ): Promise<Address[]> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'generateEd25519Addresses',
                data: {
                    amount,
                    options,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /** Generate EVM addresses */
    async generateEvmAddresses(
        generateAddressesOptions: GenerateAddressesOptions,
    ): Promise<string[]> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'generateEvmAddresses',
                data: {
                    options: generateAddressesOptions,
                },
            },
        );

        return JSON.parse(response).payload;
    }

    /**
     * Verifies an ed25519 signature against a message.
     */
    async verifyEd25519Signature(
        signature: Ed25519Signature,
        message: HexEncodedString,
    ): Promise<boolean> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'verifyEd25519Signature',
                data: {
                    signature,
                    message,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Verifies a Secp256k1Ecdsa signature against a message.
     */
    async verifySecp256k1EcdsaSignature(
        signature: Secp256k1EcdsaSignature,
        message: HexEncodedString,
    ): Promise<boolean> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'verifySecp256k1EcdsaSignature',
                data: {
                    publicKey: signature.publicKey,
                    signature: signature.signature,
                    message,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Signs a message with a Secp256k1Ecdsa private key.
     */
    async signSecp256k1Ecdsa(
        message: HexEncodedString,
        chain: Bip44,
    ): Promise<Secp256k1EcdsaSignature> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'signSecp256k1Ecdsa',
                data: {
                    message,
                    chain,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Get the account balance.
     * @returns The account balance.
     */
    async getBalance(): Promise<Balance> {
        const response = await this.messageHandler.callAccountMethod(
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
        const response = await this.messageHandler.callAccountMethod(
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
        const response = await this.messageHandler.callAccountMethod(
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
        node: Node,
        eventType?: ParticipationEventType,
    ): Promise<ParticipationEventId[]> {
        const response = await this.messageHandler.callAccountMethod(
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
        const response = await this.messageHandler.callAccountMethod(
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
        const response = await this.messageHandler.callAccountMethod(
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
        const response = await this.messageHandler.callAccountMethod(
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
    async claimableOutputs(outputs: OutputsToClaim): Promise<string[]> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'claimableOutputs',
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
        const response = await this.messageHandler.callAccountMethod(
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
    async getIncomingTransaction(transactionId: string): Promise<Transaction> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'getIncomingTransaction',
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
    async addresses(): Promise<Address[]> {
        const response = await this.messageHandler.callAccountMethod(
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
        const response = await this.messageHandler.callAccountMethod(
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
        const response = await this.messageHandler.callAccountMethod(
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
        const response = await this.messageHandler.callAccountMethod(
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
        const response = await this.messageHandler.callAccountMethod(
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
        const response = await this.messageHandler.callAccountMethod(
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
        const response = await this.messageHandler.callAccountMethod(
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
        const response = await this.messageHandler.callAccountMethod(
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
     * Mint additional native tokens.
     * @param tokenId The native token id.
     * @param mintAmount To be minted amount.
     * @param transactionOptions The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns The minting transaction.
     */
    async mintNativeToken(
        tokenId: string,
        mintAmount: HexEncodedAmount,
        transactionOptions?: TransactionOptions,
    ): Promise<Transaction> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'mintNativeToken',
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
     * Create a native token.
     * @param params The options for creating the token.
     * @param transactionOptions The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns The creating transaction and the token ID.
     */
    async createNativeToken(
        params: CreateNativeTokenParams,
        transactionOptions?: TransactionOptions,
    ): Promise<CreateNativeTokenTransaction> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'createNativeToken',
                data: {
                    params: params,
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
    async mintNfts(
        params: MintNftParams[],
        transactionOptions?: TransactionOptions,
    ): Promise<Transaction> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'mintNfts',
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
        const response = await this.messageHandler.callAccountMethod(
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
     * Prepare a send transaction, useful for offline signing.
     * @param params Address with amounts to send.
     * @param options The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns The prepared transaction data.
     */
    async prepareSend(
        params: SendParams[],
        options?: TransactionOptions,
    ): Promise<PreparedTransactionData> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'prepareSend',
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
        const response = await this.messageHandler.callAccountMethod(
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
        const response = await this.messageHandler.callAccountMethod(
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
     * Request funds from a faucet.
     */
    async requestFundsFromFaucet(
        url: string,
        address: string,
    ): Promise<string> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'requestFundsFromFaucet',
                data: { url, address },
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
        const response = await this.messageHandler.callAccountMethod(
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
     * Send a transaction with amounts from input addresses.
     * @param params Addresses with amounts.
     * @param transactionOptions The options to define a `RemainderValueStrategy`
     * or custom inputs.
     * @returns The sent transaction.
     */
    async send(
        params: SendParams[],
        transactionOptions?: TransactionOptions,
    ): Promise<Transaction> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'send',
                data: {
                    params,
                    options: transactionOptions,
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
    async sendNativeTokens(
        params: SendNativeTokensParams[],
        transactionOptions?: TransactionOptions,
    ): Promise<Transaction> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'sendNativeTokens',
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
    async sendNft(
        params: SendNftParams[],
        transactionOptions?: TransactionOptions,
    ): Promise<Transaction> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'sendNft',
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
        const response = await this.messageHandler.callAccountMethod(
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
        await this.messageHandler.callAccountMethod(this.meta.index, {
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
        await this.messageHandler.callAccountMethod(this.meta.index, {
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
        const response = await this.messageHandler.callAccountMethod(
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
     * Validate the transaction, submit it to a node and store it in the account.
     * @param signedTransactionData A signed transaction to submit and store.
     * @returns The sent transaction.
     */
    async submitAndStoreTransaction(
        signedTransactionData: SignedTransactionEssence,
    ): Promise<Transaction> {
        const response = await this.messageHandler.callAccountMethod(
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
    async sync(options?: SyncOptions): Promise<Balance> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'syncAccount',
                data: {
                    options,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    async vote(
        eventId?: ParticipationEventId,
        answers?: number[],
    ): Promise<Transaction> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'vote',
                data: {
                    eventId,
                    answers,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    async stopParticipating(
        eventId: ParticipationEventId,
    ): Promise<Transaction> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'stopParticipating',
                data: {
                    eventId,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    /**
     * Calculates the voting overview of an account.
     * @param eventIds Optional, filters participations only for provided events.
     * @returns ParticipationOverview
     */
    async getParticipationOverview(
        eventIds?: ParticipationEventId[],
    ): Promise<ParticipationOverview> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'getParticipationOverview',
                data: {
                    eventIds,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    async increaseVotingPower(amount: string): Promise<Transaction> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'increaseVotingPower',
                data: {
                    amount,
                },
            },
        );
        return JSON.parse(response).payload;
    }

    async decreaseVotingPower(amount: string): Promise<Transaction> {
        const response = await this.messageHandler.callAccountMethod(
            this.meta.index,
            {
                name: 'decreaseVotingPower',
                data: {
                    amount,
                },
            },
        );
        return JSON.parse(response).payload;
    }
}
