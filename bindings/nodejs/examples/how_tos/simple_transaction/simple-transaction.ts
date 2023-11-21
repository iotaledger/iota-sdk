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
        for (const envVar of [
            'STRONGHOLD_PASSWORD',
            'WALLET_DB_PATH',
            'EXPLORER_URL',
        ]) {
            if (!(envVar in process.env)) {
                throw new Error(`.env ${envVar} is not defined`);
            }
        }

        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        const account = await wallet.getAccount('Alice');

        await account.sync();

        // To sign a transaction we need to unlock stronghold.
        await wallet.setStrongholdPassword(
            process.env.STRONGHOLD_PASSWORD as string,
        );

        // Replace with the address of your choice!
        const address =
            'rms1qrrv7flg6lz5cssvzv2lsdt8c673khad060l4quev6q09tkm9mgtupgf0h0';
        const amount = BigInt(1000000);

        const response = await account.send(amount, address);

        console.log(
            `Block sent: ${process.env.EXPLORER_URL}/block/${response.blockId}`,
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
