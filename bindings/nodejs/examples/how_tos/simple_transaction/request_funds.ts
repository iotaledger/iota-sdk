// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/simple_transaction/request_funds.ts

// This example requests funds from the faucet
async function run() {
    initLogger();
    if (!process.env.FAUCET_URL) {
        throw new Error('.env FAUCET_URL is undefined, see .env.example');
    }
    try {
        const wallet = new Wallet({
            storagePath: './alice-database',
        });

        const account = await wallet.getAccount('Alice');

        const address = (await account.addresses())[0].address;
        console.log(address);

        const faucetResponse = await (
            await wallet.getClient()
        ).requestFundsFromFaucet(process.env.FAUCET_URL, address);
        console.log(faucetResponse);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
