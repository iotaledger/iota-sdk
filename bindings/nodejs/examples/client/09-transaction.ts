// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, SecretManager, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./client/09-transaction.ts

// In this example we will send a transaction
async function run() {
    initLogger();
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }

    const client = new Client({
        // Insert your node URL in the .env.
        nodes: [process.env.NODE_URL],
        localPow: true,
    });

    try {
        if (!process.env.MNEMONIC) {
            throw new Error('.env MNEMONIC is undefined, see .env.example');
        }

        // Configure your own mnemonic in ".env". Since the output amount cannot be zero, the mnemonic must contain non-zero
        // balance
        const secretManager = {
            mnemonic: process.env.MNEMONIC,
        };

        // We generate an address from our own mnemonic so that we send the funds to ourselves
        const addresses = await new SecretManager(
            secretManager,
        ).generateEd25519Addresses({
            range: {
                start: 1,
                end: 2,
            },
        });

        // We prepare the transaction
        // Insert the output address and amount to spend. The amount cannot be zero.
        const blockIdAndBlock = await client.buildAndPostBlock(secretManager, {
            output: {
                address: addresses[0],
                amount: BigInt(1000000),
            },
        });

        console.log(
            `Block sent: ${process.env.EXPLORER_URL}/block/${blockIdAndBlock[0]}`,
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
