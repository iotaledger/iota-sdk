// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import 'reflect-metadata';

import { describe, it, expect } from '@jest/globals';
import {
    Wallet,
    CoinType,
    WalletOptions,
    SecretManager,
    Utils,
} from '../../lib/';

describe('Wallet', () => {

    it('create wallet', async () => {
        let storagePath = 'test-create-wallet';
        removeDir(storagePath);

        const walletOptions = await createDefaultWalletOptions(storagePath);
        const wallet = await Wallet.create(walletOptions);
        await wallet.destroy();

        removeDir(storagePath);
    }, 20000);

    it('restore wallet', async () => {
        let storagePath = 'test-restore-wallet';
        removeDir(storagePath);

        const walletOptions = await createDefaultWalletOptions(storagePath);
        const wallet = await Wallet.create(walletOptions);

        const client = await wallet.getClient();
        const hrp = await client.getBech32Hrp();
        expect(hrp).toEqual('smr');

        const originalAddress = await wallet.address();
        expect(originalAddress).toEqual(walletOptions.address);

        await wallet.destroy();

        const restoredWallet = await Wallet.create({ storagePath });
        expect(originalAddress).toEqual(await restoredWallet.address());

        const restoredClient = await restoredWallet.getClient();
        const restoredHrp = await restoredClient.getBech32Hrp();
        expect(restoredHrp).toEqual(hrp);

        // TODO: make add test to verify wallet equality
        // expect(accounts.length).toStrictEqual(0);
        await restoredWallet.destroy();

        removeDir(storagePath);
    }, 20000);

    it('error after destroy', async () => {
        let storagePath = 'test-error-after-destroy';
        removeDir(storagePath);

        const walletOptions = await createDefaultWalletOptions(storagePath);
        const wallet = await Wallet.create(walletOptions);
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

    it('error on address conflict', async () => {
        let storagePath = 'test-error-on-address-conflict';
        removeDir(storagePath);

        const walletOptions = await createDefaultWalletOptions(storagePath);
        const wallet = await Wallet.create(walletOptions);
        await wallet.destroy();

        walletOptions.address = "tst1qpqzgvcehafmlxh87zrf9w8ck8q2kw5070ztf68ylhzk89en9a4fyg6k2w3";

        try {
            const _restoredWallet = await Wallet.create(walletOptions);
            throw 'Should return an error because the provided address conflicts with the stored one';
        } catch (err: any) {
            expect(err.message).toEqual(
                'wallet address mismatch: tst1qpqzgvcehafmlxh87zrf9w8ck8q2kw5070ztf68ylhzk89en9a4fyg6k2w3, existing address is: smr1qpqzgvcehafmlxh87zrf9w8ck8q2kw5070ztf68ylhzk89en9a4fyq4ten7');
        }

        removeDir(storagePath);
    }, 20000);

    it('error on bip path conflict', async () => {
        let storagePath = 'test-error-on-bip-path-conflict';
        removeDir(storagePath);

        const walletOptions = await createDefaultWalletOptions(storagePath);
        const wallet = await Wallet.create(walletOptions);
        await wallet.destroy();

        walletOptions.bipPath = { coinType: CoinType.IOTA };

        try {
            const _restoredWallet = await Wallet.create(walletOptions);
            throw 'Should return an error because the provided bip path conflicts with the stored one';
        } catch (err: any) {
            expect(err.message).toEqual('bip path mismatch: Some(Bip44 { coin_type: 4218, account: 0, change: 0, address_index: 0 }), existing bip path is: Some(Bip44 { coin_type: 4219, account: 0, change: 0, address_index: 0 })');
        }

        removeDir(storagePath);
    }, 20000);

    it('error on alias conflict', async () => {
        let storagePath = 'test-error-on-alias-conflict';
        removeDir(storagePath);

        const walletOptions = await createDefaultWalletOptions(storagePath);
        const wallet = await Wallet.create(walletOptions);
        await wallet.destroy();

        walletOptions.alias = "Bob";

        try {
            const _restoredWallet = await Wallet.create(walletOptions);
            throw 'Should return an error because the provided alias conflicts with the stored one';
        } catch (err: any) {
            expect(err.message).toEqual('wallet alias mismatch: Bob, existing alias is: Alice');
        }

        removeDir(storagePath);
    }, 20000);

});

async function createDefaultWalletOptions(storagePath: string): Promise<WalletOptions> {
    const strongholdSecretManager = {
        stronghold: {
            snapshotPath: `./${storagePath}/wallet.stronghold`,
            password: `A12345678*`,
        },
    };
    const secretManager = SecretManager.create(strongholdSecretManager);
    await secretManager.storeMnemonic(
        'vital give early extra blind skin eight discover scissors there globe deal goat fat load robot return rate fragile recycle select live ordinary claim',
    );

    const coinType = CoinType.Shimmer;
    const address = (await secretManager.generateEd25519Addresses({
        coinType,
        accountIndex: 0,
        range: {
            start: 0,
            end: 1,
        },
        bech32Hrp: 'smr',
    }))[0];

    return {
        address,
        alias: "Alice",
        bipPath: {
            coinType,
        },
        storagePath,
        clientOptions: {
            nodes: ['https://api.testnet.shimmer.network'],
            protocolParameters: Utils.shimmerMainnetProtocolParameters(),
        },
        secretManager: strongholdSecretManager,
    };
}

function removeDir(storagePath: string) {
    const fs = require('fs');
    fs.rmSync(storagePath, { recursive: true, force: true });
}
