// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./exchange/2-generate-address.ts

// This example generates an address for an account.
async function run() {
    try {
        if (!process.env.WALLET_DB_PATH) {
            throw new Error(
                '.env WALLET_DB_PATH is undefined, see .env.example',
            );
        }
        if (!process.env.STRONGHOLD_PASSWORD) {
            throw new Error(
                '.env STRONGHOLD_PASSWORD is undefined, see .env.example',
            );
        }

        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        await wallet.setStrongholdPassword(process.env.STRONGHOLD_PASSWORD);

        const account = await wallet.getAccount('Alice');

        const address = (await account.generateEd25519Addresses(1))[0];

        console.log('Address:', address);
    } catch (error) {
        console.error(error);
    }
}

run().then(() => process.exit());
