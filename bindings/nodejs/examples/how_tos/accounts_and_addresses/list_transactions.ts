// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';

// Run with command:
// yarn run-example ./how_tos/accounts_and_addresses/list_transactions.ts

// This example lists all transactions in the account
async function run() {
    initLogger();
    try {
        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        const account = await wallet.getAccount(
            `${process.env.ACCOUNT_ALIAS_1}`,
        );
        await account.sync({ syncIncomingTransactions: true });

        const transactions = await account.transactions();
        console.log('Sent transactions:');
        for (const transaction of transactions)
            console.log(transaction.transactionId);

        const incomingTransactions = await account.incomingTransactions();
        console.log('Incoming transactions:');
        for (const transaction of incomingTransactions) {
            console.log(transaction.transactionId);
        }
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
