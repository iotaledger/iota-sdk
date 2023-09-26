import { Buffer } from 'buffer';

export class SimpleBufferCursor {
  private _buffer: Buffer;
  private _traverse: number;

  get buffer(): Buffer {
    return this._buffer;
  }

  constructor(buffer: Buffer = Buffer.alloc(0)) {
    this._buffer = buffer;
    this._traverse = 0;
  }

  readIntBE(length: number): number {
    const value = this._buffer.readIntBE(this._traverse, length);
    this._traverse += length;

    return value;
  }

  readUInt32LE(): number {
    const value = this._buffer.readUInt32LE(this._traverse);
    this._traverse += 4;

    return value;
  }

  readUInt64LE(): BigInt {
    const value = this._buffer.readBigUInt64LE(this._traverse);
    this._traverse += 8;

    return value;
  }

  readUInt16LE(): number {
    const value = this._buffer.readUInt16LE(this._traverse);
    this._traverse += 2;

    return value;
  }

  readBytes(length: number): Uint8Array {
    const subBuffer = this._buffer.subarray(
      this._traverse,
      this._traverse + length,
    );
    this._traverse += length;

    return subBuffer;
  }

  writeIntBE(value: number, length: number): void {
    const nBuffer = Buffer.alloc(length);
    nBuffer.writeIntBE(value, 0, length);

    this._buffer = Buffer.concat([this._buffer, nBuffer]);
  }

  writeInt8(value: number): void {
    const nBuffer = Buffer.alloc(1);
    nBuffer.writeInt8(value, 0);

    this._buffer = Buffer.concat([this._buffer, nBuffer]);
  }

  writeUInt8(value: number): void {
    const nBuffer = Buffer.alloc(1);
    nBuffer.writeUInt8(value, 0);

    this._buffer = Buffer.concat([this._buffer, nBuffer]);
  }

  writeUInt32LE(value: number): void {
    const nBuffer = Buffer.alloc(4);
    nBuffer.writeUInt32LE(value, 0);

    this._buffer = Buffer.concat([this._buffer, nBuffer]);
  }


  writeUInt16LE(value: number): void {
    const nBuffer = Buffer.alloc(2);
    nBuffer.writeUInt16LE(value, 0);

    this._buffer = Buffer.concat([this._buffer, nBuffer]);
  }

  writeUint8Array(bytes: Uint8Array) {
    for (let i = 0; i < bytes.length; i++) {
      this.writeUInt8(bytes[i]);
    }
  }

  writeBytes(bytes: Buffer): void {
    this._buffer = Buffer.concat([this._buffer, bytes]);
  }

  writeUInt64SpecialEncoding(value: BigInt): void {
    this.writeBytes(size64Encode(value.valueOf()))
  }

  writeUInt32SpecialEncoding(value: number): void {
    this.writeBytes(size64Encode(BigInt(value)))
  }

}

function shiftRight(s: bigint, n: bigint): bigint {
  return s / (BigInt(2) ** n);
}

// from https://github.com/iotaledger/wasp/blob/12845adea4fc097813a30a061853af4a43407d3c/packages/util/rwutil/convert.go#L113
function size64Encode(n: bigint): Buffer {
  if (n < BigInt(0x80)) {
    return Buffer.from([Number(n)]);
  } else if (n < BigInt(0x4000)) {
    return Buffer.from([Number(n | BigInt(0x80)), Number(shiftRight(n, BigInt(7)))]);
  } else if (n < BigInt(0x20_0000)) {
    return Buffer.from([Number(n | BigInt(0x80)), Number(shiftRight(n, BigInt(7)) | BigInt(0x80)), Number(shiftRight(n, BigInt(14)))]);
  } else if (n < BigInt(0x1000_0000)) {
    return Buffer.from([Number(n | BigInt(0x80)), Number(shiftRight(n, BigInt(7)) | BigInt(0x80)), Number(shiftRight(n, BigInt(14)) | BigInt(0x80)), Number(shiftRight(n, BigInt(21)))]);
  } else if (n < BigInt(0x8_0000_0000)) {
    return Buffer.from([Number(n | BigInt(0x80)), Number(shiftRight(n, BigInt(7)) | BigInt(0x80)), Number(shiftRight(n, BigInt(14)) | BigInt(0x80)), Number(shiftRight(n, BigInt(21)) | BigInt(0x80)), Number(shiftRight(n, BigInt(28)))]);
  } else if (n < BigInt(0x400_0000_0000)) {
    return Buffer.from([Number(n | BigInt(0x80)), Number(shiftRight(n, BigInt(7)) | BigInt(0x80)), Number(shiftRight(n, BigInt(14)) | BigInt(0x80)), Number(shiftRight(n, BigInt(21)) | BigInt(0x80)), Number(shiftRight(n, BigInt(28)) | BigInt(0x80)), Number(shiftRight(n, BigInt(35)))]);
  } else if (n < BigInt(0x2_0000_0000_0000)) {
    return Buffer.from([Number(n | BigInt(0x80)), Number(shiftRight(n, BigInt(7)) | BigInt(0x80)), Number(shiftRight(n, BigInt(14)) | BigInt(0x80)), Number(shiftRight(n, BigInt(21)) | BigInt(0x80)), Number(shiftRight(n, BigInt(28)) | BigInt(0x80)), Number(shiftRight(n, BigInt(35)) | BigInt(0x80)), Number(shiftRight(n, BigInt(42)))]);
  } else if (n < BigInt(0x100_0000_0000_0000)) {
    return Buffer.from([Number(n | BigInt(0x80)), Number(shiftRight(n, BigInt(7)) | BigInt(0x80)), Number(shiftRight(n, BigInt(14)) | BigInt(0x80)), Number(shiftRight(n, BigInt(21)) | BigInt(0x80)), Number(shiftRight(n, BigInt(28)) | BigInt(0x80)), Number(shiftRight(n, BigInt(35)) | BigInt(0x80)), Number(shiftRight(n, BigInt(42)) | BigInt(0x80)), Number(shiftRight(n, BigInt(49)))]);
  } else if (n < BigInt(0x8000_0000_0000_0000)) {
    return Buffer.from([Number(n | BigInt(0x80)), Number(shiftRight(n, BigInt(7)) | BigInt(0x80)), Number(shiftRight(n, BigInt(14)) | BigInt(0x80)), Number(shiftRight(n, BigInt(21)) | BigInt(0x80)), Number(shiftRight(n, BigInt(28)) | BigInt(0x80)), Number(shiftRight(n, BigInt(35)) | BigInt(0x80)), Number(shiftRight(n, BigInt(42)) | BigInt(0x80)), Number(shiftRight(n, BigInt(49)) | BigInt(0x80)), Number(shiftRight(n, BigInt(56)))]);
  } else {
    return Buffer.from([Number(n | BigInt(0x80)), Number(shiftRight(n, BigInt(7)) | BigInt(0x80)), Number(shiftRight(n, BigInt(14)) | BigInt(0x80)), Number(shiftRight(n, BigInt(21)) | BigInt(0x80)), Number(shiftRight(n, BigInt(28)) | BigInt(0x80)), Number(shiftRight(n, BigInt(35)) | BigInt(0x80)), Number(shiftRight(n, BigInt(42)) | BigInt(0x80)), Number(shiftRight(n, BigInt(49)) | BigInt(0x80)), Number(shiftRight(n, BigInt(56)) | BigInt(0x80)), Number(shiftRight(n, BigInt(63)))]);
  }
}