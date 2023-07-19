/* eslint-disable @typescript-eslint/no-var-requires */
/**
 * In this example we sign and send an EVM transaction.
 */
const assert = require('assert/strict')
const { Buffer } = require('buffer')
const Web3 = require('web3')
const { Common } = require('@ethereumjs/common')
const { Transaction } = require('@ethereumjs/tx')
const { RLP } = require('@ethereumjs/rlp')
const { fromRpcSig } = require('@ethereumjs/util')

const { ERC_20_ABI } = require('./erc-20.abi')

const RPC_ENDPOINT = 'https://json-rpc.evm.testnet.shimmer.network'
const RECIPIENT_ACCOUNT_ADDRESS = '0x2fF33407c26E36c32cA50A4e63ce661b2eeED3dd'
const CHAIN_ID = 1072
const ETHEREUM_COIN_TYPE = 60

// fUSDC address. Tokens are available through: https://deepr.finance/faucet
const TOKEN_CONTRACT_ADDRESS = '0x01ee95C34AeCAE1948aB618e467A6806b25fe7e4'
const AMOUNT = 1e6 //fUSDC has 6 decimals

const TX_OPTIONS = { 
    common: Common.custom({
        chainId: CHAIN_ID,
    }),
    freeze: false
}

const getUnlockedManager = require('./account-manager');

async function run() {
    const provider = new Web3(RPC_ENDPOINT)
    try {
        let manager = await getUnlockedManager();
        const account = await manager.getAccount('Alice');

        const addresses = await account.generateEvmAddresses({
            coinType: ETHEREUM_COIN_TYPE,
            accountIndex: 0,
        })
        const senderAddress = addresses[0]

        // 1. Create unsigned transaction data
        const txData = await createTxData(provider, senderAddress)
        const transaction = Transaction.fromTxData(txData, TX_OPTIONS)

        // 2. Create messageToSign by external signer
        const message = transaction.getMessageToSign(false)
        const serializedMessage = Buffer.from(RLP.encode(message))
        const messageToSign = '0x' + serializedMessage.toString('hex')
        
        // 3. Sign message with external signer
        const bip44Path = {
            coinType: ETHEREUM_COIN_TYPE,
            account: 0,
            change: 0,
            addressIndex: 0,
        }
        const { signature } = await account.signSecp256k1Ecdsa(messageToSign, bip44Path)
        
        // 4. Make Secp256k1Ecdsa an Eip155Compatible Signature
        const ecdsaSignature = fromRpcSig(signature)
        ecdsaSignature.v = convertsVToEip155Compatible(ecdsaSignature.v, CHAIN_ID)

        // 5. Sign Transaction
        const signedTransaction = createSignedTransaction(transaction, ecdsaSignature)
        
        // Testing: check sender address matches
        assert.strictEqual(senderAddress, signedTransaction.getSenderAddress().toString(), 'Mismatch in addresses', )
        
        // 6. Send signed transaction
        const hexSignedTransaction = getHexEncodedTransaction(signedTransaction)
        const sentTransaction = await provider.eth.sendSignedTransaction(hexSignedTransaction)
        console.log('Sent Transaction: ', sentTransaction)

    } catch (error) {
        console.error('Error: ', error);
    }
    process.exit(0);
}

function createSignedTransaction(transaction, signature) {
    const rawTx = transaction.raw()

    const vHex = padHexString(signature.v.toString(16))
    rawTx[6] = Buffer.from(vHex, 'hex')
    rawTx[7] = signature.r
    rawTx[8] = signature.s
    const signedTransaction = Transaction.fromValuesArray(rawTx, TX_OPTIONS)

    return signedTransaction
}

function getHexEncodedTransaction(transaction) {
    const serializedTransaction = transaction.serialize()
    const hexEncodedTransaction = '0x' + serializedTransaction.toString('hex')
    return hexEncodedTransaction
}

function convertsVToEip155Compatible(v, chainId) {
    const parity = Number(v) % 27
    const newV = (chainId * 2) + (35 + parity)
    return newV
}

async function createTxData(provider, address) {
    const erc20Contract = new provider.eth.Contract(ERC_20_ABI, TOKEN_CONTRACT_ADDRESS)
    
    const data = erc20Contract.methods.transfer(RECIPIENT_ACCOUNT_ADDRESS, provider.utils.toHex(AMOUNT)).encodeABI()
    
    const nonce = provider.utils.toHex(await provider.eth.getTransactionCount(address))

    const gasPrice = provider.utils.toHex(await provider.eth.getGasPrice())

    const estimatedGas = await provider.eth.estimateGas({ from: address, to: TOKEN_CONTRACT_ADDRESS, data })
    const gasLimit = provider.utils.toHex(estimatedGas)

    const to = TOKEN_CONTRACT_ADDRESS
    const value = 0
    
    return { nonce, gasPrice, gasLimit, to, value, data }
}

function padHexString(str) {
    return str.length % 2 !== 0 ? "0" + str : str;
}

run();
