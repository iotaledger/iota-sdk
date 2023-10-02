// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// Temp solution for not double parsing JSON
import { HexEncodedString } from '../utils';

/**
 * All of the signature types.
 */
enum SignatureType {
    /**
     * An Ed25519 signature.
     */
    Ed25519 = 0,
}

/**
 * The base class for signatures.
 */
abstract class Signature {
    readonly type: SignatureType;

    /**
     * @param type The type of signature.
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
 * An Ed25519 signature.
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
     * @param publicKey A Ed25519 public key as hex-encoded string.
     * @param signature A Ed25519 signature as hex-encoded string.
     */
    constructor(publicKey: HexEncodedString, signature: HexEncodedString) {
        super(SignatureType.Ed25519);
        this.publicKey = publicKey;
        this.signature = signature;
    }
}

export { SignatureType, Ed25519Signature, Signature };
