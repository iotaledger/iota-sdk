// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { HexEncodedString } from '../../utils';

/**
 * All of the block issuer key types.
 */
enum BlockIssuerKeyType {
    /** An Ed25519 public key hash block issuer key. */
    Ed25519PublicKeyHash = 0,
}

/** The base class for a block issuer key. */
abstract class BlockIssuerKey {
    readonly type: BlockIssuerKeyType;

    /**
     * @param type The type of block issuer key.
     */
    constructor(type: BlockIssuerKeyType) {
        this.type = type;
    }
}

/**
 * Ed25519 public key hash Block Issuer Key.
 */
class Ed25519PublicKeyHashBlockIssuerKey extends BlockIssuerKey {
    /**
     * An Ed25519 public key hash.
     */
    readonly pubKeyHash: HexEncodedString;

    constructor(pubKeyHash: HexEncodedString) {
        super(BlockIssuerKeyType.Ed25519PublicKeyHash);
        this.pubKeyHash = pubKeyHash;
    }
}

const BlockIssuerKeyDiscriminator = {
    property: 'type',
    subTypes: [
        {
            value: Ed25519PublicKeyHashBlockIssuerKey,
            name: BlockIssuerKeyType.Ed25519PublicKeyHash as any,
        },
    ],
};

export {
    BlockIssuerKeyDiscriminator,
    BlockIssuerKey,
    BlockIssuerKeyType,
    Ed25519PublicKeyHashBlockIssuerKey,
};
