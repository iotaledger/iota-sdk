// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { WalletMethodHandler } from './wallet-method-handler';
import {
    Balance,
    SyncOptions,
    SendParams,
    SendNativeTokenParams,
    SendNftParams,
    AccountOutputParams,
    FilterOptions,
    CreateNativeTokenParams,
    MintNftParams,
    OutputData,
    OutputParams,
    OutputsToClaim,
    TransactionWithMetadata,
    TransactionOptions,
    ParticipationOverview,
    ParticipationEventId,
    ParticipationEventStatus,
    ParticipationEventType,
    ParticipationEventWithNodes,
    ParticipationEventRegistrationOptions,
    ParticipationEventMap,
    SignedTransactionData,
    PreparedTransaction,
    PreparedCreateNativeTokenTransactionData,
    ConsolidationParams,
} from '../types/wallet';
import { Client, INode, Burn, PreparedTransactionData } from '../client';
import {
    Output,
    FoundryOutput,
    Response,
    PreparedCreateNativeTokenTransaction,
    u64,
    u256,
    NftId,
    TokenId,
    OutputId,
    AccountId,
    FoundryId,
    TransactionId,
    NumericString,
    Bech32Address,
} from '../types';
import { plainToInstance } from 'class-transformer';
import { bigIntToHex, hexToBigInt } from '../types/utils/hex-encoding';
import type {
    WalletOptions,
    WalletEventType,
    WalletEvent,
    Event,
} from '../types/wallet';
import { IAuth, IClientOptions, LedgerNanoStatus } from '../types/client';
import { SecretManager } from '../secret_manager';

/** The Wallet class. */
export class Wallet {
    private methodHandler: WalletMethodHandler;

    /**
     * @param methodHandler The Rust method handler created in `WalletMethodHandler.create()`.
     */
    constructor(methodHandler: WalletMethodHandler) {
        this.methodHandler = methodHandler;
    }

    /**
     * @param options The wallet options.
     */
    static async create(options: WalletOptions): Promise<Wallet> {
        return new Wallet(await WalletMethodHandler.create(options));
    }

    /**
     * Backup the data to a Stronghold snapshot.
     */
    async backup(destination: string, password: string): Promise<void> {
        await this.methodHandler.callMethod({
            name: 'backup',
            data: {
                destination,
                password,
            },
        });
    }

    /**
     * Change the Stronghold password.
     */
    async changeStrongholdPassword(
        currentPassword: string,
        newPassword: string,
    ): Promise<void> {
        await this.methodHandler.callMethod({
            name: 'changeStrongholdPassword',
            data: {
                currentPassword,
                newPassword,
            },
        });
    }

    /**
     * Clear the Stronghold password from memory.
     */
    async clearStrongholdPassword(): Promise<void> {
        await this.methodHandler.callMethod({
            name: 'clearStrongholdPassword',
        });
    }

    /**
     * Destroy the Wallet and drop its database connection.
     */
    async destroy(): Promise<void> {
        return this.methodHandler.destroy();
    }

    /**
     * Emit a provided event for testing of the event system.
     */
    async emitTestEvent(event: WalletEvent): Promise<void> {
        await this.methodHandler.callMethod({
            name: 'emitTestEvent',
            data: { event },
        });
    }

    /**
     * Get client.
     */
    async getClient(): Promise<Client> {
        return this.methodHandler.getClient();
    }

    /**
     * Get secret manager.
     */
    async getSecretManager(): Promise<SecretManager> {
        return this.methodHandler.getSecretManager();
    }

    /**
     * Get the status for a Ledger Nano.
     */
    async getLedgerNanoStatus(): Promise<LedgerNanoStatus> {
        const response = await this.methodHandler.callMethod({
            name: 'getLedgerNanoStatus',
        });
        return JSON.parse(response).payload;
    }

    /**
     * Check if the Stronghold password has been set.
     */
    async isStrongholdPasswordAvailable(): Promise<boolean> {
        const response = await this.methodHandler.callMethod({
            name: 'isStrongholdPasswordAvailable',
        });
        return JSON.parse(response).payload;
    }

    /**
     * Listen to wallet events with a callback. An empty array will listen to all possible events.
     */
    async listen(
        eventTypes: WalletEventType[],
        callback: (error: Error, event: Event) => void,
    ): Promise<void> {
        return this.methodHandler.listen(eventTypes, callback);
    }

    /**
     * Clear the callbacks for provided events. An empty array will clear all listeners.
     */
    async clearListeners(eventTypes: WalletEventType[]): Promise<void> {
        const response = await this.methodHandler.callMethod({
            name: 'clearListeners',
            data: { eventTypes },
        });
        return JSON.parse(response).payload;
    }

    /**
     * Restore a backup from a Stronghold file
     * Replaces client_options, coin_type, secret_manager and accounts. Returns an error if accounts were already created
     * If Stronghold is used as secret_manager, the existing Stronghold file will be overwritten. If a mnemonic was
     * stored, it will be gone.
     * if ignore_if_coin_type_mismatch is provided client options will not be restored
     * if ignore_if_coin_type_mismatch == true, client options coin type and accounts will not be restored if the cointype doesn't match
     * If a bech32 hrp is provided to ignore_if_bech32_hrp_mismatch, that doesn't match the one of the current address, the wallet will not be restored.
     */
    async restoreBackup(
        source: string,
        password: string,
        ignoreIfCoinTypeMismatch?: boolean,
        ignoreIfBech32Mismatch?: string,
    ): Promise<void> {
        await this.methodHandler.callMethod({
            name: 'restoreBackup',
            data: {
                source,
                password,
                ignoreIfCoinTypeMismatch,
                ignoreIfBech32Mismatch,
            },
        });
    }

