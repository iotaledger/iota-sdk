// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    AccountOutput,
    StateControllerAddressUnlockCondition,
    UnlockConditionType,
    Utils,
    Wallet,
    initLogger,
} from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/account_output/governance-transition.ts

// In this example we will update the state controller of an account output.
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
            throw new Error(`No account output available in account 'Alice'`);
        }

        // We try to update the first account output in the account
        const accountId = balance.accounts[0];

        const accountOutputData = (
            await account.unspentOutputs({ accountIds: [accountId] })
        )[0];
        console.log(
            `Account ${accountId} found in unspent output: ${accountOutputData.outputId}`,
        );

        await wallet.setStrongholdPassword(process.env.STRONGHOLD_PASSWORD);

        const newStateController = Utils.parseBech32Address(
            (await account.generateEd25519Addresses(1))[0].address,
        );

        const accountOutput = accountOutputData.output as AccountOutput;
        const updatedUnlockConditions = accountOutput.unlockConditions.map(
            (unlock) => {
                if (unlock.type == UnlockConditionType.StateControllerAddress) {
                    return new StateControllerAddressUnlockCondition(
                        newStateController,
                    );
                } else {
                    return unlock;
                }
            },
        );

        const updatedAccountOutput = await (
            await wallet.getClient()
        ).buildAccountOutput({
            accountId,
            amount: accountOutput.amount,
            unlockConditions: updatedUnlockConditions,
            stateIndex: accountOutput.stateIndex,
            stateMetadata: accountOutput.stateMetadata,
            foundryCounter: accountOutput.foundryCounter,
            immutableFeatures: accountOutput.immutableFeatures,
            features: accountOutput.features,
        });

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
