// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Ed25519Address, Utils, utf8ToHex } from '@iota/sdk';

// In this example we will verify an Ed25519 signature.
// Run with command:
// yarn run-example ./how_tos/sign_and_verify_ed25519/verify_ed25519_signature.ts

const FOUNDRY_METADATA = {
    standard: 'IRC30',
    name: 'NativeToken',
    description: 'A native token',
    symbol: 'NT',
    decimals: 6,
    logoUrl: 'https://my.website/nativeToken.png',
};
const PUBLIC_KEY =
    '0x67b7fc3f78763c9394fc4fcdb52cf3a973b6e064bdc3defb40a6cb2c880e6f5c';
const ED25519_SIGNATURE =
    '0x5437ee671f182507103c6ae2f6649083475019f2cc372e674be164577dd123edd7a76291ba88732bbe1fae39688b50a3678bce05c9ef32c9494b3968f4f07a01';

function run() {
    try {
        const bech32Address = Utils.hexPublicKeyToBech32Address(
            PUBLIC_KEY,
            'rms',
        );
        const address = Utils.parseBech32Address(
            bech32Address,
        ) as Ed25519Address;

        const message = utf8ToHex(JSON.stringify(FOUNDRY_METADATA));
        const validSignature = Utils.verifyEd25519Signature(
            { type: 0, publicKey: PUBLIC_KEY, signature: ED25519_SIGNATURE },
            message,
            address,
        );
        console.log('Valid signature: ' + validSignature);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run();
