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
    if (!process.env.FAUCET_URL) {
        throw new Error('.env FAUCET_URL is undefined, see .env.example');
    }
    try {
        const faucetUrl = process.env.FAUCET_URL;

        // Create the wallet
        const wallet = await Wallet.create({
            storagePath: process.env.WALLET_DB_PATH,
        });

        const address = await wallet.address();
        console.log(address);

        const faucetResponse = await wallet
            .getClient()
            .requestFundsFromFaucet(faucetUrl, address);
        console.log(faucetResponse);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
