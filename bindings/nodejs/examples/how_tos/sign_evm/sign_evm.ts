// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    CoinType,
    HD_WALLET_TYPE,
    HARDEN_MASK,
    initLogger,
    SecretManager,
    utf8ToHex,
} from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// In this example we will sign with secp256k1.
// Run with command:
// yarn run-example ./how_tos/sign_evm/sign_evm.ts

const FOUNDRY_METADATA = {
    standard: 'IRC30',
    name: 'NativeToken',
    description: 'A native token',
    symbol: 'NT',
    decimals: 6,
    logoUrl: 'https://my.website/nativeToken.png',
};
const ACCOUNT_INDEX = 0;
const INTERNAL_ADDRESS = false;
const ADDRESS_INDEX = 0;

async function run() {
    initLogger();

    try {
        if (!process.env.STRONGHOLD_PASSWORD) {
            throw new Error(
                '.env stronghold password is undefined, see .env.example',
            );
        }
        if (!process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1) {
            throw new Error('.env mnemonic is undefined, see .env.example');
        }
        const secretManager = new SecretManager({
            stronghold: {
                password: process.env.STRONGHOLD_PASSWORD,
                snapshotPath: 'sign_evm.stronghold',
            },
        });

        // A mnemonic can be generated with `Utils.generateMnemonic()`.
        // Store the mnemonic in the Stronghold snapshot, this needs to be done only the first time.
        // The mnemonic can't be retrieved from the Stronghold file, so make a backup in a secure place!
        await secretManager.storeMnemonic(
            process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1,
        );

        const bip32Chain = [
            (HD_WALLET_TYPE | HARDEN_MASK) >>> 0,
            (CoinType.Ether | HARDEN_MASK) >>> 0,
            (ACCOUNT_INDEX | HARDEN_MASK) >>> 0,
            INTERNAL_ADDRESS ? 1 : 0,
            ADDRESS_INDEX,
        ];
        const message = utf8ToHex(JSON.stringify(FOUNDRY_METADATA));
        const evmSignature = await secretManager.signEvm(message, bip32Chain);

        console.log(`Public key: ${evmSignature.publicKey}`);
        console.log(`Signature: ${evmSignature.signature}`);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
