// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Client,
    initLogger,
    utf8ToHex,
    Utils,
    AddressUnlockCondition,
    TagFeature,
    MetadataFeature,
    SenderFeature,
    Ed25519Address,
    IssuerFeature,
    Irc27Metadata,
} from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./client/15-build-nft-output.ts

// In this example we will build an NFT output.
async function run() {
    initLogger();
    for (const envVar of ['NODE_URL']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }

    const client = new Client({
        // Insert your node URL in the .env.
        nodes: [process.env.NODE_URL as string],
    });

    try {
        const hexAddress = Utils.bech32ToHex(
            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
        );

        const tip27ImmutableMetadata = new Irc27Metadata(
            'image/jpeg',
            'https://mywebsite.com/my-nft-files-1.jpeg',
            'My NFT #0001',
        );

        const nftOutput = await client.buildNftOutput({
            // NftId needs to be null the first time
            nftId: '0x0000000000000000000000000000000000000000000000000000000000000000',
            unlockConditions: [
                new AddressUnlockCondition(new Ed25519Address(hexAddress)),
            ],
            immutableFeatures: [
                new IssuerFeature(new Ed25519Address(hexAddress)),
                tip27ImmutableMetadata.asFeature(),
            ],
            features: [
                new SenderFeature(new Ed25519Address(hexAddress)),
                new MetadataFeature(utf8ToHex('mutable metadata')),
                new TagFeature(utf8ToHex('my tag')),
            ],
        });

        console.log(JSON.stringify(nftOutput, null, 2));
        process.exit();
    } catch (error) {
        console.error('Error: ', error);
    }
}

run();
