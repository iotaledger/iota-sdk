/**
 * In this example we check if an output has only an address unlock condition and that the address is from the account.
 */
const { strict } = require('assert')
const { Buffer } = require('buffer')
const Web3 = require('web3')
const { Chain, Common } = require('@ethereumjs/common')
const { Transaction, TxData } = require('@ethereumjs/tx')
const { RLP } = require('@ethereumjs/rlp')
const { fromRpcSig } = require('@ethereumjs/util')

const { ERC_20_ABI } = require('./erc-20.abi')

const RPC_ENDPOINT = 'https://json-rpc.evm.testnet.shimmer.network/v1/chains/rms1prwgvvw472spqusqeufvlmp8xdpyxtrnmvt26jnuk6sxdcq2hk8scku26h7/evm'
const RECIPIENT_ACCOUNT_ADDRESS = '0xcBCd6D8659Ed1998A452335AE53904dc0Af1c99b'
const TOKEN_CONTRACT_ADDRESS = '0x1074010000000000000000000000000000000000'
const CHAIN_ID = 1071 // ShimmerEVM Chain ID
const COIN_TYPE = 60
const ETH_AMOUNT = 1000000 // Since we don't want to transfer ETH

const TX_OPTIONS = { common: Common.custom({
    chainId: CHAIN_ID,
})}

const getUnlockedManager = require('./account-manager');

async function run() {
    const provider = new Web3(RPC_ENDPOINT)
    try {
        let manager = await getUnlockedManager();
        const account = await manager.getAccount('Alice');

        // Why pass in accountIndex here?
        // What is the default range? And why is it not 1
        // How are the ranges specified, inclusive / exclusive
        const addresses = await account.generateEvmAddresses({
            coinType: COIN_TYPE,
            accountIndex: 0,
        })
        const senderAddress = addresses[0]
        const txData = await createTxData(provider, senderAddress)
        console.log('TxData', txData)


        const transactionObject = Transaction.fromTxData(txData, TX_OPTIONS)
        const message = transactionObject.getMessageToSign(false)
        const serializedMessage = Buffer.from(RLP.encode(message)).toString('hex')     
        
        // What is this? And why should we be doing this in typescript?
        // As a library this should expose secure defaults => Always hardened
        const HARDEN_MASK = (1 << 31) >>> 0;

        // This needs documentation + be moved to rust?
        // Should be consistent with generateEvmAddresses()
        // Why do we need to pass all this in?
        // Coin type, address index and internal flag should be enough from our side
        const bip32Chain = [
            (44 | HARDEN_MASK) >>> 0,
            (COIN_TYPE | HARDEN_MASK) >>> 0,
            (0 | HARDEN_MASK) >>> 0,
            0,
            0,
        ];

        // Example shows signEvm on secret manager but it is on the account?
        const { publicKey, signature } = await account.signEvm(`0x${serializedMessage}`, bip32Chain)
        console.log('signature', signature)
        console.log('publicKey', publicKey)
        const txSignature = fromRpcSig(signature)

        // We need EIP155 compatibility for Shimmer EVM
        console.log('Parsed signature', txSignature.v)
        txSignature.v = convertsVtoEip155Compatible(txSignature.v, CHAIN_ID)
        console.log('Parsed signature 2', txSignature.v)
        txData.v = txSignature.v
        txData.r = txSignature.r
        txData.s = txSignature.s

        const signedTransaction = Transaction.fromTxData(txData, TX_OPTIONS)
        // Unable to get the address to match the BIP path specified
        const actualSender = signedTransaction.getSenderAddress().toString('hex')
        strict.strictEqual(actualSender, senderAddress)
        
        const serializedTransaction = signedTransaction.serialize()
        console.log('serializedTransaction: ', serializedTransaction)
        const signedTransactionData = '0x' + serializedTransaction.toString('hex')
        console.log('signedTransactionData:', signedTransactionData)
        const tx = await provider.eth.sendSignedTransaction(signedTransactionData)
        console.log('tx', tx)
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

function convertsVtoEip155Compatible(v, chainId) {
    const parity = Number(v) - 27
    const newV = parity + chainId * 2 + 35
    return newV
}

async function createTxData(provider, address) {
    // Disregard this for now
    const erc20Contract = new provider.eth.Contract(ERC_20_ABI, TOKEN_CONTRACT_ADDRESS)
    const data = erc20Contract.methods.transfer(RECIPIENT_ACCOUNT_ADDRESS, provider.utils.toHex(ETH_AMOUNT)).encodeABI()
    
    const nonce = provider.utils.toHex(await provider.eth.getTransactionCount(address))

    const _gasPrice = await provider.eth.getGasPrice()
    const gasPrice = '0x' + _gasPrice
    
    // const estimatedGas = await provider.eth.estimateGas({ from: address, to: TOKEN_CONTRACT_ADDRESS, data })
    const gasLimit = provider.utils.toHex(2000000) // Double to ensure we have enough gas

    const to = RECIPIENT_ACCOUNT_ADDRESS
    const value = provider.utils.toHex(ETH_AMOUNT)

    return { nonce, gasPrice, gasLimit, to, value, data, chainId: BigInt(CHAIN_ID), r: 0, s: 0 }
}

run();
