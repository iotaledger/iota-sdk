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
    utf8ToHex,
} from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/outputs/features.ts

// Build outputs with all features
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
            immutableFeatures: [
                new IssuerFeature(new Ed25519Address(hexAddress)),
            ],
        });

        // Output with metadata feature
        const nftOutputWithMetadata = await client.buildNftOutput({
            nftId: '0x0000000000000000000000000000000000000000000000000000000000000000',
            unlockConditions: [addressUnlockCondition],
            features: [new MetadataFeature(utf8ToHex('Hello, World!'))],
        });

        // Output with immutable metadata feature
        const nftOutputWithImmutableMetadata = await client.buildNftOutput({
            nftId: '0x0000000000000000000000000000000000000000000000000000000000000000',
            unlockConditions: [addressUnlockCondition],
            immutableFeatures: [
                new MetadataFeature(utf8ToHex('Hello, World!')),
            ],
        });

        // Output with tag feature
        const nftOutputWithTag = await client.buildNftOutput({
            nftId: '0x0000000000000000000000000000000000000000000000000000000000000000',
            unlockConditions: [addressUnlockCondition],
            features: [new TagFeature(utf8ToHex('Hello, World!'))],
        });

        console.log(
            JSON.stringify(
                [
                    nftOutputWithSender,
                    nftOutputWithIssuer,
                    nftOutputWithMetadata,
                    nftOutputWithImmutableMetadata,
                    nftOutputWithTag,
                ],
                null,
                2,
            ),
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
