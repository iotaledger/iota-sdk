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
    Ed25519Address,
    IssuerFeature,
} from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/outputs/output_features.ts

// Build ouputs with all features
async function run() {
    initLogger();

    const client = new Client({});

    try {
        const hexAddress = Utils.bech32ToHex(
            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
        );

        const addressUnlockCondition: UnlockCondition =
            new AddressUnlockCondition(new Ed25519Address(hexAddress));

        // Output with sender feature
        const nftOutputWithSender = await client.buildNftOutput({
            nftId: '0x0000000000000000000000000000000000000000000000000000000000000000',
            unlockConditions: [addressUnlockCondition],
            features: [new SenderFeature(new Ed25519Address(hexAddress))],
        });

        // Output with issuer feature
        const nftOutputWithIssuer = await client.buildNftOutput({
            nftId: '0x0000000000000000000000000000000000000000000000000000000000000000',
            unlockConditions: [addressUnlockCondition],
            immutableFeatures: [new IssuerFeature(new Ed25519Address(hexAddress))],
        });

        // Output with metadata feature
        const nftOutputWithMetadata = await client.buildNftOutput({
            nftId: '0x0000000000000000000000000000000000000000000000000000000000000000',
            unlockConditions: [addressUnlockCondition],
            features: [
                // "Hello, World!" hex encoded
                new MetadataFeature('0x48656c6c6f2c20576f726c6421'),
            ],
        });

        // Output with immutable metadata feature
        const nftOutputWithImmutableMetadata = await client.buildNftOutput({
            nftId: '0x0000000000000000000000000000000000000000000000000000000000000000',
            unlockConditions: [addressUnlockCondition],
            immutableFeatures: [
                // "Hello, World!" hex encoded
                new MetadataFeature('0x48656c6c6f2c20576f726c6421'),
            ],
        });

        // Output with tag feature
        const nftOutputWithTag = await client.buildNftOutput({
            nftId: '0x0000000000000000000000000000000000000000000000000000000000000000',
            unlockConditions: [addressUnlockCondition],
            // "Hello, World!" hex encoded
            features: [new TagFeature('0x48656c6c6f2c20576f726c6421')],
        });

        console.log(JSON.stringify(
            [nftOutputWithSender, nftOutputWithIssuer, nftOutputWithMetadata, nftOutputWithImmutableMetadata, nftOutputWithTag], null, 2));
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