    /**
     * Set ClientOptions.
     */
    async setClientOptions(clientOptions: IClientOptions): Promise<void> {
        await this.methodHandler.callMethod({
            name: 'setClientOptions',
            data: { clientOptions },
        });
    }

    /**
     * Set the Stronghold password.
     */
    async setStrongholdPassword(password: string): Promise<void> {
        await this.methodHandler.callMethod({
            name: 'setStrongholdPassword',
            data: { password },
        });
    }

    /**
     * Set the interval after which the Stronghold password gets cleared from memory.
     */
    async setStrongholdPasswordClearInterval(
        intervalInMilliseconds?: number,
    ): Promise<void> {
        await this.methodHandler.callMethod({
            name: 'setStrongholdPasswordClearInterval',
            data: { intervalInMilliseconds },
        });
    }

    /**
     * Start the background syncing process for all accounts.
     */
    async startBackgroundSync(
        options?: SyncOptions,
        intervalInMilliseconds?: number,
    ): Promise<void> {
        await this.methodHandler.callMethod({
            name: 'startBackgroundSync',
            data: {
                options,
                intervalInMilliseconds,
            },
        });
    }

    /**
     * Stop the background syncing process for all accounts.
     */
    async stopBackgroundSync(): Promise<void> {
        await this.methodHandler.callMethod({
            name: 'stopBackgroundSync',
        });
    }

    /**
     * Store a mnemonic in the Stronghold snapshot.
     */
    async storeMnemonic(mnemonic: string): Promise<void> {
        await this.methodHandler.callMethod({
            name: 'storeMnemonic',
            data: { mnemonic },
        });
    }

    /**
     * Update the authentication for the provided node.
     */
    async updateNodeAuth(url: string, auth?: IAuth): Promise<void> {
        await this.methodHandler.callMethod({
            name: 'updateNodeAuth',
            data: { url, auth },
        });
    }

    /**
     * Returns the accounts of the wallet.
     *
     * @returns The accounts of the wallet.
     */
    async accounts(): Promise<OutputData[]> {
        const response = await this.methodHandler.callMethod({
            name: 'accounts',
        });

        const parsed = JSON.parse(response) as Response<OutputData[]>;
        return plainToInstance(OutputData, parsed.payload);
    }

    /**
     * A generic function that can be used to burn native tokens, nfts, foundries and accounts.
     * @param burn The outputs or native tokens to burn
     * @param transactionOptions Additional transaction options
     * or custom inputs.
     * @returns The transaction.
     */
    async burn(
        burn: Burn,
        transactionOptions?: TransactionOptions,
    ): Promise<TransactionWithMetadata> {
        return (await this.prepareBurn(burn, transactionOptions)).send();
    }

    /**
     * A generic function that can be used to prepare to burn native tokens, nfts, foundries and accounts.
     * @param burn The outputs or native tokens to burn
     * @param transactionOptions Additional transaction options
     * @returns The prepared transaction.
     */
    async prepareBurn(
        burn: Burn,
        transactionOptions?: TransactionOptions,
    ): Promise<PreparedTransaction> {
        const response = await this.methodHandler.callMethod({
            name: 'prepareBurn',
            data: {
                burn,
                options: transactionOptions,
            },
        });
        const parsed = JSON.parse(
            response,
        ) as Response<PreparedTransactionData>;
        return new PreparedTransaction(
            plainToInstance(PreparedTransactionData, parsed.payload),
            this,
        );
    }

    /**
     * Burn native tokens. This doesn't require the foundry output which minted them, but will not increase
     * the foundries `melted_tokens` field, which makes it impossible to destroy the foundry output. Therefore it's
     * recommended to use melting, if the foundry output is available.
     * @param tokenId The native token id.
     * @param burnAmount The to be burned amount.
     * @param transactionOptions Additional transaction options
     * or custom inputs.
     * @returns The prepared transaction.
     */
    async prepareBurnNativeToken(
        tokenId: TokenId,
        burnAmount: u256,
        transactionOptions?: TransactionOptions,
    ): Promise<PreparedTransaction> {
        const response = await this.methodHandler.callMethod({
            name: 'prepareBurn',
            data: {
                burn: {
                    nativeTokens: new Map([[tokenId, burnAmount]]),
                },
                options: transactionOptions,
            },
        });
        const parsed = JSON.parse(
            response,
        ) as Response<PreparedTransactionData>;
        return new PreparedTransaction(
            plainToInstance(PreparedTransactionData, parsed.payload),
            this,
        );
    }

    /**
     * Burn an nft output.
     * @param nftId The NftId.
     * @param transactionOptions Additional transaction options
     * or custom inputs.
     * @returns The prepared transaction.
     */
    async prepareBurnNft(
        nftId: NftId,
        transactionOptions?: TransactionOptions,
    ): Promise<PreparedTransaction> {
        const response = await this.methodHandler.callMethod({
            name: 'prepareBurn',
            data: {
                burn: {
                    nfts: [nftId],
                },
                options: transactionOptions,
            },
        });
        const parsed = JSON.parse(
            response,
        ) as Response<PreparedTransactionData>;
        return new PreparedTransaction(
            plainToInstance(PreparedTransactionData, parsed.payload),
            this,
        );
    }

    /**
     * Claim basic or nft outputs that have additional unlock conditions
     * to their `AddressUnlockCondition` from the wallet.
     * @param outputIds The outputs to claim.
     * @returns The resulting transaction.
     */
    async claimOutputs(
        outputIds: OutputId[],
    ): Promise<TransactionWithMetadata> {
        return (await this.prepareClaimOutputs(outputIds)).send();
    }

