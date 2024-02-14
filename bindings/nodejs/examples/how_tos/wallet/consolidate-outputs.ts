// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { CommonOutput, Utils, Wallet, initLogger } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/wallet/consolidate-outputs.ts

// In this example we will consolidate basic outputs from an wallet with only an AddressUnlockCondition by sending
// them to the same address again.
async function run() {
    initLogger();
    try {
        for (const envVar of [
            'WALLET_DB_PATH',
            'STRONGHOLD_PASSWORD',
            'EXPLORER_URL',
        ])
            if (!(envVar in process.env)) {
                throw new Error(
                    `.env ${envVar} is undefined, see .env.example`,
                );
            }

        const wallet = await Wallet.create({
            storagePath: process.env.WALLET_DB_PATH,
        });

        // To create an address we need to unlock stronghold.
        await wallet.setStrongholdPassword(
            process.env.STRONGHOLD_PASSWORD as string,
        );

        // Sync wallet to make sure wallet is updated with outputs from previous examples
        await wallet.sync();
        console.log('Wallet synced');

        // List unspent outputs before consolidation.
        // The output we created with example `request_funds` and the basic output from `mint` have only one
        // unlock condition and it is an `AddressUnlockCondition`, and so they are valid for consolidation. They have the
        // same `AddressUnlockCondition`(the address of the wallet), so they will be consolidated into one
        // output.
        const outputs = await wallet.unspentOutputs();
        console.log('Outputs BEFORE consolidation:');

        outputs.forEach(({ output, address }, i) => {
            console.log(`OUTPUT #${i}`);
            console.log(
                '- address: %s\n- amount: %d\n- native token: %s',
                Utils.hexToBech32(address.toString(), 'rms'),
                output.getAmount(),
                output instanceof CommonOutput
                    ? (output as CommonOutput).getNativeToken() ?? []
                    : [],
            );
        });

        console.log('Sending consolidation transaction...');

        // Consolidate unspent outputs and print the consolidation transaction ID
        // Set `force` to true to force the consolidation even though the `output_threshold` isn't reached
        const transaction = await wallet.consolidateOutputs({
            force: true,
        });
        console.log('Transaction sent: %s', transaction.transactionId);

        // Wait for the consolidation transaction to get accepted
        const blockId = wallet.waitForTransactionAcceptance(
            transaction.transactionId,
        );

        console.log(
            'Transaction accepted: %s/block/$s',
            process.env.EXPLORER_URL,
            blockId,
        );

        // Sync wallet
        await wallet.sync();
        console.log('Wallet synced');

        // Outputs after consolidation
        console.log('Outputs AFTER consolidation:');
        outputs.forEach(({ output, address }, i) => {
            console.log(`OUTPUT #${i}`);
            console.log(
                '- address: %s\n- amount: %d\n- native tokens: %s',
                Utils.hexToBech32(address.toString(), 'rms'),
                output.getAmount(),
                output instanceof CommonOutput
                    ? (output as CommonOutput).getNativeToken()
                    : undefined,
            );
        });
    } catch (error) {
        console.error('Error: ', error);
    }
}

void run().then(() => process.exit());