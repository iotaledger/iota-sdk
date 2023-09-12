// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { WalletMethodHandler } from './wallet-method-handler';
import { Account } from './account';

import type {
    AccountId,
    WalletOptions,
    CreateAccountPayload,
    WalletEventType,
    GenerateAddressOptions,
    SyncOptions,
    WalletEvent,
    Event,
} from '../types/wallet';
import { IAuth, IClientOptions, LedgerNanoStatus } from '../types/client';
import { Client } from '../client';
import { SecretManager } from '../secret_manager';

/** The Wallet class. */
export class Wallet {
    private methodHandler: WalletMethodHandler;

    /**
     * @param options Wallet options.
     */
    constructor(options: WalletOptions) {
        this.methodHandler = new WalletMethodHandler(options);
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
     * Create a new account.
     */
    async createAccount(data: CreateAccountPayload): Promise<Account> {
        const response = await this.methodHandler.callMethod({
            name: 'createAccount',
            data,
        });
        return new Account(JSON.parse(response).payload, this.methodHandler);
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
     * Get an account by its alias or index.
     */
    async getAccount(accountId: AccountId): Promise<Account> {
        const response = await this.methodHandler.callMethod({
            name: 'getAccount',
            data: { accountId },
        });

        const account = new Account(
            JSON.parse(response).payload,
            this.methodHandler,
        );

        return account;
    }

    /**
     * Get all account indexes.
     */
    async getAccountIndexes(): Promise<number[]> {
        const response = await this.methodHandler.callMethod({
            name: 'getAccountIndexes',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get all accounts.
     */
    async getAccounts(): Promise<Account[]> {
        const response = await this.methodHandler.callMethod({
            name: 'getAccounts',
        });

        const { payload } = JSON.parse(response);

        const accounts: Account[] = [];

        for (const account of payload) {
            accounts.push(new Account(account, this.methodHandler));
        }
        return accounts;
    }

    /**
     * Get client.
     */
    async getClient(): Promise<Client> {
        return this.methodHandler.getClient();
    }

    /**
     * Get chrysalis data.
     */
    async getChrysalisData(): Promise<Record<string, string>> {
        const response = await this.methodHandler.callMethod({
            name: 'getChrysalisData',
        });

        return JSON.parse(response).payload;
    }

    /**
     * Get secret manager.
     */
    async getSecretManager(): Promise<SecretManager> {
        return this.methodHandler.getSecretManager();
    }

    /**
     * Generate an address without storing it.
     */
    async generateEd25519Address(
        accountIndex: number,
        addressIndex: number,
        options?: GenerateAddressOptions,
        bech32Hrp?: string,
    ): Promise<string> {
        const response = await this.methodHandler.callMethod({
            name: 'generateEd25519Address',
            data: {
                accountIndex,
                addressIndex,
                options,
                bech32Hrp,
            },
        });
        return JSON.parse(response).payload;
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
     * Find accounts with unspent outputs.
     */
    async recoverAccounts(
        accountStartIndex: number,
        accountGapLimit: number,
        addressGapLimit: number,
        syncOptions: SyncOptions,
    ): Promise<Account[]> {
        const response = await this.methodHandler.callMethod({
            name: 'recoverAccounts',
            data: {
                accountStartIndex,
                accountGapLimit,
                addressGapLimit,
                syncOptions,
            },
        });
        const accounts: Account[] = [];

        for (const account of JSON.parse(response).payload) {
            accounts.push(new Account(account, this.methodHandler));
        }
        return accounts;
    }

    /**
     * Delete the latest account.
     */
    async removeLatestAccount(): Promise<void> {
        await this.methodHandler.callMethod({
            name: 'removeLatestAccount',
        });
    }

    /**
     * Restore a backup from a Stronghold file
     * Replaces client_options, coin_type, secret_manager and accounts. Returns an error if accounts were already created
     * If Stronghold is used as secret_manager, the existing Stronghold file will be overwritten. If a mnemonic was
     * stored, it will be gone.
     * if ignore_if_coin_type_mismatch is provided client options will not be restored
     * if ignore_if_coin_type_mismatch == true, client options coin type and accounts will not be restored if the cointype doesn't match
     * if ignore_if_bech32_hrp_mismatch == Some("rms"), but addresses have something different like "smr", no accounts
     * will be restored.
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
}