    /**
     * Claim basic or nft outputs that have additional unlock conditions
     * to their `AddressUnlockCondition` from the wallet.
     * @param outputIds The outputs to claim.
     * @returns The prepared transaction.
     */
    async prepareClaimOutputs(
        outputIds: OutputId[],
    ): Promise<PreparedTransaction> {
        const response = await this.methodHandler.callMethod({
            name: 'prepareClaimOutputs',
            data: {
                outputIdsToClaim: outputIds,
            },
        });
        const parsed = JSON.parse(
            response,
        ) as Response<PreparedTransactionData>;
        return new PreparedTransaction(
            plainToInstance(PreparedTransactionData, parsed.payload),
            this,
        );
    }

    /**
     * Consolidate basic outputs with only an `AddressUnlockCondition` from a wallet
     * by sending them to an own address again if the output amount is greater or
     * equal to the output consolidation threshold.
     * @param params Consolidation options.
     * @returns The consolidation transaction.
     */
    async consolidateOutputs(
        params: ConsolidationParams,
    ): Promise<TransactionWithMetadata> {
        return (await this.prepareConsolidateOutputs(params)).send();
    }

    /**
     * Consolidate basic outputs with only an `AddressUnlockCondition` from a wallet
     * by sending them to an own address again if the output amount is greater or
     * equal to the output consolidation threshold.
     * @param params Consolidation options.
     * @returns The prepared consolidation transaction.
     */
    async prepareConsolidateOutputs(
        params: ConsolidationParams,
    ): Promise<PreparedTransaction> {
        const response = await this.methodHandler.callMethod({
            name: 'prepareConsolidateOutputs',
            data: {
                params,
            },
        });
        const parsed = JSON.parse(
            response,
        ) as Response<PreparedTransactionData>;
        return new PreparedTransaction(
            plainToInstance(PreparedTransactionData, parsed.payload),
            this,
        );
    }

    /**
     * Creates an account output.
     * @param params The account output options.
     * @param transactionOptions Additional transaction options
     * or custom inputs.
     * @returns The transaction.
     */
    async createAccountOutput(
        params?: AccountOutputParams,
        transactionOptions?: TransactionOptions,
    ): Promise<TransactionWithMetadata> {
        return (
            await this.prepareCreateAccountOutput(params, transactionOptions)
        ).send();
    }

    /**
     * Creates an account output.
     * @param params The account output options.
     * @param transactionOptions Additional transaction options
     * or custom inputs.
     * @returns The prepared transaction.
     */
    async prepareCreateAccountOutput(
        params?: AccountOutputParams,
        transactionOptions?: TransactionOptions,
    ): Promise<PreparedTransaction> {
        const response = await this.methodHandler.callMethod({
            name: 'prepareCreateAccountOutput',
            data: {
                params,
                options: transactionOptions,
            },
        });
        const parsed = JSON.parse(
            response,
        ) as Response<PreparedTransactionData>;
        return new PreparedTransaction(
            plainToInstance(PreparedTransactionData, parsed.payload),
            this,
        );
    }

    /**
     * Melt native tokens. This happens with the foundry output which minted them, by increasing its
     * `melted_tokens` field.
     * @param tokenId The native token id.
     * @param meltAmount To be melted amount.
     * @param transactionOptions Additional transaction options
     * or custom inputs.
     * @returns The transaction.
     */
    async meltNativeToken(
        tokenId: TokenId,
        meltAmount: bigint,
        transactionOptions?: TransactionOptions,
    ): Promise<TransactionWithMetadata> {
        return (
            await this.prepareMeltNativeToken(
                tokenId,
                meltAmount,
                transactionOptions,
            )
        ).send();
    }

    /**
     * Melt native tokens. This happens with the foundry output which minted them, by increasing its
     * `melted_tokens` field.
     * @param tokenId The native token id.
     * @param meltAmount To be melted amount.
     * @param transactionOptions Additional transaction options
     * or custom inputs.
     * @returns The prepared transaction.
     */
    async prepareMeltNativeToken(
        tokenId: TokenId,
        meltAmount: u256,
        transactionOptions?: TransactionOptions,
    ): Promise<PreparedTransaction> {
        const response = await this.methodHandler.callMethod({
            name: 'prepareMeltNativeToken',
            data: {
                tokenId,
                meltAmount: bigIntToHex(meltAmount),
                options: transactionOptions,
            },
        });
        const parsed = JSON.parse(
            response,
        ) as Response<PreparedTransactionData>;
        return new PreparedTransaction(
            plainToInstance(PreparedTransactionData, parsed.payload),
            this,
        );
    }

    /**
     * Deregister a participation event.
     *
     * @param eventId The id of the participation event to deregister.
     */
    async deregisterParticipationEvent(
        eventId: ParticipationEventId,
    ): Promise<void> {
        const response = await this.methodHandler.callMethod({
            name: 'deregisterParticipationEvent',
            data: {
                eventId,
            },
        });
        return JSON.parse(response).payload;
    }

    /**
     * Destroy an account output.
     *
     * @param accountId The AccountId.
     * @param transactionOptions Additional transaction options
     * or custom inputs.
     * @returns The prepared transaction.
     */
    async prepareDestroyAccount(
        accountId: AccountId,
        transactionOptions?: TransactionOptions,
    ): Promise<PreparedTransaction> {
        const response = await this.methodHandler.callMethod({
            name: 'prepareBurn',
            data: {
                burn: {
                    accounts: [accountId],
                },
                options: transactionOptions,
            },
        });
        const parsed = JSON.parse(
            response,
        ) as Response<PreparedTransactionData>;
        return new PreparedTransaction(
            plainToInstance(PreparedTransactionData, parsed.payload),
            this,
        );
    }

