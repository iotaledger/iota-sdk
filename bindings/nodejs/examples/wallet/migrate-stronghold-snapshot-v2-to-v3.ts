// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    CoinType,
    WalletOptions,
    Wallet,
    migrateStrongholdSnapshotV2ToV3,
} from '@iota/sdk';
require('dotenv').config({ path: '.env' });

const v2Path = '../../../sdk/tests/wallet/fixtures/v2.stronghold';
const v3Path = './v3.stronghold';

// Run with command:
// yarn run-example wallet/migrate-stronghold-snapshot-v2-to-v3.ts

async function run() {
    for (const envVar of ['NODE_URL', 'WALLET_DB_PATH']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }

    let walletOptions: WalletOptions = {
        storagePath: process.env.WALLET_DB_PATH,
        clientOptions: {
            nodes: [process.env.NODE_URL as string],
        },
        coinType: CoinType.Shimmer,
        secretManager: {
            stronghold: {
                snapshotPath: v2Path,
                password: 'current_password',
            },
        },
    };

    try {
        // This should fail with error, migration required.
        new Wallet(walletOptions);
    } catch (error) {
        console.error(error);
    }

    migrateStrongholdSnapshotV2ToV3(
        v2Path,
        'current_password',
        'wallet.rs',
        100,
        v3Path,
        'new_password',
    );

    walletOptions = {
        storagePath: process.env.WALLET_DB_PATH,
        clientOptions: {
            nodes: [process.env.NODE_URL as string],
        },
        coinType: CoinType.Shimmer,
        secretManager: {
            stronghold: {
                snapshotPath: v3Path,
                password: 'new_password',
            },
        },
    };

    // This shouldn't fail anymore as snapshot has been migrated.
    new Wallet(walletOptions);
}

run().then(() => process.exit());
