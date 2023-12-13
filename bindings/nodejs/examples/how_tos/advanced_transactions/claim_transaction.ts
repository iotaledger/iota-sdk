// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { OutputsToClaim, Wallet, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/advanced_transactions/claim_transaction.ts

// This example claims all claimable outputs in the wallet.
async function run() {
    initLogger();
    try {
        for (const envVar of [
            'WALLET_DB_PATH',
            'STRONGHOLD_PASSWORD',
            'EXPLORER_URL',
        ]) {
            if (!(envVar in process.env)) {
                throw new Error(
                    `.env ${envVar} is undefined, see .env.example`,
                );
            }
        }

        const wallet = await Wallet.create({
            storagePath: process.env.WALLET_DB_PATH,
        });

        await wallet.sync();

        // To sign a transaction we need to unlock stronghold.
        await wallet.setStrongholdPassword(
            process.env.STRONGHOLD_PASSWORD as string,
        );

        // Get all claimable outputs
        const output_ids = await wallet.claimableOutputs(OutputsToClaim.All);
        console.log(`Available outputs to claim:`);
        for (const output_id of output_ids) {
            console.log(output_id);
        }

        const transaction = await wallet.claimOutputs(output_ids);
        console.log(`Transaction sent: ${transaction.transactionId}`);

        const blockId = await wallet.reissueTransactionUntilIncluded(
            transaction.transactionId,
        );
        console.log(`Block sent: ${process.env.EXPLORER_URL}/block/${blockId}`);
    } catch (error) {
        console.error('Error: ', error);
    }
}

void run().then(() => process.exit());
