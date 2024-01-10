// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { migrateStrongholdSnapshotV2ToV3, SecretManager } from '@iota/sdk';
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

    const strongholdSecretManager = {
        stronghold: {
            snapshotPath: process.env.STRONGHOLD_SNAPSHOT_PATH,
            password: process.env.STRONGHOLD_PASSWORD,
        },
    };

    try {
        // This should fail with error, migration required.
        SecretManager.create(strongholdSecretManager);
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

    // This shouldn't fail anymore as snapshot has been migrated.
    SecretManager.create(strongholdSecretManager);
}

void run().then(() => process.exit());
