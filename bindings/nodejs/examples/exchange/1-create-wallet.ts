// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// This example creates a new database and wallet.
// Run with command:
// yarn run-example ./exchange/1-create-wallet.ts

import { Wallet, WalletOptions, CoinType, SecretManager } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

async function run() {
    try {
        for (const envVar of [
            'WALLET_DB_PATH',
            'NODE_URL',
            'STRONGHOLD_SNAPSHOT_PATH',
            'STRONGHOLD_PASSWORD',
            'MNEMONIC',
        ])
            if (!(envVar in process.env)) {
                throw new Error(
                    `.env ${envVar} is undefined, see .env.example`,
                );
            }

        const strongholdSecretManager = {
            stronghold: {
                snapshotPath: process.env.STRONGHOLD_SNAPSHOT_PATH,
                password: process.env.STRONGHOLD_PASSWORD,
            },
        };

        const secretManager = SecretManager.create(strongholdSecretManager);

        // A mnemonic can be generated with `Utils.generateMnemonic()`.
        // Store the mnemonic in the Stronghold snapshot, this needs to be done only the first time.
        // The mnemonic can't be retrieved from the Stronghold file, so make a backup in a secure place!
        await secretManager.storeMnemonic(process.env.MNEMONIC as string);

        const walletAddress = await secretManager.generateEd25519Addresses({
            coinType: CoinType.IOTA,
            accountIndex: 0,
            range: {
                start: 0,
                end: 1,
            },
            bech32Hrp: 'tst',
        });

        const walletOptions: WalletOptions = {
            address: walletAddress[0],
            storagePath: process.env.WALLET_DB_PATH,
            clientOptions: {
                nodes: [process.env.NODE_URL as string],
            },
            bipPath: {
                coinType: CoinType.IOTA,
            },
            secretManager: strongholdSecretManager,
        };

        const wallet = await Wallet.create(walletOptions);

        // Set syncOnlyMostBasicOutputs to true if not interested in outputs that are timelocked,
        // have a storage deposit return, expiration or are nft/account/foundry outputs.
        await wallet.setDefaultSyncOptions({ syncOnlyMostBasicOutputs: true });
    } catch (error) {
        console.error(error);
    }
}

void run().then(() => process.exit());
