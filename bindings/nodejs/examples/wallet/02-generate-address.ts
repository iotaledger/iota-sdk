// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AccountAddress, Address } from '@iota/sdk';
import { getUnlockedManager } from './accounts';

// The number of addresses to generate
const NUM_ADDRESSES_TO_GENERATE = 2;

// In this example we will generate an address for an already existing wallet.
//
// Make sure that `example.stronghold` and `example.walletdb` already exist by
// running the `01-create-wallet` example!
//
// Rename `.env.example` to `.env` first, then run the command:
// yarn run-example ./wallet/02-generate.address.ts
async function run() {
    try {
        // Create the wallet
        const manager = await getUnlockedManager();

        // Get the account we generated with `01-create-wallet`
        const account = await manager.getAccount(
            `${process.env.ACCOUNT_ALIAS_1}`,
        );

        let prepended = process.env.EXPLORER_URL + '/addr/';
        console.log('Current addresses:');
        for (let address of await account.addresses()) {
            console.log(' - ' + prepended + address.address);
        }

        // Generate some addresses
        let addresses: AccountAddress[] = await account.generateAddresses(
            NUM_ADDRESSES_TO_GENERATE,
        );

        console.log('Generated ' + addresses.length + ' new addresses:');
        const accountAddresses: AccountAddress[] = await account.addresses();

        for (let address of addresses) {
            // TODO: Make includes work on the AccountAddress object
            if (
                accountAddresses.map((a) => a.address).includes(address.address)
            ) {
                console.log(' - ' + prepended + address.address);
            } else {
                throw new Error('this should never happen');
            }
        }
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
