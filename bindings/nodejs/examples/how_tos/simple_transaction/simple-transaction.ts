// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/simple_transaction/simple-transaction.ts

// This example syncs the account and prints the balance
async function run() {
    initLogger();
    try {
        if (!process.env.STRONGHOLD_PASSWORD) {
            throw new Error(
                '.env STRONGHOLD_PASSWORD is undefined, see .env.example',
            );
        }

        const wallet = await Wallet.create({
            storagePath: process.env.WALLET_DB_PATH,
        });

        await wallet.sync();

        // To sign a transaction we need to unlock stronghold.
        await wallet.setStrongholdPassword(process.env.STRONGHOLD_PASSWORD);

        // Replace with the address of your choice!
        const address =
            'rms1qrrv7flg6lz5cssvzv2lsdt8c673khad060l4quev6q09tkm9mgtupgf0h0';
        const amount = BigInt(1000000);

        const response = await wallet.send(amount, address);

        console.log(
            `Block sent: ${process.env.EXPLORER_URL}/block/${response.blockId}`,
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
