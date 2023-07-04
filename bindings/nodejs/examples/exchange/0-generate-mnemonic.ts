// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Utils } from '@iota/sdk';

// Run with command:
// yarn run-example ./exchange/0-generate-mnemonic.ts

// This example generates a new random mnemonic.
async function run() {
    try {
        console.log('Generated mnemonic:', Utils.generateMnemonic());
        // Set generated mnemonic as env variable for MNEMONIC so it can be used in 1-create-account.js
    } catch (error) {
        console.log('Error: ', error);
    }
}

run().then(() => process.exit());
