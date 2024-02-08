// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/wallet/list-accounts.ts

// This example lists all account outputs in the wallet.
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

        const accounts = await wallet.accounts();

        for (const account of accounts) console.log(account);
    } catch (error) {
        console.error('Error: ', error);
    }
}

void run().then(() => process.exit());