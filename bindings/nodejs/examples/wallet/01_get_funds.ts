// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, Utils, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// node ./dist/wallet/01_get_funds.js

// This example requests funds from the faucet
async function run() {
    initLogger();
    try {
        const wallet = new Wallet({
            storagePath: './alice-database',
        });

        const account = await wallet.getAccount('Alice');

        const accountAddresses = await account.addresses();
        console.log('Account addresses:', accountAddresses);

        const faucetResponse = Utils.requestFundsFromFaucet(
            process.env.FAUCET_URL,
            accountAddresses[0].address,
        );
        console.log(faucetResponse);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
