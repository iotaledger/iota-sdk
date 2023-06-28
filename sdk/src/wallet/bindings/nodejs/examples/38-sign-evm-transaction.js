/* eslint-disable @typescript-eslint/no-var-requires */
/**
 * In this example we check if an output has only an address unlock condition and that the address is from the account.
 */
const { Buffer } = require('buffer')
const Web3 = require('web3')
const { Common } = require('@ethereumjs/common')
const { Transaction } = require('@ethereumjs/tx')
const { RLP } = require('@ethereumjs/rlp')
const { fromRpcSig } = require('@ethereumjs/util')

const { ERC_20_ABI } = require('./erc-20.abi')

const RPC_ENDPOINT = 'https://rpc.sepolia.org/'
const RECIPIENT_ACCOUNT_ADDRESS = '0xcBCd6D8659Ed1998A452335AE53904dc0Af1c99b'
const TOKEN_CONTRACT_ADDRESS = '0x68194a729C2450ad26072b3D33ADaCbcef39D574'
const CHAIN_ID = 11155111
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

        const addresses = await account.generateEvmAddresses({
            coinType: COIN_TYPE,
            accountIndex: 0,
        })
        const senderAddress = addresses[0]

        // 1. Create unsigned transaction data
        const txData = await createTxData(provider, senderAddress)
        const transaction = Transaction.fromTxData(txData, TX_OPTIONS)
        
        // 2. Replace v value of raw transaction
        const rawTx = transaction.raw()
        const chainId = TX_OPTIONS.common.chainId().toString(16)
        const vHex = padHexString(chainId)
        rawTx[6] = Buffer.from(vHex, 'hex')

        // 3. RLP encode message for Stronghold
        const message = '0x' + Buffer.from(RLP.encode(rawTx)).toString('hex')
        console.log('message', message)

        // 4. Generate hardened bip32 chain
        const HARDEN_MASK = (1 << 31) >>> 0;
        const bip32Chain = [
            (44 | HARDEN_MASK) >>> 0,
            (COIN_TYPE | HARDEN_MASK) >>> 0,
            (0 | HARDEN_MASK) >>> 0,
            0,
            0,
        ];

        // 5. Sign the message using stronghold
        const { signature } = await account.signSecp256k1Ecdsa(message, bip32Chain)
        // Extracts v,r & s values from the signature
        const txSignature = fromRpcSig(signature)

        // 6. Replace with Eip155 compatible signature
        txSignature.v = convertsVtoEip155Compatible(txSignature.v, CHAIN_ID)
        
        // 7. Recreate signed transaction
        const { signedTransaction, fromAddress } = createSignedTransaction(rawTx, txSignature)
        
        // Unable to get the address to match the BIP path specified
        console.warn('Are addresses equal:', fromAddress === senderAddress)   
        
        // 8. Broadcast transaction
        const tx = await provider.eth.sendSignedTransaction(signedTransaction)
        console.log('tx', tx)
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

function createSignedTransaction(rawTx, signature) {
    const vHex = padHexString(signature.v.toString(16))
    rawTx[6] = Buffer.from(vHex, 'hex')
    rawTx[7] = signature.r
    rawTx[8] = signature.s

    const transaction = Transaction.fromValuesArray(rawTx, TX_OPTIONS)
    const fromAddress = transaction.getSenderAddress().toString('hex')
    
    const serializedTx = transaction.serialize()
    return { signedTransaction: `0x${serializedTx.toString('hex')}`, sender: fromAddress }
}

function convertsVtoEip155Compatible(v, chainId) {
    const parity = Number(v) % 27
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

    const to = TOKEN_CONTRACT_ADDRESS
    const value = provider.utils.toHex(ETH_AMOUNT)

    return { nonce, gasPrice, gasLimit, to, value, data }
}

function padHexString(str) {
    return str.length % 2 !== 0 ? "0" + str : str;
}

run();
