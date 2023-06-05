// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    AccountManager,
    CoinType,
    initLogger,
    ConsolidationRequiredWalletEvent,
    TransactionProgressWalletEvent,
    SelectingInputsProgress,
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
            storagePath: './alice-database',
            clientOptions: {
                nodes: [process.env.NODE_URL],
            },
            coinType: CoinType.Shimmer,
            secretManager: {
                stronghold: {
                    snapshotPath: `./wallet.stronghold`,
                    password: `${process.env.STRONGHOLD_PASSWORD}`,
                },
            },
        };

        const wallet = new AccountManager(walletOptions);

        const callback = function (err: any, data: string) {
            // don't know if something like this could make sense
            // const walletEvent: typeof WalletEvent = JSON.parse(data);
            console.log('data:', JSON.parse(data));
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
