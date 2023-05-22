// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';

// Run with command:
// yarn run-example ./how_tos/accounts_and_addresses/list_addresses.ts

// This example lists all addresses in the account
async function run() {
    initLogger();
    try {
        const wallet = new Wallet({
            storagePath: './alice-database',
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
