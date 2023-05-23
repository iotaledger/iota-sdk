// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AddressUnlockCondition, BasicOutput, Output, Utils } from '@iota/sdk';

import { getUnlockedManager } from './account-manager';

// The amount to build the basic output with
const AMOUNT = '1000000';

// In this example we check if an output has only an address unlock condition and that the address is from the account.
//
// Make sure that `example.stronghold` and `example.walletdb` already exist by
// running the `01-create-wallet` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./wallet/17-check-unlock-conditions.ts
async function run() {
    try {
        // Create the wallet
        const manager = await getUnlockedManager();

        // Get the account we generated with `01-create-wallet`
        const account = await manager.getAccount(
            `${process.env.ACCOUNT_ALIAS_1}`,
        );

        let accountAddresses = await account.addresses();
        // console.log("Account addresses: ", accountAddresses);

        let output: Output = await account.prepareOutput({
            recipientAddress: accountAddresses[0].address,
            amount: AMOUNT,
        });

        let hexEncodedAccountAddresses = accountAddresses.map((a) =>
            Utils.bech32ToHex(a.address),
        );

        if (output instanceof BasicOutput) {
            let basicOutput = output as BasicOutput;
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
