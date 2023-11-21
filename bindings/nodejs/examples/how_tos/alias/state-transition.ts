// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AliasOutput, Wallet, initLogger, utf8ToHex } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/alias/state-transition.ts

const NEW_STATE_METADATA = 'updated state metadata 1';

// In this example we will update the state metadata of an alias output.
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

        const aliasOutput = aliasOutputData.output as AliasOutput;

        const updatedAliasOutput = await (
            await wallet.getClient()
        ).buildAliasOutput({
            aliasId,
            unlockConditions: aliasOutput.unlockConditions,
            stateIndex: aliasOutput.stateIndex + 1,
            stateMetadata: utf8ToHex(NEW_STATE_METADATA),
            foundryCounter: aliasOutput.foundryCounter,
            immutableFeatures: aliasOutput.immutableFeatures,
            features: aliasOutput.features,
        });

        await wallet.setStrongholdPassword(
            process.env.STRONGHOLD_PASSWORD as string,
        );

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
