// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AccountManager } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example yarn run-example ./exchange/0-generate-mnemonic.ts 

// This example sends tokens to an address.
async function run() {
    try {
        const manager = new AccountManager({
            storagePath: './alice-database',
        });

        await manager.setStrongholdPassword(`${process.env.STRONGHOLD_PASSWORD}`)

        const account = await manager.getAccount('Alice');
        console.log('Account:', account);

        const response = await account.sendAmount([
            {
                //TODO: Replace with the address of your choice!
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