    /**
     * Function to destroy a foundry output with a circulating supply of 0.
     * Native tokens in the foundry (minted by other foundries) will be transacted to the controlling account.
     * @param foundryId The FoundryId.
     * @param transactionOptions Additional transaction options
     * or custom inputs.
     * @returns The prepared transaction.
     */
    async prepareDestroyFoundry(
        foundryId: FoundryId,
        transactionOptions?: TransactionOptions,
    ): Promise<PreparedTransaction> {
        const response = await this.methodHandler.callMethod({
            name: 'prepareBurn',
            data: {
                burn: {
                    foundries: [foundryId],
                },
                options: transactionOptions,
            },
        });
        const parsed = JSON.parse(
            response,
        ) as Response<PreparedTransactionData>;
        return new PreparedTransaction(
            plainToInstance(PreparedTransactionData, parsed.payload),
            this,
        );
    }

    /**
     * Get the account balance.
     *
     * @returns The account balance.
     */
    async getBalance(): Promise<Balance> {
        const response = await this.methodHandler.callMethod({
            name: 'getBalance',
        });
        const payload = JSON.parse(response).payload;
        return this.adjustBalancePayload(payload);
    }

    /**
     * Converts hex encoded or decimal strings of amounts to `bigint`
     * for the balance payload.
     */
    private adjustBalancePayload(payload: any): Balance {
        for (let i = 0; i < payload.nativeTokens.length; i++) {
            payload.nativeTokens[i].total = hexToBigInt(
                payload.nativeTokens[i].total,
            );
            payload.nativeTokens[i].available = hexToBigInt(
                payload.nativeTokens[i].available,
            );
        }
        payload.baseCoin.total = BigInt(payload.baseCoin.total);
        payload.baseCoin.available = BigInt(payload.baseCoin.available);

        payload.requiredStorageDeposit.account = BigInt(
            payload.requiredStorageDeposit.account,
        );
        payload.requiredStorageDeposit.basic = BigInt(
            payload.requiredStorageDeposit.basic,
        );
        payload.requiredStorageDeposit.foundry = BigInt(
            payload.requiredStorageDeposit.foundry,
        );
        payload.requiredStorageDeposit.nft = BigInt(
            payload.requiredStorageDeposit.nft,
        );

        return payload;
    }

    /**
     * Get the data for an output.
     * @param outputId The output to get.
     * @returns The `OutputData`.
     */
    async getOutput(outputId: OutputId): Promise<OutputData> {
        const response = await this.methodHandler.callMethod({
            name: 'getOutput',
            data: {
                outputId,
            },
        });
        const parsed = JSON.parse(response) as Response<OutputData>;
        return plainToInstance(OutputData, parsed.payload);
    }

    /**
     * Get a participation event.
     *
     * @param eventId The ID of the event to get.
     */
    async getParticipationEvent(
        eventId: ParticipationEventId,
    ): Promise<ParticipationEventWithNodes> {
        const response = await this.methodHandler.callMethod({
            name: 'getParticipationEvent',
            data: {
                eventId,
            },
        });
        return JSON.parse(response).payload;
    }

    /**
     * Get IDs of participation events of a certain type.
     *
     * @param node The node to get events from.
     * @param eventType The type of events to get.
     */
    async getParticipationEventIds(
        node: INode,
        eventType?: ParticipationEventType,
    ): Promise<ParticipationEventId[]> {
        const response = await this.methodHandler.callMethod({
            name: 'getParticipationEventIds',
            data: {
                node,
                eventType,
            },
        });
        return JSON.parse(response).payload;
    }

    /**
     * Get all participation events.
     */
    async getParticipationEvents(): Promise<ParticipationEventMap> {
        const response = await this.methodHandler.callMethod({
            name: 'getParticipationEvents',
        });
        return JSON.parse(response).payload;
    }

    /**
     * Get the participation event status by its ID.
     *
     * @param eventId The ID of the event status to get.
     */
    async getParticipationEventStatus(
        eventId: ParticipationEventId,
    ): Promise<ParticipationEventStatus> {
        const response = await this.methodHandler.callMethod({
            name: 'getParticipationEventStatus',
            data: {
                eventId,
            },
        });
        return JSON.parse(response).payload;
    }

    /**
     * Get a `FoundryOutput` by native token ID. It will try to get the foundry from
     * the account, if it isn't in the wallet it will try to get it from the node.
     *
     * @param tokenId The native token ID to get the foundry for.
     * @returns The `FoundryOutput` that minted the token.
     */
    async getFoundryOutput(tokenId: TokenId): Promise<FoundryOutput> {
        const response = await this.methodHandler.callMethod({
            name: 'getFoundryOutput',
            data: {
                tokenId,
            },
        });
        return Output.parse(JSON.parse(response).payload) as FoundryOutput;
    }

    /**
     * Get outputs with additional unlock conditions.
     *
     * @param outputs The type of outputs to claim.
     * @returns The output IDs of the unlockable outputs.
     */
    async claimableOutputs(outputs: OutputsToClaim): Promise<OutputId[]> {
        const response = await this.methodHandler.callMethod({
            name: 'claimableOutputs',
            data: {
                outputsToClaim: outputs,
            },
        });
        return JSON.parse(response).payload;
    }

    /**
     * Get a transaction stored in the wallet.
     *
     * @param transactionId The ID of the transaction to get.
     * @returns The transaction.
     */
    async getTransaction(
        transactionId: TransactionId,
    ): Promise<TransactionWithMetadata> {
        const response = await this.methodHandler.callMethod({
            name: 'getTransaction',
            data: {
                transactionId,
            },
        });
        const parsed = JSON.parse(
            response,
        ) as Response<TransactionWithMetadata>;
        return plainToInstance(TransactionWithMetadata, parsed.payload);
    }

