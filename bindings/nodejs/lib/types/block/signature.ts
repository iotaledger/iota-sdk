// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// Temp solution for not double parsing JSON
import { HexEncodedString } from '../utils';

/**
 * All of the signature types.
 */
enum SignatureType {
    Ed25519 = 0,
}

abstract class Signature {
    private type: SignatureType;

    constructor(type: SignatureType) {
        this.type = type;
    }

    /**
     * The type of signature.
     */
    getType(): SignatureType {
        return this.type;
    }
}

/**
 * Ed25519Signature signature.
 */
class Ed25519Signature extends Signature {
    /**
     * The public key.
     */
    publicKey: HexEncodedString;
    /**
     * The signature.
     */
    signature: HexEncodedString;

    constructor(publicKey: HexEncodedString, signature: HexEncodedString) {
        super(SignatureType.Ed25519);
        this.publicKey = publicKey;
        this.signature = signature;
    }
}

const SignatureDiscriminator = {
    property: 'type',
    subTypes: [{ value: Ed25519Signature, name: SignatureType.Ed25519 as any }],
};

export { SignatureDiscriminator, SignatureType, Ed25519Signature, Signature };
