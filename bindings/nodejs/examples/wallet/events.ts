// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Event,
    ConsolidationRequiredWalletEvent,
    TransactionProgressWalletEvent,
    SelectingInputsProgress,
} from '@iota/sdk';
import { getUnlockedManager } from './account-manager';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example wallet/events.ts

// This example listens to wallet events.
async function run() {
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }
    if (!process.env.STRONGHOLD_PASSWORD) {
        throw new Error(
            '.env STRONGHOLD_PASSWORD is undefined, see .env.example',
        );
    }
    try {
        // Create the wallet
        const wallet = await getUnlockedManager();

        const callback = function (err: any, event: Event) {
            console.log(
                'AccountIndex:',
                event.getAccountIndex(),
                ', Event:',
                event.getEvent(),
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
