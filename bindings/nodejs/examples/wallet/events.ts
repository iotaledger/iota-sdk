// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Event,
    ConsolidationRequiredWalletEvent,
    TransactionProgressWalletEvent,
    SelectingInputsProgress,
} from '@iota/sdk';
import { getUnlockedWallet } from './common';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example wallet/events.ts

// This example listens to wallet events.
async function run() {
    for (const envVar of ['NODE_URL', 'STRONGHOLD_PASSWORD'])
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }

    try {
        // Create the wallet
        const wallet = await getUnlockedWallet();

        const callback = function (err: any, event: Event) {
            console.log(
                'AccountIndex:',
                event.accountIndex,
                ', Event:',
                event.event,
            );
        };

        await wallet.listen([], callback);

        await wallet.emitTestEvent(new ConsolidationRequiredWalletEvent());
        await wallet.emitTestEvent(
            new TransactionProgressWalletEvent(new SelectingInputsProgress()),
        );

        await wallet.destroy();
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
