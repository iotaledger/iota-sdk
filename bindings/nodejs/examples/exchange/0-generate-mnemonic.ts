// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Utils } from '@iota/sdk';

// Run with command:
// yarn run-example ./exchange/0-generate-mnemonic.ts

// This example generates a new random mnemonic.
async function run() {
    try {
        // Set the generated mnemonic as env variable MNEMONIC so it can be used in the next examples.
        console.log('Mnemonic:', Utils.generateMnemonic());
    } catch (error) {
        console.log('Error: ', error);
    }
}

run().then(() => process.exit());
