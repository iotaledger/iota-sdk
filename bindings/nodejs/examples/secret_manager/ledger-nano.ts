// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { initLogger, SecretManager } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./secret_manager/ledger-nano.ts

// In this example we will get the ledger status and generate an address
// To use the ledger nano simulator clone https://github.com/iotaledger/ledger-iota-app, run `git submodule init && git submodule update --recursive`,
// then `./build.sh -m nanos|nanox|nanosplus -s` and use `true` in `LedgerSecretManager::new(true)`.
async function run() {
    initLogger();

    try {
        const isSimulator = false;

        const ledgerNanoSecretManager = new SecretManager({
            ledgerNano: isSimulator,
        });

        const ledgerNanoStatus =
            await ledgerNanoSecretManager.getLedgerNanoStatus();

        console.log(ledgerNanoStatus);

        const address = await ledgerNanoSecretManager.generateEd25519Addresses({
            accountIndex: 0,
            range: {
                start: 0,
                end: 1,
            },
        });

        console.log('First public address:', address, '\n');
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
