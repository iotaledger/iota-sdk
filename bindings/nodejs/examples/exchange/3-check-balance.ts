// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// This example gets the balance of an account.
// Run with command:
// yarn run-example ./exchange/3-check-balance.ts

import { Wallet } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

async function run() {
    try {
        for (const envVar of ['WALLET_DB_PATH']) {
            if (!(envVar in process.env)) {
                throw new Error(
                    `.env ${envVar} is undefined, see .env.example`,
                );
            }
        }

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

        // Use the faucet to send tokens to your address.
        console.log(
            'Fill your address with the Faucet: https://faucet.testnet.shimmer.network/',
        );
    } catch (error) {
        console.error(error);
    }
}

run().then(() => process.exit());
