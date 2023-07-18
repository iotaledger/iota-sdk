/* eslint-disable @typescript-eslint/no-var-requires */
/**
 * In this example we check if an output has only an address unlock condition and that the address is from the account.
 */
const { Buffer } = require('buffer')
const Web3 = require('web3')
const { Common } = require('@ethereumjs/common')
const { Transaction } = require('@ethereumjs/tx')
const { RLP } = require('@ethereumjs/rlp')
const { fromRpcSig, bufArrToArr } = require('@ethereumjs/util')

const { ERC_20_ABI } = require('./erc-20.abi')

const RPC_ENDPOINT = 'https://rpc.sepolia.org'
// const RPC_ENDPOINT = 'https://json-rpc.evm.testnet.shimmer.network'
const RECIPIENT_ACCOUNT_ADDRESS = '0xcBCd6D8659Ed1998A452335AE53904dc0Af1c99b'
const TOKEN_CONTRACT_ADDRESS = '0x68194a729C2450ad26072b3D33ADaCbcef39D574'
const CHAIN_ID = 11155111
const COIN_TYPE = 60
const AMOUNT = 1000000 // Since we don't want to transfer ETH

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
            coinType: COIN_TYPE,
            accountIndex: 0,
        })
        const senderAddress = addresses[0]

        console.log('Expect to send from:', senderAddress)

        // 1. Create unsigned transaction data
        const txData = await createTxData(provider, senderAddress)
        const transaction = Transaction.fromTxData(txData, TX_OPTIONS)

        // 2. Replace v value of raw transaction
        const rawTx = transaction.raw()


        // 3. RLP encode message for Stronghold        
        const ethTx = Transaction.fromValuesArray(rawTx)
        const rlpEncodedMessage = Buffer.from(RLP.encode(bufArrToArr(ethTx.getMessageToSign(false))))
        const messageToSign = '0x' + rlpEncodedMessage.toString('hex')

        // 4. Generate hardened bip44 path
        const bip44Path = {
            coinType: 60,
            account: 0,
            change: 0,
            addressIndex: 0,
        }

        // 5. Sign the message using stronghold
        const { publicKey, signature } = await account.signSecp256k1Ecdsa(messageToSign, bip44Path)
        console.log('Public key from signSecp256k1:', publicKey.slice(2))
        console.log('Signature from signSecp256k1:', signature)
        // Extracts v,r & s values from the signature
        const txSignature = fromRpcSig(signature)
        // 6. Replace with Eip155 compatible signature
        txSignature.v = convertsVtoEip155Compatible(txSignature.v, CHAIN_ID)
        console.log('txSignature', txSignature)

        // 7. Recreate signed transaction
        const { signedTransaction, fromAddress } = createSignedTransaction(rawTx, txSignature)
        console.log('Address recovered from Ethereum Transaction', fromAddress)
        // Unable to get the address to match the BIP path specified
        console.warn('Are addresses equal:', fromAddress === senderAddress)   
        
        // 8. Broadcast transaction
        // const tx = await provider.eth.sendSignedTransaction(signedTransaction)
        // console.log('tx', tx)
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
    return { signedTransaction: `0x${serializedTx.toString('hex')}`, fromAddress }
}

function convertsVtoEip155Compatible(v, chainId) {
    const parity = Number(v) % 27
    const newV = parity + chainId * 2 + 35
    return newV
}

async function createTxData(provider, address) {
    // Disregard this for now
    const erc20Contract = new provider.eth.Contract(ERC_20_ABI, TOKEN_CONTRACT_ADDRESS)
    
    const data = '' //erc20Contract.methods.transfer(RECIPIENT_ACCOUNT_ADDRESS, provider.utils.toHex(AMOUNT)).encodeABI()
    
    const nonce = provider.utils.toHex(9)

    const _gasPrice = await provider.eth.getGasPrice()
    console.log('Gas Price:', _gasPrice)
    const gasPrice = '0x4000'
    // const estimatedGas = await provider.eth.estimateGas({ from: address, to: TOKEN_CONTRACT_ADDRESS, data })
    const gasLimit = provider.utils.toHex(6000000) // Double to ensure we have enough gas

    const to = TOKEN_CONTRACT_ADDRESS
    const value = provider.utils.toHex(0)
    
    const v = 2 * CHAIN_ID + 35

    return { nonce, gasPrice, gasLimit, to, value, data, v, r: 0, s: 0 }
}

function padHexString(str) {
    return str.length % 2 !== 0 ? "0" + str : str;
}

run();
