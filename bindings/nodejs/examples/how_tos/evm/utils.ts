import { SimpleBufferCursor } from './simple-buffer-cursor';

function hexToBytes(hex: any) {
    for (var bytes = [], c = 0; c < hex.length; c += 2)
        bytes.push(parseInt(hex.substr(c, 2), 16));
    return Buffer.from(bytes);
}

export async function prepareMetadata(evmAddress: string, amount: bigint, gas: bigint) {
    const metadata = new SimpleBufferCursor();

    /* Write contract meta data */
    metadata.writeUInt8(0); // nil sender contract
    metadata.writeUInt32LE(0x3c4b5e02); // "accounts"
    metadata.writeUInt32LE(0x23f4e3a1); // "transferAllowanceTo"
    metadata.writeUInt64SpecialEncoding(gas); // gas

    /* Create evm address buffer */
    const evmAddressBuffer = new SimpleBufferCursor();
    evmAddressBuffer.writeInt8(3); // EVM address type (3)
    evmAddressBuffer.writeBytes(hexToBytes(evmAddress.toLowerCase())) // EVM address

    /* Write length of contract arguments (1) */
    metadata.writeUInt32SpecialEncoding(1);

    // Write evm address (arg1)
    metadata.writeUInt32SpecialEncoding(1);// Length of key (len(a) == 1)
    metadata.writeInt8('a'.charCodeAt(0)); // Write key (a == 'agentID')
    metadata.writeUInt32SpecialEncoding(evmAddressBuffer.buffer.length); // Length of value (len(agentID) == 21 for evm address)
    metadata.writeBytes(evmAddressBuffer.buffer); //  Write value (bytes(agentID))

    /* Write allowance */
    // see https://github.com/iotaledger/wasp/blob/12845adea4fc097813a30a061853af4a43407d3c/packages/isc/assets.go#L348-L356 
    metadata.writeUInt8(128); // 0x80 flag meaning there are native tokens in the allowance
    metadata.writeUInt64SpecialEncoding(amount - gas); // IOTA amount to send
    // console.log(metadata.buffer.toString('hex'))
    return '0x' + metadata.buffer.toString('hex');
}
