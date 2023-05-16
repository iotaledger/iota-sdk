// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';

// Run with command:
// yarn run-example ./how_tos/accounts_and_addresses/list_addresses.ts

// This example lists all addresses in the account
async function run() {
    initLogger();
    try {
        const wallet = new Wallet({
            storagePath: './alice-database',
        });

        const account = await wallet.getAccount('Alice');
        await account.sync({ syncIncomingTransactions: true });

        const transactions = await account.transactions();
        console.log('Sent transactions:');
        for (const transaction of transactions)
            console.log(transaction.transactionId);

        const incomingTransactions = await account.incomingTransactions();
        console.log('Incoming transactions:');
        for (const transaction of incomingTransactions)
            console.log(transaction[0]);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
