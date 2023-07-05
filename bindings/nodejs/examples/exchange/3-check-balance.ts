// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./exchange/3-check-balance.ts

// This example gets the balance of an account.
async function run() {
    try {
        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        const account = await wallet.getAccount('Alice');
        const addresses = await account.addresses();

        console.log('Addresses:', addresses);

        // Set syncOnlyMostBasicOutputs to true if not interested in outputs that are timelocked,
        // have a storage deposit return, expiration or are nft/alias/foundry outputs.
        const balance = await account.sync({ syncOnlyMostBasicOutputs: true });

        console.log('Balance', balance);

        // Use the Faucet to send testnet tokens to your address:
        console.log(
            'Fill your address with the Faucet: https://faucet.testnet.shimmer.network/',
        );
    } catch (error) {
        console.log('Error: ', error);
    }
}

run().then(() => process.exit());
