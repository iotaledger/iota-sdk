// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Client,
    initLogger,
    Utils,
    UnlockCondition,
    AddressUnlockCondition,
    MetadataFeature,
    SenderFeature,
    TagFeature,
    StorageDepositReturnUnlockCondition,
    Ed25519Address,
    ExpirationUnlockCondition,
    TimelockUnlockCondition,
} from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// node ./dist/client/11_build_output.js

// Build a basic output
async function run() {
    initLogger();
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }

    const client = new Client({
        nodes: [process.env.NODE_URL],
    });

    try {
        const hexAddress = Utils.bech32ToHex(
            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
        );

        const addressUnlockCondition: UnlockCondition =
            new AddressUnlockCondition(new Ed25519Address(hexAddress));

        // Build most basic output with amound and a single address unlock condition
        const basicOutput = await client.buildBasicOutput({
            amount: '1000000',
            unlockConditions: [addressUnlockCondition],
        });

        console.log(JSON.stringify(basicOutput, null, 2));

        // Output with metadata feature block
        const basicOutputWithMetadata = await client.buildBasicOutput({
            amount: '1000000',
            unlockConditions: [addressUnlockCondition],
            features: [
                // "Hello, World!" hex encoded
                new MetadataFeature('0x48656c6c6f2c20576f726c6421'),
            ],
        });

        console.log(JSON.stringify(basicOutputWithMetadata, null, 2));

        // Output with storage deposit return
        const basicOutputWithStorageReturn = await client.buildBasicOutput({
            amount: '1000000',
            unlockConditions: [
                addressUnlockCondition,
                new StorageDepositReturnUnlockCondition(
                    new Ed25519Address(hexAddress),
                    '1000000',
                ),
            ],
        });

        console.log(JSON.stringify(basicOutputWithStorageReturn, null, 2));

        // Output with expiration
        const basicOutputWithExpiration = await client.buildBasicOutput({
            amount: '1000000',
            unlockConditions: [
                addressUnlockCondition,
                new ExpirationUnlockCondition(
                    new Ed25519Address(hexAddress),
                    1,
                ),
            ],
        });

        console.log(JSON.stringify(basicOutputWithExpiration, null, 2));

        // Output with timelock
        const basicOutputWithTimelock = await client.buildBasicOutput({
            amount: '1000000',
            unlockConditions: [
                addressUnlockCondition,
                new TimelockUnlockCondition(1),
            ],
        });

        console.log(JSON.stringify(basicOutputWithTimelock, null, 2));

        // Output with tag feature
        const basicOutputWithTag = await client.buildBasicOutput({
            amount: '1000000',
            unlockConditions: [addressUnlockCondition],
            features: [new TagFeature('0x48656c6c6f2c20576f726c6421')],
        });

        console.log(JSON.stringify(basicOutputWithTag, null, 2));

        // Output with sender feature
        const basicOutputWithSender = await client.buildBasicOutput({
            amount: '1000000',
            unlockConditions: [addressUnlockCondition],
            features: [new SenderFeature(new Ed25519Address(hexAddress))],
        });

        console.log(JSON.stringify(basicOutputWithSender, null, 2));
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
