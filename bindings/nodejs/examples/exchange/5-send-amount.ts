// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./exchange/5-send-amount.ts

// This example sends tokens to an address.
async function run() {
    try {
        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        await wallet.setStrongholdPassword(`${process.env.STRONGHOLD_PASSWORD}`)

        const account = await wallet.getAccount('Alice');
        console.log('Account:', account);

        const response = await account.send([
            {
                // Replace with the address of your choice!
                address: 'rms1qrrv7flg6lz5cssvzv2lsdt8c673khad060l4quev6q09tkm9mgtupgf0h0',
                amount: '1000000',
            },
        ]);

        console.log(response);

        console.log(
            `Check your block on https://explorer.testnet.shimmer.network/testnet/block/${response.blockId}`,
        );
    } catch (error) {
        console.log('Error: ', error);
    }
}

run().then(() => process.exit());
