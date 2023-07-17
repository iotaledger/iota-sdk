// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    AccountManager,
    CoinType,
    initLogger,
    ConsolidationRequiredWalletEvent,
    TransactionProgressWalletEvent,
    SelectingInputsProgress,
    Event,
} from '@iota/wallet';
require('dotenv').config({ path: '.env' });

// Run with command:
// ts-node events.ts

// This example creates a new database and account
async function run() {
    initLogger({
        name: './wallet.log',
        levelFilter: 'debug',
        targetExclusions: ["h2", "hyper", "rustls"]
    });
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }
    if (!process.env.STRONGHOLD_PASSWORD) {
        throw new Error(
            '.env STRONGHOLD_PASSWORD is undefined, see .env.example',
        );
    }
    try {
        const walletOptions = {
            storagePath: process.env.WALLET_DB_PATH,
            clientOptions: {
                nodes: [process.env.NODE_URL],
            },
            coinType: CoinType.Shimmer,
            secretManager: {
                stronghold: {
                    snapshotPath: process.env.STRONGHOLD_SNAPSHOT_PATH,
                    password: process.env.STRONGHOLD_PASSWORD,
                },
            },
        };

        const wallet = new AccountManager(walletOptions);

        const callback = function (err: any, event: Event) {
            console.log('AccountIndex:', event.getAccountIndex(), ', Event:', event.getEvent());
        };

        await wallet.listen([], callback);

        await wallet.emitTestEvent(new ConsolidationRequiredWalletEvent());
        await wallet.emitTestEvent(
            new TransactionProgressWalletEvent(new SelectingInputsProgress()),
        );

        await wallet.destroy();
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
