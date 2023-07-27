// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AddressUnlockCondition, BasicOutput, Output, Utils } from '@iota/sdk';

import { getUnlockedWallet } from './common';

// The amount to build the basic output with
const AMOUNT = BigInt(1000000);

// In this example we check if an output has only an address unlock condition and that the address is from the account.
//
// Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
// running the `how_tos/accounts_and_addresses/create-account` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./wallet/17-check-unlock-conditions.ts
async function run() {
    try {
        // Create the wallet
        const wallet = await getUnlockedWallet();

        // Get the account we generated with `01-create-wallet`
        const account = await wallet.getAccount('Alice');

        const accountAddresses = await account.addresses();

        const output: Output = await account.prepareOutput({
            recipientAddress: accountAddresses[0].address,
            amount: AMOUNT,
        });

        const hexEncodedAccountAddresses = accountAddresses.map((a) =>
            Utils.bech32ToHex(a.address),
        );

        if (output instanceof BasicOutput) {
            const basicOutput = output as BasicOutput;
            let controlledByAccount = false;
            if (
                basicOutput.getUnlockConditions().length === 1 &&
                basicOutput.getUnlockConditions()[0] instanceof
                    AddressUnlockCondition &&
                hexEncodedAccountAddresses.includes(
                    (
                        basicOutput.getUnlockConditions()[0] as AddressUnlockCondition
                    )
                        .getAddress()
                        .toString(),
                )
            ) {
                controlledByAccount = true;
            }

            console.log(
                'The output has only an address unlock condition and the address is from the account: ' +
                    controlledByAccount,
            );
        }
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
