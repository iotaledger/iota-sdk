// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/accounts_and_addresses/list-outputs.ts

// This example lists all outputs in the account.
async function run() {
    initLogger();
    for (const envVar of ['WALLET_DB_PATH']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }
    try {
        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        const account = await wallet.getAccount('Alice');

        await account.sync();

        const outputs = await account.outputs();

        console.log('Output ids:');
        for (const output of outputs) console.log(output.outputId);

        const unspentOutputs = await account.unspentOutputs();

        console.log('Unspent output ids:');
        for (const output of unspentOutputs) console.log(output.outputId);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
