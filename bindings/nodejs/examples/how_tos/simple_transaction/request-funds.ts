// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/simple_transaction/request-funds.ts

// This example requests funds from the faucet
async function run() {
    initLogger();
    for (const envVar of ['FAUCET_URL', 'WALLET_DB_PATH']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is not defined`);
        }
    }
    try {
        const faucetUrl = process.env.FAUCET_URL as string;

        // Create the wallet
        const wallet = await Wallet.create({
            storagePath: process.env.WALLET_DB_PATH,
        });

        const address = await wallet.address();
        console.log(address);

        const client = await wallet.getClient();
        const faucetResponse = await client.requestFundsFromFaucet(
            faucetUrl,
            address,
        );
        console.log(faucetResponse);
    } catch (error) {
        console.error('Error: ', error);
    }
}

void run().then(() => process.exit());
