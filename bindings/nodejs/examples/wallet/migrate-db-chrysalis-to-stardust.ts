// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { WalletOptions, Wallet, migrateDbChrysalisToStardust } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example wallet/migrate-db-chrysalis-to-stardust.ts

const walletDbPath = './chrysalis-db';

async function run() {
    const { initLogger } = require('@iota/sdk');
    initLogger({
        name: './wallet.log',
        levelFilter: 'debug',
        targetExclusions: ['h2', 'hyper', 'rustls'],
    });
    for (const envVar of ['NODE_URL', 'STRONGHOLD_PASSWORD']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }

    migrateDbChrysalisToStardust(walletDbPath, 'password');

    const walletOptions: WalletOptions = {
        storagePath: walletDbPath,
        clientOptions: {
            nodes: [process.env.NODE_URL as string],
        },
        secretManager: {
            stronghold: {
                snapshotPath: walletDbPath + 'wallet.stronghold',
                password: process.env.STRONGHOLD_PASSWORD,
            },
        },
    };
    console.log(walletOptions);
    const wallet = new Wallet(walletOptions);

    // Accounts migrated from the Chrysalis db
    const accounts = await wallet.getAccounts();
    console.log(accounts);

    const historicChrysalisData = await wallet.getChrysalisData();
    console.log(historicChrysalisData);
}

run().then(() => process.exit());
