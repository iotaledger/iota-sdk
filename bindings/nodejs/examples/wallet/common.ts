// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { initLogger, Wallet, CoinType, WalletOptions } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

async function getUnlockedWallet() {
    initLogger();
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }
    if (!process.env.STRONGHOLD_PASSWORD) {
        throw new Error(
            '.env STRONGHOLD_PASSWORD is undefined, see .env.example',
        );
    }
    if (!process.env.STRONGHOLD_SNAPSHOT_PATH) {
        throw new Error(
            '.env STRONGHOLD_SNAPSHOT_PATH is undefined, see .env.example',
        );
    }
    if (!process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1) {
        throw new Error(
            '.env NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1 is undefined, see .env.example',
        );
    }
    if (!process.env.WALLET_DB_PATH) {
        throw new Error('.env WALLET_DB_PATH is undefined, see .env.example');
    }

    const walletOptions: WalletOptions = {
        storagePath: `${process.env.WALLET_DB_PATH}`,
        clientOptions: {
            nodes: [process.env.NODE_URL],
        },
        coinType: CoinType.Shimmer,
        secretManager: {
            stronghold: {
                snapshotPath: `${process.env.STRONGHOLD_SNAPSHOT_PATH}`,
                password: `${process.env.STRONGHOLD_PASSWORD}`,
            },
        },
    };
    const wallet = new Wallet(walletOptions);

    return wallet;
}

export { getUnlockedWallet };
