// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// This example generates a new random mnemonic.
// Run with command:
// yarn run-example ./exchange/0-generate-mnemonic.ts

import { Utils } from '@iota/sdk';

async function run() {
    try {
        // Set the generated mnemonic as env variable MNEMONIC so it can be used in the next examples.
        console.log('Mnemonic:', Utils.generateMnemonic());
    } catch (error) {
        console.error(error);
    }
}

run().then(() => process.exit());
