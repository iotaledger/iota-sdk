// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { initLogger } from '@iota/sdk';
import { getUnlockedManager } from './accounts';

// In this example we request funds from the faucet to the first address in the account.
//
// Make sure that `example.stronghold` and `example.walletdb` already exist by
// running the `01-create-wallet` example!
//
// Rename `.env.example` to `.env` first, then run the command:
// yarn run-example ./wallet/03-get-funds.ts
async function run() {
    initLogger();
    try {
        // Create the wallet
        const manager = await getUnlockedManager();

        const account = await manager.getAccount(
            `${process.env.ACCOUNT_ALIAS_1}`,
        );
        let balance = await account.sync();
        console.log('Account synced');

        const addresses = await account.addresses();

        let fundsBefore = balance.baseCoin.available;
        console.log('Current available funds:', fundsBefore);

        console.log('Requesting funds from faucet...');
        const faucetResponse = await account.requestFundsFromFaucet(
            process.env.FAUCET_URL!,
            addresses[0].address,
        );

        console.log('Response from faucet: ', faucetResponse);

        console.log('Waiting for funds (timeout=60s)...');
        // Check for changes to the balance
        const start = new Date();

        // TODO: Make this number
        let fundsAfter;
        while (true) {
            if (
                Math.floor((new Date().getTime() - start.getTime()) / 1000) > 60
            ) {
                throw new Error('took too long waiting for funds');
            }
            fundsAfter = (await account.sync()).baseCoin.available;
            if (fundsAfter !== fundsBefore) {
                break;
            } else {
                await new Promise<void>((resolve) => setTimeout(resolve, 2000));
            }
        }

        console.log('New available funds: ', fundsAfter);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
