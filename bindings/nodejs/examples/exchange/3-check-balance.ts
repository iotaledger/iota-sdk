// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AccountManager } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/accounts_and_addresses/check-balance.ts

// This example gets the balance of an account.
async function run() {
    try {
        const manager = new AccountManager({
            storagePath: './alice-database',
        });

        const account = await manager.getAccount('Alice');
        const addressObject = await account.addresses();
        console.log('Addresses before:', addressObject);

        // syncOnlyMostBasicOutputs if not interested in outputs that are timelocked, 
        // have a storage deposit return, expiration or are nft/alias/foundry outputs
        const synced = await account.sync({ syncOnlyMostBasicOutputs: true });
        console.log('Syncing... - ', synced);

        console.log('Available balance', await account.getBalance());

        // Use the Faucet to send testnet tokens to your address:
        console.log("Fill your address with the Faucet: https://faucet.testnet.shimmer.network/")
    } catch (error) {
        console.log('Error: ', error);
    }
}

run().then(() => process.exit());
