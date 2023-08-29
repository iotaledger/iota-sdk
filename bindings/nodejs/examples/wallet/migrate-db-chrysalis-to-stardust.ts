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
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }
    if (!process.env.STRONGHOLD_PASSWORD) {
        throw new Error(
            '.env STRONGHOLD_PASSWORD is undefined, see .env.example',
        );
    }

    migrateDbChrysalisToStardust(walletDbPath, 'password');

    let walletOptions: WalletOptions = {
        storagePath: walletDbPath,
        clientOptions: {
            nodes: [process.env.NODE_URL],
        },
        secretManager: {
            stronghold: {
                snapshotPath: walletDbPath + 'wallet.stronghold',
                password: process.env.STRONGHOLD_PASSWORD,
            },
        },
    };
    console.log(walletOptions);
    let wallet = new Wallet(walletOptions);

    // Accounts migrated from the Chrysalis db
    let accounts = await wallet.getAccounts();
    console.log(accounts);

    let historicChrysalisData = await wallet.getChrysalisData();
    console.log(historicChrysalisData);
}

run().then(() => process.exit());
