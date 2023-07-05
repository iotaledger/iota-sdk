// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, Event, WalletEventType } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./exchange/4-listen-events.ts

// This example listen to the NewOutput event.
async function run() {
    try {
        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        const callback = function (err: any, event: Event) {
            console.log('AccountIndex:', event.getAccountIndex());
            console.log('Event:', event.getEvent());

            // Exit after receiving an event.
            process.exit(0);
        };

        // Only interested in new outputs here.
        await wallet.listen([WalletEventType.NewOutput], callback);

        const account = await wallet.getAccount('Alice');

        // Use the faucet to send testnet tokens to your address.
        console.log(
            'Fill your address with the Faucet: https://faucet.testnet.shimmer.network/',
        );

        const addresses = await account.addresses();
        console.log('Send funds to:', addresses[0].address);

        // Sync every 5 seconds until the faucet transaction gets confirmed.
        for (let i = 0; i < 100; i++) {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            // Sync to detect new outputs
            // Set syncOnlyMostBasicOutputs to true if not interested in outputs that are timelocked,
            // have a storage deposit return, expiration or are nft/alias/foundry outputs.
            await account.sync({ syncOnlyMostBasicOutputs: true });
        }
    } catch (error) {
        console.log('Error: ', error);
    }
}

run().then(() => process.exit());
