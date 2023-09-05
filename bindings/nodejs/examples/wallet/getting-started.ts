// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, CoinType, WalletOptions, Utils } from '@iota/sdk';

// Run with command:
// yarn run-example ./wallet/getting-started.ts

// The database path.
const WALLET_DB_PATH = 'getting-started-db';

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
    const walletOptions: WalletOptions = {
        storagePath: WALLET_DB_PATH,
        clientOptions: {
            nodes: [NODE_URL],
        },
        coinType: CoinType.Shimmer,
        secretManager: {
            stronghold: {
                snapshotPath: STRONGHOLD_SNAPSHOT_PATH,
                password: STRONGHOLD_PASSWORD,
            },
        },
    };

    const wallet = new Wallet(walletOptions);

    // Generate a mnemonic and store its seed in the Stronghold vault.
    // INFO: It is best practice to back up the mnemonic somewhere secure.
    const mnemonic = Utils.generateMnemonic();
    console.log('Mnemonic:' + mnemonic);
    await wallet.storeMnemonic(mnemonic);

    // Create an account.
    const account = await wallet.createAccount({
        alias: ACCOUNT_ALIAS,
    });

    // Get the first address and print it.
    const address = (await account.addresses())[0];
    console.log(`Address: ${address.address}\n`);

    process.exit(0);
}

main();
