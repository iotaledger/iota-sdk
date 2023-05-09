// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Utils } from '@iota/sdk';

// Run with command:
// node ./dist/utils/generate_mnemonic.js

// In this example we will generate a random BIP39 mnemonic
async function run() {
    try {
        const mnemonic = Utils.generateMnemonic();
        console.log('Mnemonic: ' + mnemonic);
        // Example output:
        // Mnemonic: endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
