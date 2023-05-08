// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const console = require('console');
const fs = require('fs');
const { Wallet, CoinType, initLogger } = require('../node/lib');

async function run() {
    try {
        fs.rmdirSync('./alice-database', { recursive: true });
    } catch (e) {
        // ignore it
    }

    // Config doesn't work yet but this is an example for the future
    await initLogger({
        name: 'stdout',
        levelFilter: 'debug',
        colorEnabled: true,
    });

    const wallet = new Wallet({
        storagePath: './alice-database',
        coinType: CoinType.Shimmer,
        clientOptions: {
            nodes: ['https://api.testnet.shimmer.network'],
        },
        secretManager: {
            mnemonic:
                'inhale gorilla deny three celery song category owner lottery rent author wealth penalty crawl hobby obtain glad warm early rain clutch slab august bleak',
        },
    });

    const account = await wallet.createAccount({
        alias: 'Alice',
        bech32Hrp: 'rms',
    });

    console.log('Account created:', account);
    account.setAlias('new alias');

    const balance = await account.sync();
    console.log(balance);
}

run();
