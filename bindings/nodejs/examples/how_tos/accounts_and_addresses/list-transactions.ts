// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/accounts_and_addresses/list-transactions.ts

// This example lists all transactions in the wallet.
async function run() {
    initLogger();
    for (const envVar of ['WALLET_DB_PATH']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }
    try {
        const wallet = await Wallet.create({
            storagePath: process.env.WALLET_DB_PATH,
        });
        await wallet.sync({ syncIncomingTransactions: true });

        const transactions = await wallet.transactions();
        console.log('Sent transactions:');
        for (const transaction of transactions)
            console.log(transaction.transactionId);

        const incomingTransactions = await wallet.incomingTransactions();
        console.log('Incoming transactions:');
        for (const transaction of incomingTransactions) {
            console.log(transaction.transactionId);
        }
    } catch (error) {
        console.error('Error: ', error);
    }
}

void run().then(() => process.exit());
