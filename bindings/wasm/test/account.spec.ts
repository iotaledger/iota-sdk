// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import fs from 'fs';
import { Account, AccountBalance, Wallet, CoinType } from '../node/lib';

async function run() {
    try {
        fs.rmdirSync('./test-alice-database', { recursive: true });
    } catch (e) {
        // ignore it
    }

    const wallet = new Wallet({
        storagePath: './test-alice-database',
        coinType: CoinType.Shimmer,
        clientOptions: {
            nodes: ['http://localhost:14265'],
        },
        secretManager: {
            mnemonic:
                'inhale gorilla deny three celery song category owner lottery rent author wealth penalty crawl hobby obtain glad warm early rain clutch slab august bleak',
        },
    });

    const account: Account = await wallet.createAccount({
        alias: 'Alice',
    });

    expect(account.getMetadata().alias).toBe('Alice');

    const balance: AccountBalance = await account.sync();
    expect(balance.baseCoin.available).not.toBeNaN();

    await account.setAlias('new alias');
    const savedAccount: Account = await wallet.getAccount('new alias');
    expect(savedAccount).not.toBeNull();
}

// Tests requiring a local node
describe('local node tests', () => {
    jest.setTimeout(10000);
    it('account', async () => {
        await run();
    });
});
