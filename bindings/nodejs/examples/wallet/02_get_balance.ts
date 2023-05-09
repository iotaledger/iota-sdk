// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// node ./dist/wallet/02_get_balance.js

// This example syncs the account and prints the balance
async function run() {
    initLogger();
    try {
        const wallet = new Wallet({
            storagePath: './alice-database',
        });

        const account = await wallet.getAccount('Alice');

        // Sync new outputs from the node.
        const balance = await account.sync();
        console.log('Account balance:', balance);

        // After syncing the balance can also be computed with the local data
        const balance2 = await account.getBalance();
        console.log('Account balance:', balance2);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
