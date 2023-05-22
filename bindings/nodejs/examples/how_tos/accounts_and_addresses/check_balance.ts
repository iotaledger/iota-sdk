// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./wallet/02_get_balance.ts

// This example syncs the account and prints the balance
async function run() {
    initLogger();
    try {
        const wallet = new Wallet({
            storagePath: './alice-database',
        });

        const account = await wallet.getAccount('Alice');

        // Sync new outputs from the node.
        // eslint-disable-next-line no-unused-vars
        const syncBalance = await account.sync();

        // After syncing the balance can also be computed with the local data
        const balance = await account.getBalance();
        console.log('AccountBalance', balance);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