    /**
     * Get the transaction with inputs of an incoming transaction stored in the wallet
     * List might not be complete, if the node pruned the data already
     *
     * @param transactionId The ID of the transaction to get.
     * @returns The transaction.
     */
    async getIncomingTransaction(
        transactionId: TransactionId,
    ): Promise<TransactionWithMetadata> {
        const response = await this.methodHandler.callMethod({
            name: 'getIncomingTransaction',
            data: {
                transactionId,
            },
        });
        const parsed = JSON.parse(
            response,
        ) as Response<TransactionWithMetadata>;
        return plainToInstance(TransactionWithMetadata, parsed.payload);
    }

    /**
     * Get the address of the wallet.
     *
     * @returns The address.
     */
    async address(): Promise<Bech32Address> {
        const response = await this.methodHandler.callMethod({
            name: 'getAddress',
        });

        return JSON.parse(response).payload;
    }

    /**
     * List all outputs of the wallet.
     *
     * @param filterOptions Options to filter the to be returned outputs.
     * @returns The outputs with metadata.
     */
    async outputs(filterOptions?: FilterOptions): Promise<OutputData[]> {
        const response = await this.methodHandler.callMethod({
            name: 'outputs',
            data: { filterOptions },
        });

        const parsed = JSON.parse(response) as Response<OutputData[]>;
        return plainToInstance(OutputData, parsed.payload);
    }

    /**
     * List all the pending transactions of the wallet.
     *
     * @returns The transactions.
     */
    async pendingTransactions(): Promise<TransactionWithMetadata[]> {
        const response = await this.methodHandler.callMethod({
            name: 'pendingTransactions',
        });
        const parsed = JSON.parse(response) as Response<
            TransactionWithMetadata[]
        >;
        return plainToInstance(TransactionWithMetadata, parsed.payload);
    }

