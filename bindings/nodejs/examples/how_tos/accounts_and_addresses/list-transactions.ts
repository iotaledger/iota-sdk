// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/accounts_and_addresses/list-transactions.ts

// This example lists all transactions in the account.
async function run() {
    initLogger();
    for (const envVar of ['WALLET_DB_PATH']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }
    try {
        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        const account = await wallet.getAccount('Alice');
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
