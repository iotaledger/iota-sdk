// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Utils } from '@iota/sdk';

// Run with command:
// yarn run-example ./how_tos/accounts_and_addresses/create-mnemonic.ts

// In this example we will generate a random BIP39 mnemonic
async function run() {
    try {
        const mnemonic = Utils.generateMnemonic();
        console.log('Mnemonic: ' + mnemonic);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
