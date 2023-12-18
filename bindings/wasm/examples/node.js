// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const console = require('console');
const { Wallet, CoinType, initLogger, SecretManager } = require('../node/lib');

async function run() {
    // Config doesn't work yet but this is an example for the future
    await initLogger({
        name: 'stdout',
        levelFilter: 'debug',
        colorEnabled: true,
    });

    const mnemonicSecretManager = {
        mnemonic:
            'inhale gorilla deny three celery song category owner lottery rent author wealth penalty crawl hobby obtain glad warm early rain clutch slab august bleak',
    };

    const secretManager = SecretManager.create(mnemonicSecretManager);

    const walletAddress = await secretManager.generateEd25519Addresses({
        coinType: CoinType.IOTA,
        accountIndex: 0,
        range: {
            start: 0,
            end: 1,
        },
        bech32Hrp: 'tst',
    });

    const wallet = await Wallet.create({
        address: walletAddress[0],
        storagePath: './alice-database',
        bipPath: {
            coinType: CoinType.IOTA,
        },
        clientOptions: {
            nodes: ['https://api.testnet.shimmer.network'],
        },
        secretManager: mnemonicSecretManager,
    });

    console.log('wallet created');

    const balance = await wallet.sync();
    console.log(balance);
}

void run().then(() => process.exit());
