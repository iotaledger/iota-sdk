// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AccountOutput, Wallet, initLogger, utf8ToHex } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/account_output/state-transition.ts

const NEW_STATE_METADATA = 'updated state metadata 1';

// In this example we will update the state metadata of an account output.
async function run() {
    initLogger();
    if (!process.env.FAUCET_URL) {
        throw new Error('.env FAUCET_URL is undefined, see .env.example');
    }
    if (!process.env.WALLET_DB_PATH) {
        throw new Error('.env WALLET_DB_PATH is undefined, see .env.example');
    }
    if (!process.env.STRONGHOLD_PASSWORD) {
        throw new Error(
            '.env STRONGHOLD_PASSWORD is undefined, see .env.example',
        );
    }
    try {
        // Create the wallet
        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        // Get the account we generated with `01-create-wallet`
        const account = await wallet.getAccount('Alice');

        // May want to ensure the account is synced before sending a transaction.
        const balance = await account.sync();

        if (balance.accounts.length == 0) {
            throw new Error(`No Alias available in account 'Alice'`);
        }

        // We try to update the first account output in the account
        const accountId = balance.accounts[0];

        const accountOutputData = (
            await account.unspentOutputs({ accountIds: [accountId] })
        )[0];
        console.log(
            `Alias ${accountId} found in unspent output: ${accountOutputData.outputId}`,
        );

        const accountOutput = accountOutputData.output as AccountOutput;

        const updatedAccountOutput = await (
            await wallet.getClient()
        ).buildAccountOutput({
            accountId,
            unlockConditions: accountOutput.unlockConditions,
            stateIndex: accountOutput.stateIndex + 1,
            stateMetadata: utf8ToHex(NEW_STATE_METADATA),
            foundryCounter: accountOutput.foundryCounter,
            immutableFeatures: accountOutput.immutableFeatures,
            features: accountOutput.features,
        });

        await wallet.setStrongholdPassword(process.env.STRONGHOLD_PASSWORD);

        console.log('Sending transaction...');

        const transaction = await account.sendOutputs([updatedAccountOutput]);
        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get included
        const blockId = await account.reissueTransactionUntilIncluded(
            transaction.transactionId,
        );
        console.log(
            `Block included: ${process.env.EXPLORER_URL}/block/${blockId}`,
        );
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
