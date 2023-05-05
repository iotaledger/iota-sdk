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
} from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// node ./dist/client/15_build_nft_output.js

// Build an nft output
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

        // IOTA NFT Standard - IRC27: https://github.com/iotaledger/tips/blob/main/tips/TIP-0027/tip-0027.md
        const tip27ImmutableMetadata = {
            standard: 'IRC27',
            version: 'v1.0',
            type: 'image/jpeg',
            uri: 'https://mywebsite.com/my-nft-files-1.jpeg',
            name: 'My NFT #0001',
        };

        const nftOutput = await client.buildNftOutput({
            // NftId needs to be null the first time
            nftId: '0x0000000000000000000000000000000000000000000000000000000000000000',
            unlockConditions: [
                new AddressUnlockCondition(new Ed25519Address(hexAddress)),
            ],
            immutableFeatures: [
                new IssuerFeature(new Ed25519Address(hexAddress)),
                new MetadataFeature(
                    utf8ToHex(JSON.stringify(tip27ImmutableMetadata)),
                ),
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
