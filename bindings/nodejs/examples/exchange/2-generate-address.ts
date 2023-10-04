// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// This example generates an address for an account.
// Run with command:
// yarn run-example ./exchange/2-generate-address.ts

import { Wallet } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

async function run() {
    try {
        for (const envVar of ['WALLET_DB_PATH', 'STRONGHOLD_PASSWORD'])
            if (!(envVar in process.env)) {
                throw new Error(
                    `.env ${envVar} is undefined, see .env.example`,
                );
            }

        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        await wallet.setStrongholdPassword(
            process.env.STRONGHOLD_PASSWORD as string,
        );

        const account = await wallet.getAccount('Alice');

        const address = (await account.generateEd25519Addresses(1))[0];

        console.log('Address:', address);
    } catch (error) {
        console.error(error);
    }
}

run().then(() => process.exit());
