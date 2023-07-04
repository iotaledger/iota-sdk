// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AccountManager } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/accounts_and_addresses/check-balance.ts

// This example generates an address for an account.
async function run() {
    try {
        const manager = new AccountManager({
            storagePath: './alice-database',
        });

        await manager.setStrongholdPassword(
            `${process.env.STRONGHOLD_PASSWORD}`,
        );

        const account = await manager.getAccount('Alice');

        const address = await account.generateEd25519Address();

        console.log('Address generated:', address);
    } catch (error) {
        console.log('Error: ', error);
    }
}

run().then(() => process.exit());
