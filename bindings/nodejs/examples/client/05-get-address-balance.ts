// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, CommonOutput, SecretManager, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./client/05-get-address-balance.ts

// In this example we will get the outputs of an address that has no additional unlock
// conditions and sum the amounts and native tokens.
async function run() {
    initLogger();
    for (const envVar of ['NODE_URL', 'MNEMONIC']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }

    const client = await Client.create({
        // Insert your node URL in the .env.
        nodes: [process.env.NODE_URL as string],
    });

    try {
        const secretManager = SecretManager.create({
            mnemonic: process.env.MNEMONIC as string,
        });

        // Generate the first address
        const addresses = await secretManager.generateEd25519Addresses({
            accountIndex: 0,
            range: {
                start: 0,
                end: 1,
            },
        });

        // Get output ids of basic outputs that can be controlled by this address without further unlock constraints
        const outputIdsResponse = await client.basicOutputIds({
            address: addresses[0],
            hasExpiration: false,
            hasTimelock: false,
            hasStorageDepositReturn: false,
        });

        // Get outputs by their IDs
        const addressOutputs = await client.getOutputs(outputIdsResponse.items);

        // Calculate the total amount and native tokens
        let totalAmount = BigInt(0);
        const totalNativeTokens: { [id: string]: bigint } = {};
        for (const outputResponse of addressOutputs) {
            const output = outputResponse['output'];
            if (output instanceof CommonOutput) {
                const nativeTokenFeature = (
                    output as CommonOutput
                ).getNativeToken();
                if (nativeTokenFeature != undefined) {
                    totalNativeTokens[nativeTokenFeature.id] =
                        (totalNativeTokens[nativeTokenFeature.id] ||
                            BigInt(0)) + nativeTokenFeature.amount;
                }
            }

            totalAmount += output.getAmount();
        }

        console.log(
            `Outputs controlled by ${addresses[0]} have: ${totalAmount} glow and native tokens: `,
            totalNativeTokens,
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

void run().then(() => process.exit());
