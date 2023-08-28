// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { HexEncodedString } from '../utils';

/**
 * All of the public key types.
 */
enum PublicKeyType {
    Ed25519 = 0,
}

/**
 * A public key.
 */
abstract class PublicKey {
    readonly type: PublicKeyType;

    constructor(type: PublicKeyType) {
        this.type = type;
    }
}

/**
 * Ed25519 public key.
 */
class Ed25519PublicKey extends PublicKey {
    /**
     * The public key.
     */
    readonly publicKey: HexEncodedString;

    constructor(publicKey: HexEncodedString) {
        super(PublicKeyType.Ed25519);
        this.publicKey = publicKey;
    }
}

const PublicKeyDiscriminator = {
    property: 'type',
    subTypes: [{ value: Ed25519PublicKey, name: PublicKeyType.Ed25519 as any }],
};

export { PublicKeyDiscriminator, Ed25519PublicKey, PublicKey, PublicKeyType };
