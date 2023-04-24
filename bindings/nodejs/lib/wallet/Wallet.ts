// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { WalletMethodHandler } from './WalletMethodHandler';
import { Account } from './Account';
import { getClient } from './bindings';

import type {
    AccountId,
    WalletOptions,
    CreateAccountPayload,
    EventType,
    GenerateAddressOptions,
    SyncOptions,
    WalletEvent,
} from '../../types/wallet';
import {
    IAuth,
    IClientOptions,
    INodeInfoWrapper,
    LedgerNanoStatus,
} from '../../types/client';
import { Client } from '../client';

/** The Wallet class. */
export class Wallet {
    private methodHandler: WalletMethodHandler;

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
        await this.methodHandler.destroy();
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
     * Generate an address without storing it.
     */
    async generateAddress(
        accountIndex: number,
        addressIndex: number,
        options?: GenerateAddressOptions,
        bech32Hrp?: string,
    ): Promise<string> {
        const response = await this.methodHandler.callMethod({
            name: 'generateAddress',
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
        eventTypes: EventType[],
        callback: (error: Error, result: string) => void,
    ): Promise<void> {
        return this.methodHandler.listen(eventTypes, callback);
    }

    /**
     * Clear the callbacks for provided events. An empty array will clear all listeners.
     */
    async clearListeners(eventTypes: EventType[]): Promise<void> {
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
     */
    async restoreBackup(
        source: string,
        password: string,
        ignoreIfCoinTypeMismatch?: boolean,
    ): Promise<void> {
        await this.methodHandler.callMethod({
            name: 'restoreBackup',
            data: {
                source,
                password,
                ignoreIfCoinTypeMismatch,
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
