// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    AliasOutput,
    StateControllerAddressUnlockCondition,
    UnlockConditionType,
    Utils,
    Wallet,
    initLogger,
} from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/alias/governance-transition.ts

// In this example we will update the state controller of an alias output.
async function run() {
    initLogger();
    for (const envVar of [
        'WALLET_DB_PATH',
        'STRONGHOLD_PASSWORD',
        'EXPLORER_URL',
    ]) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
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

        if (balance.aliases.length == 0) {
            throw new Error(`No Alias available in account 'Alice'`);
        }

        // We try to update the first alias in the account
        const aliasId = balance.aliases[0];

        const aliasOutputData = (
            await account.unspentOutputs({ aliasIds: [aliasId] })
        )[0];
        console.log(
            `Alias ${aliasId} found in unspent output: ${aliasOutputData.outputId}`,
        );

        await wallet.setStrongholdPassword(
            process.env.STRONGHOLD_PASSWORD as string,
        );

        const newStateController = Utils.parseBech32Address(
            (await account.generateEd25519Addresses(1))[0].address,
        );

        const aliasOutput = aliasOutputData.output as AliasOutput;
        const updatedUnlockConditions = aliasOutput.unlockConditions.map(
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

        const updatedAliasOutput = await (
            await wallet.getClient()
        ).buildAliasOutput({
            aliasId,
            amount: aliasOutput.amount,
            unlockConditions: updatedUnlockConditions,
            stateIndex: aliasOutput.stateIndex,
            stateMetadata: aliasOutput.stateMetadata,
            foundryCounter: aliasOutput.foundryCounter,
            immutableFeatures: aliasOutput.immutableFeatures,
            features: aliasOutput.features,
        });

        console.log('Sending transaction...');

        const transaction = await account.sendOutputs([updatedAliasOutput]);
        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get included
        const blockId = await account.retryTransactionUntilIncluded(
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
