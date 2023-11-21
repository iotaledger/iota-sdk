/* eslint-disable @typescript-eslint/no-var-requires */

// Run with command:
// yarn run-example ./evm/send-evm-transaction.ts

// In this example we will send an ERC-20 token transfer using a SecretManager.
import { strictEqual } from 'assert/strict';
import { Buffer } from 'buffer';
import Web3 from 'web3';
import { Common } from '@ethereumjs/common';
import { Transaction, TxData } from '@ethereumjs/tx';
import { RLP } from '@ethereumjs/rlp';
import { fromRpcSig } from '@ethereumjs/util';

import { SecretManager } from '@iota/sdk';

import { ERC_20_ABI } from './erc-20.abi';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

const RPC_ENDPOINT = 'https://json-rpc.evm.testnet.shimmer.network';
const RECIPIENT_ACCOUNT_ADDRESS = '0x2fF33407c26E36c32cA50A4e63ce661b2eeED3dd';
const CHAIN_ID = 1072;
const ETHEREUM_COIN_TYPE = 60;

// fUSDC address. Tokens are available through: https://deepr.finance/faucet
const TOKEN_CONTRACT_ADDRESS = '0x01ee95C34AeCAE1948aB618e467A6806b25fe7e4';
const AMOUNT = 1e6; //fUSDC has 6 decimals

const TX_OPTIONS = {
    common: Common.custom({
        chainId: CHAIN_ID,
    }),
    freeze: false,
};

async function run(): Promise<void> {
    const provider = new Web3(RPC_ENDPOINT);
    for (const envVar of ['MNEMONIC']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }
    try {
        const mnemonicSecretManager = {
            mnemonic: process.env.MNEMONIC as string,
        };

        const secretManager = new SecretManager(mnemonicSecretManager);

        const addresses = await secretManager.generateEvmAddresses({
            coinType: ETHEREUM_COIN_TYPE,
            accountIndex: 0,
        });
        const senderAddress = addresses[0];

        // 1. Create unsigned transaction data
        const txData = await createTxData(provider, senderAddress);
        const transaction = Transaction.fromTxData(txData, TX_OPTIONS);

        // 2. Create messageToSign by external signer
        const message = transaction.getMessageToSign(false);
        const serializedMessage = Buffer.from(RLP.encode(message));
        const messageToSign = '0x' + serializedMessage.toString('hex');

        // 3. Sign message with external signer
        const bip44Path = {
            coinType: ETHEREUM_COIN_TYPE,
            account: 0,
            change: 0,
            addressIndex: 0,
        };
        const { signature } = await secretManager.signSecp256k1Ecdsa(
            messageToSign,
            bip44Path,
        );

        // 4. Make Secp256k1Ecdsa an Eip155Compatible Signature
        const ecdsaSignature = fromRpcSig(signature);
        ecdsaSignature.v = convertsVToEip155Compatible(
            ecdsaSignature.v,
            CHAIN_ID,
        );

        // 5. Sign Transaction
        const signedTransaction = createSignedTransaction(
            transaction,
            ecdsaSignature,
        );

        // Testing: check sender address matches
        strictEqual(
            senderAddress,
            signedTransaction.getSenderAddress().toString(),
            'Mismatch in addresses',
        );

        // 6. Send signed transaction
        const hexSignedTransaction =
            getHexEncodedTransaction(signedTransaction);
        const sentTransaction = await provider.eth.sendSignedTransaction(
            hexSignedTransaction,
        );
        console.log('sent Transaction', sentTransaction);
    } catch (error) {
        console.error('Error: ', error);
    }
    process.exit(0);
}

function createSignedTransaction(
    transaction: Transaction,
    signature: any,
): Transaction {
    const rawTx = transaction.raw();

    const vHex = padHexString(signature.v.toString(16));
    rawTx[6] = Buffer.from(vHex, 'hex');
    rawTx[7] = signature.r;
    rawTx[8] = signature.s;
    const signedTransaction = Transaction.fromValuesArray(rawTx, TX_OPTIONS);

    return signedTransaction;
}

function getHexEncodedTransaction(transaction: Transaction): string {
    const serializedTransaction = transaction.serialize();
    const hexEncodedTransaction = '0x' + serializedTransaction.toString('hex');
    return hexEncodedTransaction;
}

function convertsVToEip155Compatible(v: bigint, chainId: number): bigint {
    const parity = Number(v) % 27;
    const newV = chainId * 2 + (35 + parity);
    return BigInt(newV);
}

async function createTxData(provider: any, address: string): Promise<TxData> {
    const erc20Contract = new provider.eth.Contract(
        ERC_20_ABI,
        TOKEN_CONTRACT_ADDRESS,
    );

    const data = erc20Contract.methods
        .transfer(RECIPIENT_ACCOUNT_ADDRESS, provider.utils.toHex(AMOUNT))
        .encodeABI();

    const nonce = provider.utils.toHex(
        await provider.eth.getTransactionCount(address),
    );

    const _gasPrice = await provider.eth.getGasPrice();
    const gasPrice = provider.utils.toHex(_gasPrice);

    const estimatedGas = await provider.eth.estimateGas({
        from: address,
        to: TOKEN_CONTRACT_ADDRESS,
        data,
    });
    const gasLimit = provider.utils.toHex(estimatedGas);

    const to = RECIPIENT_ACCOUNT_ADDRESS;
    const value = provider.utils.toHex(AMOUNT);

    return { nonce, gasPrice, gasLimit, to, value, data };
}

function padHexString(str: string): string {
    return str.length % 2 !== 0 ? '0' + str : str;
}

run();
