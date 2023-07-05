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
        if (!process.env.WALLET_DB_PATH) {
            throw new Error(
                '.env WALLET_DB_PATH is undefined, see .env.example',
            );
        }
        if (!process.env.STRONGHOLD_PASSWORD) {
            throw new Error(
                '.env STRONGHOLD_PASSWORD is undefined, see .env.example',
            );
        }

        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        await wallet.setStrongholdPassword(
            `${process.env.STRONGHOLD_PASSWORD}`,
        );

        const account = await wallet.getAccount('Alice');
        console.log('Account:', account);

        // Set syncOnlyMostBasicOutputs to true if not interested in outputs that are timelocked,
        // have a storage deposit return, expiration or are nft/alias/foundry outputs.
        await account.sync({ syncOnlyMostBasicOutputs: true });

        const response = await account.send([
            {
                // Replace with the address of your choice!
                address:
                    'rms1qrrv7flg6lz5cssvzv2lsdt8c673khad060l4quev6q09tkm9mgtupgf0h0',
                amount: '1000000',
            },
        ]);

        console.log(response);

        console.log(
            `Check your block on https://explorer.shimmer.network/testnet/block/${response.blockId}`,
        );
    } catch (error) {
        console.error(error);
    }
}

run().then(() => process.exit());
