// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import 'reflect-metadata';

import { describe, it, expect } from '@jest/globals';
import { Wallet, CoinType, WalletOptions, SecretManager } from '../../lib/';

const chain = {
    coinType: CoinType.IOTA,
    account: 0,
    change: 0,
    addressIndex: 0,
};

describe('Wallet', () => {
    it('create account', async () => {
        let storagePath = 'test-create-account';
        removeDir(storagePath);

        const strongholdSecretManager = {
            stronghold: {
                snapshotPath: `./${storagePath}/wallet.stronghold`,
                password: `A12345678*`,
            },
        }
        const secretManager = await new SecretManager(strongholdSecretManager);
        await secretManager.storeMnemonic(
            'vital give early extra blind skin eight discover scissors there globe deal goat fat load robot return rate fragile recycle select live ordinary claim',
        );

        const walletOptions = {
            storagePath: './test-create-account',
            clientOptions: {
                nodes: ['https://api.testnet.shimmer.network'],
            },
            bipPath: chain,
            secretManager: strongholdSecretManager,
        };

        const wallet = new Wallet(walletOptions);

        await wallet.destroy();
        removeDir(storagePath);
    }, 20000);

    it('generate address', async () => {
        let storagePath = 'test-generate-address';
        removeDir(storagePath);

        const strongholdSecretManager = {
            stronghold: {
                snapshotPath: `./${storagePath}/wallet.stronghold`,
                password: `A12345678*`,
            },
        }
        const secretManager = await new SecretManager(strongholdSecretManager);
        await secretManager.storeMnemonic(
            'vital give early extra blind skin eight discover scissors there globe deal goat fat load robot return rate fragile recycle select live ordinary claim',
        );

        const walletOptions: WalletOptions = {
            storagePath,
            clientOptions: {
                nodes: ['https://api.testnet.shimmer.network'],
            },
            bipPath: chain,
            secretManager: strongholdSecretManager,
        };

        const wallet = new Wallet(walletOptions);

        const address = await secretManager.generateEd25519Addresses({
            coinType: CoinType.Shimmer,
            accountIndex: 0,
            range: { start: 0, end: 1},
            options: { internal: false, ledgerNanoPrompt: false },
            bech32Hrp: 'rms',
        });

        expect(address[0]).toStrictEqual(
            'rms1qpqzgvcehafmlxh87zrf9w8ck8q2kw5070ztf68ylhzk89en9a4fy5jqrg8',
        );

        const anotherAddress = await secretManager.generateEd25519Addresses({
            coinType: CoinType.Shimmer,
            accountIndex: 10,
            range: { start: 0, end: 10 },
            options: { internal: true, ledgerNanoPrompt: false },
            bech32Hrp: 'tst',
        });

        expect(anotherAddress[0]).toStrictEqual(
            'tst1qr5ckcctawkng8mc9hmkumnf0c2rrurd56wht6wxjcywtetzpqsvyhph79x',
        );

        await wallet.destroy();
        removeDir(storagePath);
    }, 20000);

    it('recreate wallet', async () => {
        let storagePath = 'test-recreate-wallet';
        removeDir(storagePath);

        const strongholdSecretManager = {
            stronghold: {
                snapshotPath: `./${storagePath}/wallet.stronghold`,
                password: `A12345678*`,
            },
        }
        const secretManager = await new SecretManager(strongholdSecretManager);
        await secretManager.storeMnemonic(
            'vital give early extra blind skin eight discover scissors there globe deal goat fat load robot return rate fragile recycle select live ordinary claim',
        );

        const walletOptions = {
            storagePath,
            clientOptions: {
                nodes: ['https://api.testnet.shimmer.network'],
            },
            bipPath: chain,
            secretManager: strongholdSecretManager,
        };

        const wallet = new Wallet(walletOptions);

        await wallet.destroy();

        const recreatedWallet = new Wallet(walletOptions);
        const accounts = await recreatedWallet.accounts();

        // TODO: make acocunts test 
        expect(accounts.length).toStrictEqual(0);

        await recreatedWallet.destroy();
        removeDir(storagePath)
    }, 20000);

    it('error after destroy', async () => {
        let storagePath = 'test-error-after-destroy';
        removeDir(storagePath);

        const strongholdSecretManager = {
            stronghold: {
                snapshotPath: `./${storagePath}/wallet.stronghold`,
                password: `A12345678*`,
            },
        }
        const secretManager = await new SecretManager(strongholdSecretManager);
        await secretManager.storeMnemonic(
            'vital give early extra blind skin eight discover scissors there globe deal goat fat load robot return rate fragile recycle select live ordinary claim',
        );

        const walletOptions = {
            storagePath,
            clientOptions: {
                nodes: ['https://api.testnet.shimmer.network'],
            },
            bipPath: chain,
            secretManager: strongholdSecretManager,
        };

        const wallet = new Wallet(walletOptions);

        await wallet.destroy();

        try {
            const _accounts = await wallet.accounts();
            throw 'Should return an error because the wallet was destroyed';
        } catch (err: any) {
            expect(err.message).toEqual('Wallet was destroyed');
        }

        try {
            const _client = await wallet.getClient();
            throw 'Should return an error because the wallet was destroyed';
        } catch (err: any) {
            expect(err.message).toEqual('Wallet was destroyed');
        }
        removeDir(storagePath);
    }, 35000);
})

function removeDir(storagePath: string) {
    const fs = require('fs');
    fs.rmSync(storagePath, { recursive: true, force: true });
}
