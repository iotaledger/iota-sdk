// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// Temp solution for not double parsing JSON
import { HexEncodedString } from '../utils';

/**
 * All of the signature types.
 */
enum SignatureType {
    /**
     * TODO.
     */
    Ed25519 = 0,
}

/**
 * TODO.
 */
abstract class Signature {
    readonly type: SignatureType;

    /**
     * TODO.
     */
    constructor(type: SignatureType) {
        this.type = type;
    }

    /**
     * Get the type of signature.
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

    /**
     * TODO.
     */
    constructor(publicKey: HexEncodedString, signature: HexEncodedString) {
        super(SignatureType.Ed25519);
        this.publicKey = publicKey;
        this.signature = signature;
    }
}

export { SignatureType, Ed25519Signature, Signature };
