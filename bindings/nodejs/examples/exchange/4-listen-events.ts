// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// This example listens to the NewOutput event.
// Run with command:
// yarn run-example ./exchange/4-listen-events.ts

import { Wallet, Event, WalletEventType } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

async function run() {
    try {
        for (const envVar of ['WALLET_DB_PATH']) {
            if (!(envVar in process.env)) {
                throw new Error(
                    `.env ${envVar} is undefined, see .env.example`,
                );
            }
        }

        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        const callback = function (err: any, event: Event) {
            console.log('AccountIndex:', event.accountIndex);
            console.log('Event:', event.event);

            // Exit after receiving an event.
            process.exit(0);
        };

        // Only interested in new outputs here.
        await wallet.listen([WalletEventType.NewOutput], callback);

        const account = await wallet.getAccount('Alice');

        // Use the faucet to send testnet tokens to your address.
        console.log(
            'Fill your address with the faucet: https://faucet.testnet.shimmer.network/',
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
        console.error(error);
    }
}

run().then(() => process.exit());
