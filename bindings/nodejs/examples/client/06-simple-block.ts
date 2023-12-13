// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    CoinType,
    Client,
    initLogger,
    TaggedDataPayload,
    utf8ToHex,
    SecretManager,
} from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./client/06-simple-block.ts

// In this example we will send a block without a payload.
async function run() {
    initLogger();
    for (const envVar of ['NODE_URL', 'EXPLORER_URL']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }

    if (!process.env.MNEMONIC) {
        throw new Error('.env MNEMONIC is undefined, see .env.example');
    }
    const mnemonicSecretManager = {
        mnemonic: process.env.MNEMONIC,
    };

    const secretManager = SecretManager.create(mnemonicSecretManager);

    const client = await Client.create({
        // Insert your node URL in the .env.
        nodes: [process.env.NODE_URL as string],
    });

    const issuerId = process.env.ISSUER_ID
        ? process.env.ISSUER_ID
        : '0x0000000000000000000000000000000000000000000000000000000000000000';

    const chain = {
        coinType: CoinType.IOTA,
        account: 0,
        change: 0,
        addressIndex: 0,
    };

    try {
        // Create block with no payload
        // TODO: have a way in the bindings to send an empty block https://github.com/iotaledger/iota-sdk/issues/647
        const unsignedBlock = await client.buildBasicBlock(
            issuerId,
            new TaggedDataPayload(utf8ToHex('Hello'), utf8ToHex('Tangle')),
        );
        const block = await secretManager.signBlock(unsignedBlock, chain);
        const blockId = await client.postBlock(block);
        console.log('Block:', block, '\n');

        console.log(
            `Empty block sent: ${process.env.EXPLORER_URL}/block/${blockId}`,
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

void run().then(() => process.exit());
