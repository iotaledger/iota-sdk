// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// This example sends tokens to an address.
// Run with command:
// yarn run-example ./exchange/5-send-amount.ts

import { Wallet } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

async function run() {
    try {
        for (const envVar of [
            'WALLET_DB_PATH',
            'STRONGHOLD_PASSWORD',
            'EXPLORER_URL',
        ])
            if (!(envVar in process.env)) {
                throw new Error(
                    `.env ${envVar} is undefined, see .env.example`,
                );
            }

        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        await wallet.setStrongholdPassword(
            process.env.STRONGHOLD_PASSWORD as string,
        );

        const account = await wallet.getAccount('Alice');
        console.log('Account:', account);

        // Set syncOnlyMostBasicOutputs to true if not interested in outputs that are timelocked,
        // have a storage deposit return, expiration or are nft/alias/foundry outputs.
        await account.sync({ syncOnlyMostBasicOutputs: true });

        const response = await account.send(
            BigInt(1000000),
            // Replace with the address of your choice!
            'rms1qrrv7flg6lz5cssvzv2lsdt8c673khad060l4quev6q09tkm9mgtupgf0h0',
        );

        console.log(response);

        console.log(
            `Check your block on ${process.env.EXPLORER_URL}/block/${response.blockId}`,
        );
    } catch (error) {
        console.error(error);
    }
}

run().then(() => process.exit());
