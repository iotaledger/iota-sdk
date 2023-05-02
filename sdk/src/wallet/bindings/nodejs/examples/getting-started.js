// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { AccountManager, CoinType } = require('@iota/wallet');

// A name to associate with the created account.
const ACCOUNT_ALIAS = 'Alice';

// The node to connect to.
const NODE_URL = 'https://api.testnet.shimmer.network';

// A password to encrypt the stored data.
// WARNING: Never hardcode passwords in production code.
const STRONGHOLD_PASSWORD = 'a-secure-password';

// The path to store the account snapshot.
const STRONGHOLD_SNAPSHOT_PATH = 'vault.stronghold';

async function main() {
    // Set up and store the wallet.
    const accountManagerOptions = {
        clientOptions: {
            nodes: [NODE_URL],
            localPow: true,
        },
        coinType: CoinType.Shimmer,
        secretManager: {
            stronghold: {
                snapshotPath: STRONGHOLD_SNAPSHOT_PATH,
                password: STRONGHOLD_PASSWORD,
            },
        },
    };

    const manager = new AccountManager(accountManagerOptions);

    // Generate a mnemonic and store it in the Stronghold vault.
    // INFO: It is best practice to back up the mnemonic somewhere secure.
    const mnemonic = await manager.generateMnemonic();
    await manager.storeMnemonic(mnemonic);

    // Create an account.
    const account = await manager.createAccount({
        alias: ACCOUNT_ALIAS,
    });

    // Get the first address and print it.
    const address = await account.addresses().then(addresses => addresses[0]);
    console.log(`Address:\n${address.address}\n`);

    process.exit(0);
}

main();