    /**
     * Returns the implicit account creation address of the wallet if it is Ed25519 based.
     *
     * @returns The implicit account creation address.
     */
    async implicitAccountCreationAddress(): Promise<Bech32Address> {
        const response = await this.methodHandler.callMethod({
            name: 'implicitAccountCreationAddress',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Transitions an implicit account to an account.
     *
     * @param outputId Identifier of the implicit account output.
     * @returns The created transaction.
     */
    async implicitAccountTransition(
        outputId: OutputId,
    ): Promise<TransactionWithMetadata> {
        return (await this.prepareImplicitAccountTransition(outputId)).send();
    }

    /**
     * Prepares to transition an implicit account to an account.
     *
     * @param outputId Identifier of the implicit account output.
     * @returns The prepared transaction.
     */
    async prepareImplicitAccountTransition(
        outputId: OutputId,
    ): Promise<PreparedTransaction> {
        const response = await this.methodHandler.callMethod({
            name: 'prepareImplicitAccountTransition',
            data: { outputId },
        });

        const parsed = JSON.parse(
            response,
        ) as Response<PreparedTransactionData>;
        return new PreparedTransaction(
            plainToInstance(PreparedTransactionData, parsed.payload),
            this,
        );
    }

    /**
     * Returns the implicit accounts of the wallet.
     *
     * @returns The implicit accounts of the wallet.
     */
    async implicitAccounts(): Promise<OutputData[]> {
        const response = await this.methodHandler.callMethod({
            name: 'implicitAccounts',
        });

        const parsed = JSON.parse(response) as Response<OutputData[]>;
        return plainToInstance(OutputData, parsed.payload);
    }

    /**
     * List all incoming transactions of the wallet.
     *
     * @returns The incoming transactions with their inputs.
     */
    async incomingTransactions(): Promise<TransactionWithMetadata[]> {
        const response = await this.methodHandler.callMethod({
            name: 'incomingTransactions',
        });
        const parsed = JSON.parse(response) as Response<
            TransactionWithMetadata[]
        >;
        return plainToInstance(TransactionWithMetadata, parsed.payload);
    }

    /**
     * List all the transactions of the wallet.
     *
     * @returns The transactions.
     */
    async transactions(): Promise<TransactionWithMetadata[]> {
        const response = await this.methodHandler.callMethod({
            name: 'transactions',
        });
        const parsed = JSON.parse(response) as Response<
            TransactionWithMetadata[]
        >;
        return plainToInstance(TransactionWithMetadata, parsed.payload);
    }

    /**
     * List all the unspent outputs of the wallet.
     *
     * @param filterOptions Options to filter the to be returned outputs.
     * @returns The outputs with metadata.
     */
    async unspentOutputs(filterOptions?: FilterOptions): Promise<OutputData[]> {
        const response = await this.methodHandler.callMethod({
            name: 'unspentOutputs',
            data: { filterOptions },
        });
        const parsed = JSON.parse(response) as Response<OutputData[]>;
        return plainToInstance(OutputData, parsed.payload);
    }

    /**
     * Mint additional native tokens.
     *
     * @param tokenId The native token id.
     * @param mintAmount To be minted amount.
     * @param transactionOptions Additional transaction options
     * or custom inputs.
     * @returns The minting transaction.
     */
    async mintNativeToken(
        tokenId: TokenId,
        mintAmount: bigint,
        transactionOptions?: TransactionOptions,
    ): Promise<TransactionWithMetadata> {
        return (
            await this.prepareMintNativeToken(
                tokenId,
                mintAmount,
                transactionOptions,
            )
        ).send();
    }

    /**
     * Mint additional native tokens.
     *
     * @param tokenId The native token id.
     * @param mintAmount To be minted amount.
     * @param transactionOptions Additional transaction options
     * or custom inputs.
     * @returns The prepared minting transaction.
     */
    async prepareMintNativeToken(
        tokenId: string,
        mintAmount: u256,
        transactionOptions?: TransactionOptions,
    ): Promise<PreparedTransaction> {
        const response = await this.methodHandler.callMethod({
            name: 'prepareMintNativeToken',
            data: {
                tokenId,
                mintAmount: bigIntToHex(mintAmount),
                options: transactionOptions,
            },
        });

        const parsed = JSON.parse(
            response,
        ) as Response<PreparedTransactionData>;
        return new PreparedTransaction(
            plainToInstance(PreparedTransactionData, parsed.payload),
            this,
        );
    }

    /**
     * Create a native token.
     *
     * @param params The options for creating a native token.
     * @param transactionOptions Additional transaction options
     * or custom inputs.
     * @returns The created transaction.
     */
    async createNativeToken(
        params: CreateNativeTokenParams,
        transactionOptions?: TransactionOptions,
    ): Promise<TransactionWithMetadata> {
        return (
            await this.prepareCreateNativeToken(params, transactionOptions)
        ).send();
    }

    /**
     * Create a native token.
     *
     * @param params The options for creating a native token.
     * @param transactionOptions Additional transaction options
     * or custom inputs.
     * @returns The created transaction and the token ID.
     */
    async prepareCreateNativeToken(
        params: CreateNativeTokenParams,
        transactionOptions?: TransactionOptions,
    ): Promise<PreparedCreateNativeTokenTransaction> {
        const adjustedParams: any = params;
        adjustedParams.circulatingSupply = bigIntToHex(
            params.circulatingSupply,
        );
        adjustedParams.maximumSupply = bigIntToHex(params.maximumSupply);

        const response = await this.methodHandler.callMethod({
            name: 'prepareCreateNativeToken',
            data: {
                params: adjustedParams,
                options: transactionOptions,
            },
        });

        const parsed = JSON.parse(
            response,
        ) as Response<PreparedCreateNativeTokenTransactionData>;
        return new PreparedCreateNativeTokenTransaction(
            plainToInstance(
                PreparedCreateNativeTokenTransactionData,
                parsed.payload,
            ),
            this,
        );
    }

    /**
     * Mint NFTs.
     *
     * @param params The options for minting nfts.
     * @param transactionOptions Additional transaction options
     * or custom inputs.
     * @returns The minting transaction.
     */
    async mintNfts(
        params: MintNftParams[],
        transactionOptions?: TransactionOptions,
    ): Promise<TransactionWithMetadata> {
        return (await this.prepareMintNfts(params, transactionOptions)).send();
    }

    /**
     * Mint NFTs.
     *
     * @param params The options for minting nfts.
     * @param transactionOptions Additional transaction options
     * or custom inputs.
     * @returns The prepared minting transaction.
     */
    async prepareMintNfts(
        params: MintNftParams[],
        transactionOptions?: TransactionOptions,
    ): Promise<PreparedTransaction> {
        const response = await this.methodHandler.callMethod({
            name: 'prepareMintNfts',
            data: {
                params,
                options: transactionOptions,
            },
        });

        const parsed = JSON.parse(
            response,
        ) as Response<PreparedTransactionData>;
        return new PreparedTransaction(
            plainToInstance(PreparedTransactionData, parsed.payload),
            this,
        );
    }

    /**
     * Prepare an output for sending, useful for offline signing.
     *
     * @param options The options for preparing an output. If the amount is
     * below the minimum required storage deposit, by default the remaining
     * amount will automatically be added with a `StorageDepositReturn` `UnlockCondition`,
     * when setting the `ReturnStrategy` to `gift`, the full minimum required
     * storage deposit will be sent to the recipient. When the assets contain
     * an nft id, the data from the existing `NftOutput` will be used, just with
     * the address unlock conditions replaced.
     * @param transactionOptions Additional transaction options
     * or custom inputs.
     * @returns The prepared output.
     */
    async prepareOutput(
        params: OutputParams,
        transactionOptions?: TransactionOptions,
    ): Promise<Output> {
        if (typeof params.amount === 'bigint') {
            params.amount = params.amount.toString(10);
        }

        const response = await this.methodHandler.callMethod({
            name: 'prepareOutput',
            data: {
                params,
                transactionOptions,
            },
        });

        return Output.parse(JSON.parse(response).payload);
    }

    /**
     * Prepare to send base coins, useful for offline signing.
     *
     * @param params Address with amounts to send.
     * @param options Additional transaction options
     * or custom inputs.
     * @returns The prepared transaction data.
     */
    async prepareSend(
        params: SendParams[],
        options?: TransactionOptions,
    ): Promise<PreparedTransaction> {
        for (let i = 0; i < params.length; i++) {
            if (typeof params[i].amount === 'bigint') {
                params[i].amount = params[i].amount.toString(10);
            }
        }
        const response = await this.methodHandler.callMethod({
            name: 'prepareSend',
            data: {
                params,
                options,
            },
        });
        const parsed = JSON.parse(
            response,
        ) as Response<PreparedTransactionData>;
        return new PreparedTransaction(
            plainToInstance(PreparedTransactionData, parsed.payload),
            this,
        );
    }

    /**
     * Send a transaction.
     *
     * @param outputs Outputs to use in the transaction.
     * @param options Additional transaction options
     * or custom inputs.
     * @returns The transaction data.
     */
    async sendTransaction(
        outputs: Output[],
        options?: TransactionOptions,
    ): Promise<TransactionWithMetadata> {
        return (await this.prepareTransaction(outputs, options)).send();
    }

    /**
     * Prepare a transaction, useful for offline signing.
     *
     * @param outputs Outputs to use in the transaction.
     * @param options Additional transaction options
     * or custom inputs.
     * @returns The prepared transaction data.
     */
    async prepareTransaction(
        outputs: Output[],
        options?: TransactionOptions,
    ): Promise<PreparedTransaction> {
        const response = await this.methodHandler.callMethod({
            name: 'prepareTransaction',
            data: {
                outputs,
                options,
            },
        });
        const parsed = JSON.parse(
            response,
        ) as Response<PreparedTransactionData>;
        return new PreparedTransaction(
            plainToInstance(PreparedTransactionData, parsed.payload),
            this,
        );
    }

    /**
     * Register participation events.
     *
     * @param options Options to register participation events.
     * @returns A mapping between event IDs and their corresponding event data.
     */
    async registerParticipationEvents(
        options: ParticipationEventRegistrationOptions,
    ): Promise<ParticipationEventMap> {
        const response = await this.methodHandler.callMethod({
            name: 'registerParticipationEvents',
            data: {
                options,
            },
        });
        return JSON.parse(response).payload;
    }

    /**
     * Reissues a transaction sent from the wallet for a provided transaction id until it's
     * included (referenced by a milestone). Returns the included block id.
     */
    async reissueTransactionUntilIncluded(
        transactionId: string,
        interval?: number,
        maxAttempts?: number,
    ): Promise<string> {
        const response = await this.methodHandler.callMethod({
            name: 'reissueTransactionUntilIncluded',
            data: {
                transactionId,
                interval,
                maxAttempts,
            },
        });
        return JSON.parse(response).payload;
    }

    /**
     * Send base coins to an address.
     *
     * @param amount Amount of coins.
     * @param address Receiving address.
     * @param transactionOptions Additional transaction options
     * or custom inputs.
     * @returns The sent transaction.
     */
    async send(
        amount: u64 | NumericString,
        address: Bech32Address,
        transactionOptions?: TransactionOptions,
    ): Promise<TransactionWithMetadata> {
        if (typeof amount === 'bigint') {
            amount = amount.toString(10);
        }
        const response = await this.methodHandler.callMethod({
            name: 'send',
            data: {
                amount,
                address,
                options: transactionOptions,
            },
        });
        const parsed = JSON.parse(
            response,
        ) as Response<TransactionWithMetadata>;
        return plainToInstance(TransactionWithMetadata, parsed.payload);
    }

    /**
     * Send base coins with amounts from input addresses.
     *
     * @param params Addresses with amounts.
     * @param transactionOptions Additional transaction options
     * or custom inputs.
     * @returns The sent transaction.
     */
    async sendWithParams(
        params: SendParams[],
        transactionOptions?: TransactionOptions,
    ): Promise<TransactionWithMetadata> {
        for (let i = 0; i < params.length; i++) {
            if (typeof params[i].amount === 'bigint') {
                params[i].amount = params[i].amount.toString(10);
            }
        }
        const response = await this.methodHandler.callMethod({
            name: 'sendWithParams',
            data: {
                params,
                options: transactionOptions,
            },
        });
        const parsed = JSON.parse(
            response,
        ) as Response<TransactionWithMetadata>;
        return plainToInstance(TransactionWithMetadata, parsed.payload);
    }

    /**
     * Send native tokens.
     *
     * @param params Addresses amounts and native tokens.
     * @param transactionOptions Additional transaction options
     * or custom inputs.
     * @returns The transaction.
     */
    async sendNativeTokens(
        params: SendNativeTokenParams[],
        transactionOptions?: TransactionOptions,
    ): Promise<TransactionWithMetadata> {
        return (
            await this.prepareSendNativeTokens(params, transactionOptions)
        ).send();
    }

    /**
     * Send native tokens.
     *
     * @param params Addresses amounts and native tokens.
     * @param transactionOptions Additional transaction options
     * or custom inputs.
     * @returns The prepared transaction.
     */
    async prepareSendNativeTokens(
        params: SendNativeTokenParams[],
        transactionOptions?: TransactionOptions,
    ): Promise<PreparedTransaction> {
        const response = await this.methodHandler.callMethod({
            name: 'prepareSendNativeTokens',
            data: {
                params,
                options: transactionOptions,
            },
        });
        const parsed = JSON.parse(
            response,
        ) as Response<PreparedTransactionData>;
        return new PreparedTransaction(
            plainToInstance(PreparedTransactionData, parsed.payload),
            this,
        );
    }

    /**
     * Send NFT.
     *
     * @param params Addresses and nft ids.
     * @param transactionOptions Additional transaction options
     * or custom inputs.
     * @returns The transaction.
     */
    async sendNft(
        params: SendNftParams[],
        transactionOptions?: TransactionOptions,
    ): Promise<TransactionWithMetadata> {
        return (await this.prepareSendNft(params, transactionOptions)).send();
    }

    /**
     * Send NFT.
     *
     * @param params Addresses and nft ids.
     * @param transactionOptions Additional transaction options
     * or custom inputs.
     * @returns The prepared transaction.
     */
    async prepareSendNft(
        params: SendNftParams[],
        transactionOptions?: TransactionOptions,
    ): Promise<PreparedTransaction> {
        const response = await this.methodHandler.callMethod({
            name: 'prepareSendNft',
            data: {
                params,
                options: transactionOptions,
            },
        });
        const parsed = JSON.parse(
            response,
        ) as Response<PreparedTransactionData>;
        return new PreparedTransaction(
            plainToInstance(PreparedTransactionData, parsed.payload),
            this,
        );
    }

    /**
     * Send outputs in a transaction.
     *
     * @param outputs The outputs to send.
     * @param transactionOptions Additional transaction options
     * or custom inputs.
     * @returns The sent transaction.
     */
    async sendOutputs(
        outputs: Output[],
        transactionOptions?: TransactionOptions,
    ): Promise<TransactionWithMetadata> {
        const response = await this.methodHandler.callMethod({
            name: 'sendOutputs',
            data: {
                outputs,
                options: transactionOptions,
            },
        });

        const parsed = JSON.parse(
            response,
        ) as Response<TransactionWithMetadata>;
        return plainToInstance(TransactionWithMetadata, parsed.payload);
    }

    /**
     * Set the alias for the account
     *
     * @param alias The account alias to set.
     */
    async setAlias(alias: string): Promise<void> {
        await this.methodHandler.callMethod({
            name: 'setAlias',
            data: {
                alias,
            },
        });
    }

    /**
     * Set the fallback SyncOptions for account syncing.
     * If storage is enabled, will persist during restarts.
     *
     * @param options The sync options to set.
     */
    async setDefaultSyncOptions(options: SyncOptions): Promise<void> {
        await this.methodHandler.callMethod({
            name: 'setDefaultSyncOptions',
            data: {
                options,
            },
        });
    }

    /**
     * Sign a prepared transaction, useful for offline signing.
     *
     * @param preparedTransactionData The prepared transaction data to sign.
     * @returns The signed transaction.
     */
    async signTransaction(
        preparedTransactionData: PreparedTransactionData,
    ): Promise<SignedTransactionData> {
        const response = await this.methodHandler.callMethod({
            name: 'signTransaction',
            data: {
                preparedTransactionData,
            },
        });
        const parsed = JSON.parse(response) as Response<SignedTransactionData>;
        return plainToInstance(SignedTransactionData, parsed.payload);
    }

    /**
     * Sign a prepared transaction, and send it.
     *
     * @param preparedTransactionData The prepared transaction data to sign and submit.
     * @returns The transaction.
     */
    async signAndSubmitTransaction(
        preparedTransactionData: PreparedTransactionData,
    ): Promise<TransactionWithMetadata> {
        const response = await this.methodHandler.callMethod({
            name: 'signAndSubmitTransaction',
            data: {
                preparedTransactionData,
            },
        });
        const parsed = JSON.parse(
            response,
        ) as Response<TransactionWithMetadata>;
        return plainToInstance(TransactionWithMetadata, parsed.payload);
    }

    /**
     * Validate the transaction, submit it to a node and store it in the wallet.
     *
     * @param signedTransactionData A signed transaction to submit and store.
     * @returns The sent transaction.
     */
    async submitAndStoreTransaction(
        signedTransactionData: SignedTransactionData,
    ): Promise<TransactionWithMetadata> {
        const response = await this.methodHandler.callMethod({
            name: 'submitAndStoreTransaction',
            data: {
                signedTransactionData,
            },
        });
        const parsed = JSON.parse(
            response,
        ) as Response<TransactionWithMetadata>;
        return plainToInstance(TransactionWithMetadata, parsed.payload);
    }

    /**
     * Sync the account by fetching new information from the nodes.
     * Will also reissue pending transactions if necessary.
     * A custom default can be set using setDefaultSyncOptions.
     *
     * @param options Optional synchronization options.
     * @returns The account balance.
     */
    async sync(options?: SyncOptions): Promise<Balance> {
        const response = await this.methodHandler.callMethod({
            name: 'sync',
            data: {
                options,
            },
        });
        const payload = JSON.parse(response).payload;
        return this.adjustBalancePayload(payload);
    }

    /**
     * Prepare a vote.
     *
     * @param eventId The participation event ID.
     * @param answers Answers for a voting event, can be empty.
     * @returns An instance of `PreparedTransaction`.
     */
    async prepareVote(
        eventId?: ParticipationEventId,
        answers?: number[],
    ): Promise<PreparedTransaction> {
        const response = await this.methodHandler.callMethod({
            name: 'prepareVote',
            data: {
                eventId,
                answers,
            },
        });
        const parsed = JSON.parse(
            response,
        ) as Response<PreparedTransactionData>;
        return new PreparedTransaction(
            plainToInstance(PreparedTransactionData, parsed.payload),
            this,
        );
    }

    /**
     * Prepare stop participating in an event.
     *
     * @param eventId The event ID to stop participating in.
     * @returns An instance of `PreparedTransaction`.
     */
    async prepareStopParticipating(
        eventId: ParticipationEventId,
    ): Promise<PreparedTransaction> {
        const response = await this.methodHandler.callMethod({
            name: 'prepareStopParticipating',
            data: {
                eventId,
            },
        });
        const parsed = JSON.parse(
            response,
        ) as Response<PreparedTransactionData>;
        return new PreparedTransaction(
            plainToInstance(PreparedTransactionData, parsed.payload),
            this,
        );
    }

    /**
     * Calculates the voting overview of a wallet.
     *
     * @param eventIds Optional, filters participations only for provided events.
     * @returns An instance of `ParticipationOverview`
     */
    async getParticipationOverview(
        eventIds?: ParticipationEventId[],
    ): Promise<ParticipationOverview> {
        const response = await this.methodHandler.callMethod({
            name: 'getParticipationOverview',
            data: {
                eventIds,
            },
        });
        return JSON.parse(response).payload;
    }

    /**
     * Prepare to increase the voting power.
     *
     * @param amount The amount to increase the voting power by.
     * @returns An instance of `PreparedTransaction`.
     */
    async prepareIncreaseVotingPower(
        amount: NumericString,
    ): Promise<PreparedTransaction> {
        const response = await this.methodHandler.callMethod({
            name: 'prepareIncreaseVotingPower',
            data: {
                amount,
            },
        });
        const parsed = JSON.parse(
            response,
        ) as Response<PreparedTransactionData>;
        return new PreparedTransaction(
            plainToInstance(PreparedTransactionData, parsed.payload),
            this,
        );
    }

    /**
     * Prepare to decrease the voting power.
     *
     * @param amount The amount to decrease the voting power by.
     * @returns An instance of `PreparedTransaction`.
     */
    async prepareDecreaseVotingPower(
        amount: NumericString,
    ): Promise<PreparedTransaction> {
        const response = await this.methodHandler.callMethod({
            name: 'prepareDecreaseVotingPower',
            data: {
                amount,
            },
        });
        const parsed = JSON.parse(
            response,
        ) as Response<PreparedTransactionData>;
        return new PreparedTransaction(
            plainToInstance(PreparedTransactionData, parsed.payload),
            this,
        );
    }
}
