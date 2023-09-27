// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { describe, it, beforeAll, afterAll } from '@jest/globals';
import 'reflect-metadata';
import 'dotenv/config';

import {
    Client,
    Payload,
    PreparedTransactionData,
    RegularTransactionEssence,
    SecretManager,
    SHIMMER_TESTNET_BECH32_HRP,
    TransactionPayload,
    Utils
} from '../../';
import '../customMatchers';
import { addresses } from '../fixtures/offlineSigningAddresses';
import * as signedTransactionJson from '../fixtures/signedTransaction.json';
import * as sigUnlockPreparedTx from '../fixtures/sigUnlockPreparedTx.json';

describe('Offline signing examples', () => {
    let onlineClient: Client;
    let offlineClient: Client;

    const secretManager = {
        mnemonic:
            'endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river',
    };

    beforeAll(() => {
        onlineClient = new Client({
            nodes: [
                {
                    url: process.env.NODE_URL || 'http://localhost:14265',
                },
            ],
            localPow: true,
        });
        offlineClient = new Client({})
    });

    afterAll(async () => {
        await onlineClient.destroy();
        await offlineClient.destroy();
    });

    it('generates addresses offline', async () => {
        const addresses = await new SecretManager(secretManager).generateEd25519Addresses({
            range: {
                start: 0,
                end: 1,
            },
            bech32Hrp: SHIMMER_TESTNET_BECH32_HRP,
        });

        expect(addresses.length).toBe(1);
        addresses.forEach((address) => {
            expect(address).toBeValidAddress();
        });
    });

    // transaction tests disabled for workflows, because they fail if we don't have funds
    it.skip('prepares and signs a transaction', async () => {
        const address =
            'rms1qqv5avetndkxzgr3jtrswdtz5ze6mag20s0jdqvzk4fwezve8q9vkpnqlqe';
        const amount = BigInt(1000000);

        const inputs = await onlineClient.findInputs(addresses, amount);

        const preparedTransaction = await onlineClient.prepareTransaction(
            undefined,
            {
                inputs,
                output: { address, amount: amount },
            },
        );

        expect(preparedTransaction.essence).toBeInstanceOf(RegularTransactionEssence);

        const signedTransaction = await offlineClient.signTransaction(
            secretManager,
            // Imported JSON is typed with literal types
            preparedTransaction,
        );

        expect(signedTransaction).toBeInstanceOf(TransactionPayload);
    });

    // transaction tests disabled for workflows, because they fail if we don't have funds
    it.skip('sends a transaction', async () => {
        // Send block with the signed transaction as a payload
        const blockIdAndBlock = await onlineClient.postBlockPayload(
            // Imported JSON is typed with literal types
            signedTransactionJson as unknown as Payload,
        );

        expect(blockIdAndBlock[1].payload).toBeDefined();

        const blockId = Utils.blockId(blockIdAndBlock[1]);

        expect(blockId).toBe(blockIdAndBlock[0]);
        expect(blockId).toBeValidBlockId;
    });
    it('create a signature unlock', async () => {
        // Verifies that an unlock created in Rust matches that created by the binding when the mnemonic is identical.
        const secretManager = {
            mnemonic:
                'good reason pipe keen price glory mystery illegal loud isolate wolf trash raise guilt inflict guide modify bachelor length galaxy lottery there mango comfort',
        };
        const preparedTx = sigUnlockPreparedTx as any as PreparedTransactionData;
        const txEssenceHash = Utils.hashTransactionEssence(
            preparedTx.essence,
        );

        if (preparedTx.inputsData[0].chain === undefined) {
            throw 'chain is undefined';
        }

        const unlock = await new SecretManager(secretManager).signatureUnlock(
            txEssenceHash,
            preparedTx.inputsData[0].chain,
        );

        expect(unlock).toStrictEqual({
            type: 0,
            signature: {
                type: 0,
                publicKey:
                    '0xb76a23de43b8132ae18a4a479cb158563e76d89bd1e20d3ccdc7fd1db2a009d4',
                signature:
                    '0xcd905dae45010980e95ddddaebede830d9b8d7489c67e4d91a0cbfbdb03b02d337dc8162f15582ad18ee0e953cd517e32f809d533f9ccfb4beee5cb2cba16d0c',
            },
        });
    });
});
