// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import 'reflect-metadata';

import { describe, it, expect } from '@jest/globals';
import { Wallet, CoinType, WalletOptions } from '../../lib/';

describe('Wallet', () => {
    it('create account', async () => {
        let storagePath = 'test-create-account';
        removeDir(storagePath);

        const walletOptions = {
            storagePath: './test-create-account',
            clientOptions: {
                nodes: ['https://api.testnet.shimmer.network'],
            },
            coinType: CoinType.Shimmer,
            secretManager: {
                stronghold: {
                    snapshotPath: `./${storagePath}/wallet.stronghold`,
                    password: `A12345678*`,
                },
            },
        };

        const wallet = new Wallet(walletOptions);
        await wallet.storeMnemonic(
            'acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast',
        );

        const account = await wallet.createAccount({
            alias: 'Alice',
        });

        expect(account.getMetadata().index).toStrictEqual(0);

        const secretManager = await wallet.getSecretManager();
        const seed = await secretManager.getSeed();
        expect(seed).toStrictEqual("0x65d378f26a101366d2b2bc982de128382f260205a8b99266fdcee14cc12f4680eb66171c27be01066c3ea30c9c0b87e27fb90f8cab9ac7b8e205f259d275240f");

        await wallet.destroy()
        removeDir(storagePath)
    }, 8000);

    it('generate address', async () => {
        let storagePath = 'test-generate-address';
        removeDir(storagePath);

        const walletOptions: WalletOptions = {
            storagePath,
            clientOptions: {
                nodes: ['https://api.testnet.shimmer.network'],
            },
            coinType: CoinType.Shimmer,
            secretManager: {
                stronghold: {
                    snapshotPath: `./${storagePath}/wallet.stronghold`,
                    password: `A12345678*`,
                },
            },
        };

        const wallet = new Wallet(walletOptions);
        await wallet.storeMnemonic(
            'vital give early extra blind skin eight discover scissors there globe deal goat fat load robot return rate fragile recycle select live ordinary claim',
        );

        const address = await wallet.generateEd25519Address(
            0,
            0,
            { internal: false, ledgerNanoPrompt: false },
            'rms',
        );

        expect(address).toStrictEqual(
            'rms1qpqzgvcehafmlxh87zrf9w8ck8q2kw5070ztf68ylhzk89en9a4fy5jqrg8',
        );

        const anotherAddress = await wallet.generateEd25519Address(
            10,
            10,
            { internal: true, ledgerNanoPrompt: false },
            'tst',
        );

        expect(anotherAddress).toStrictEqual(
            'tst1qzp37j45rkfmqn05fapq66vyw0vkmz5zqhmeuey5fked0wt4ry43jeqp2wv',
        );

        await wallet.destroy()
        removeDir(storagePath)
    }, 8000);

    it('recreate wallet', async () => {
        let storagePath = 'test-recreate-wallet';
        removeDir(storagePath);

        const walletOptions = {
            storagePath,
            clientOptions: {
                nodes: ['https://api.testnet.shimmer.network'],
            },
            coinType: CoinType.Shimmer,
            secretManager: {
                stronghold: {
                    snapshotPath: `./${storagePath}/wallet.stronghold`,
                    password: `A12345678*`,
                },
            },
        };

        const wallet = new Wallet(walletOptions);
        await wallet.storeMnemonic(
            'vital give early extra blind skin eight discover scissors there globe deal goat fat load robot return rate fragile recycle select live ordinary claim',
        );

        const account = await wallet.createAccount({
            alias: 'Alice',
        });

        expect(account.getMetadata().index).toStrictEqual(0);

        const client = await wallet.getClient();

        const localPoW = await client.getLocalPow();
        expect(localPoW).toBeTruthy();

        await wallet.destroy();

        const recreatedWallet = new Wallet(walletOptions);
        const accounts = await recreatedWallet.getAccounts();
        expect(accounts.length).toStrictEqual(1);

        await recreatedWallet.destroy()
        removeDir(storagePath)
    }, 20000);
})

function removeDir(storagePath: string) {
    const fs = require('fs');
    fs.rmSync(storagePath, { recursive: true, force: true });
}
