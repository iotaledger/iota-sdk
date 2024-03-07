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
    ExpirationUnlockCondition,
    TimelockUnlockCondition,
    utf8ToHex,
} from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./client/11-build-output.ts

// In this example we will build a basic output with various options.
async function run() {
    initLogger();
    for (const envVar of ['NODE_URL']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }

    const client = await Client.create({
        // Insert your node URL in the .env.
        nodes: [process.env.NODE_URL as string],
    });

    try {
        const ed25519Address = Utils.parseBech32Address(
            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
        );

        const addressUnlockCondition: UnlockCondition =
            new AddressUnlockCondition(ed25519Address);

        // Build most basic output with amount and a single address unlock condition
        const basicOutput = await client.buildBasicOutput({
            amount: BigInt(1000000),
            unlockConditions: [addressUnlockCondition],
        });

        console.log(JSON.stringify(basicOutput, null, 2));

        // Output with metadata feature block
        const basicOutputWithMetadata = await client.buildBasicOutput({
            amount: BigInt(1000000),
            unlockConditions: [addressUnlockCondition],
            features: [
                new MetadataFeature({ data: utf8ToHex('Hello World!') }),
            ],
        });

        console.log(JSON.stringify(basicOutputWithMetadata, null, 2));

        // Output with storage deposit return
        const basicOutputWithStorageReturn = await client.buildBasicOutput({
            amount: BigInt(1000000),
            unlockConditions: [
                addressUnlockCondition,
                new StorageDepositReturnUnlockCondition(
                    ed25519Address,
                    '1000000',
                ),
            ],
        });

        console.log(JSON.stringify(basicOutputWithStorageReturn, null, 2));

        // Output with expiration
        const basicOutputWithExpiration = await client.buildBasicOutput({
            amount: BigInt(1000000),
            unlockConditions: [
                addressUnlockCondition,
                new ExpirationUnlockCondition(ed25519Address, 1),
            ],
        });

        console.log(JSON.stringify(basicOutputWithExpiration, null, 2));

        // Output with timelock
        const basicOutputWithTimelock = await client.buildBasicOutput({
            amount: BigInt(1000000),
            unlockConditions: [
                addressUnlockCondition,
                new TimelockUnlockCondition(1),
            ],
        });

        console.log(JSON.stringify(basicOutputWithTimelock, null, 2));

        // Output with tag feature
        const basicOutputWithTag = await client.buildBasicOutput({
            amount: BigInt(1000000),
            unlockConditions: [addressUnlockCondition],
            features: [new TagFeature(utf8ToHex('Hello, World!'))],
        });

        console.log(JSON.stringify(basicOutputWithTag, null, 2));

        // Output with sender feature
        const basicOutputWithSender = await client.buildBasicOutput({
            amount: BigInt(1000000),
            unlockConditions: [addressUnlockCondition],
            features: [new SenderFeature(ed25519Address)],
        });

        console.log(JSON.stringify(basicOutputWithSender, null, 2));
    } catch (error) {
        console.error('Error: ', error);
    }
}

void run().then(() => process.exit());
