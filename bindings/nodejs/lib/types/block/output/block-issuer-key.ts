// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { HexEncodedString } from '../../utils';
/**
 * All of the block issuer key types.
 */
enum BlockIssuerKeyType {
    /** An Ed25519 block issuer key. */
    Ed25519 = 0,
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
 * Ed25519 Block Issuer Key.
 */
class Ed25519BlockIssuerKey extends BlockIssuerKey {
    /**
     * An Ed25519 public key.
     */
    readonly publicKey: HexEncodedString;

    constructor(publicKey: HexEncodedString) {
        super(BlockIssuerKeyType.Ed25519);
        this.publicKey = publicKey;
    }
}


export {
    BlockIssuerKey,
    BlockIssuerKeyType,
    Ed25519BlockIssuerKey,
};
