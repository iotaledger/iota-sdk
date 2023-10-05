// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/accounts_and_addresses/create-address.ts

// This example creates an address
async function run() {
    initLogger();
    for (const envVar of ['WALLET_DB_PATH', 'STRONGHOLD_PASSWORD'])
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }

    try {
        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        const account = await wallet.getAccount('Alice');

        // To create an address we need to unlock stronghold.
        await wallet.setStrongholdPassword(
            process.env.STRONGHOLD_PASSWORD as string,
        );

        const address = (await account.generateEd25519Addresses(1))[0];

        console.log(`Generated address:`, address.address);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
