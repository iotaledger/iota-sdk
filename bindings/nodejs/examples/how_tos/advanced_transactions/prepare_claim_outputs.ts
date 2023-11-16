// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { OutputsToClaim, Wallet, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/advanced_transactions/prepare_claim_outputs.ts

// This example claims all claimable outputs in the account, in a two-step process.
async function run() {
    initLogger();
    try {
        if (!process.env.STRONGHOLD_PASSWORD) {
            throw new Error(
                '.env STRONGHOLD_PASSWORD is undefined, see .env.example',
            );
        }

        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        const account = await wallet.getAccount('Alice');

        await account.sync();

        // To sign a transaction we need to unlock stronghold.
        await wallet.setStrongholdPassword(process.env.STRONGHOLD_PASSWORD);

        const output_ids = await account.claimableOutputs(OutputsToClaim.All);
        console.log(`Available outputs to claim:`);
        for (const output_id of output_ids) {
            console.log(output_id);
        }

        const preparedTransaction = await account.prepareClaimOutputs(
            output_ids,
        );
        console.log(
            'Transaction prepared: ',
            preparedTransaction.preparedTransactionData(),
        );
        const transaction = await preparedTransaction.send();

        const blockId = await account.retryTransactionUntilIncluded(
            transaction.transactionId,
        );
        console.log(`Block sent: ${process.env.EXPLORER_URL}/block/${blockId}`);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
