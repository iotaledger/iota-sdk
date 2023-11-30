// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Wallet,
    CoinType,
    WalletOptions,
    Utils,
    SecretManager,
} from '@iota/sdk';

// Run with command:
// yarn run-example ./wallet/getting-started.ts

// The database path.
const WALLET_DB_PATH = 'getting-started-db';

// The node to connect to.
const NODE_URL = 'https://api.testnet.shimmer.network';

// A password to encrypt the stored data.
// WARNING: Never hardcode passwords in production code.
const STRONGHOLD_PASSWORD = 'a-secure-password';

// The path to store the wallet snapshot.
const STRONGHOLD_SNAPSHOT_PATH = 'vault.stronghold';

async function run() {
    const strongholdSecretManager = {
        stronghold: {
            snapshotPath: STRONGHOLD_SNAPSHOT_PATH,
            password: STRONGHOLD_PASSWORD,
        },
    };

    const secretManager = SecretManager.create(strongholdSecretManager);

    // Generate a mnemonic and store its seed in the Stronghold vault.
    // INFO: It is best practice to back up the mnemonic somewhere secure.
    const mnemonic = Utils.generateMnemonic();
    console.log('Mnemonic:' + mnemonic);

    // Store the mnemonic in the Stronghold snapshot, this needs to be done only the first time.
    // The mnemonic can't be retrieved from the Stronghold file, so make a backup in a secure place!
    await secretManager.storeMnemonic(mnemonic);

    const wallet_address = await secretManager.generateEd25519Addresses({
        coinType: CoinType.IOTA,
        accountIndex: 0,
        range: {
            start: 0,
            end: 1,
        },
        bech32Hrp: 'tst',
    });

    const walletOptions: WalletOptions = {
        address: wallet_address[0],
        storagePath: WALLET_DB_PATH,
        clientOptions: {
            nodes: [NODE_URL as string],
        },
        bipPath: {
            coinType: CoinType.IOTA,
        },
        secretManager: strongholdSecretManager,
    };

    const wallet = await Wallet.create(walletOptions);

    console.log('Generated wallet with address: ' + (await wallet.address()));

    process.exit(0);
}

void run().then(() => process.exit());
