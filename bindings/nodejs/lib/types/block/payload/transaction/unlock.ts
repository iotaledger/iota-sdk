// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Ed25519Signature } from '../../..';

/**
 * All of the unlock types.
 */
enum UnlockType {
    Signature = 0,
    Reference = 1,
    Alias = 2,
    Nft = 3,
}

abstract class Unlock {
    private type: UnlockType;

    constructor(type: UnlockType) {
        this.type = type;
    }

    /**
     * The type of unlock.
     */
    getType(): UnlockType {
        return this.type;
    }
}

/**
 * An unlock holding one or more signatures unlocking one or more inputs..
 */
class SignatureUnlock extends Unlock {
    /**
     * The signature.
     */
    signature: Ed25519Signature;

    constructor(signature: Ed25519Signature) {
        super(UnlockType.Signature);
        this.signature = signature;
    }
}

/**
 * An unlock which must reference a previous unlock which unlocks
 * also the input at the same index as this Reference Unlock.
 */
class ReferenceUnlock extends Unlock {
    /**
     * The reference.
     */
    reference: number;

    constructor(reference: number) {
        super(UnlockType.Reference);
        this.reference = reference;
    }
}

/**
 * An unlock which must reference a previous unlock which unlocks the alias that the input is locked to.
 */
class AliasUnlock extends Unlock {
    /**
     * The reference.
     */
    reference: number;

    constructor(reference: number) {
        super(UnlockType.Alias);
        this.reference = reference;
    }
}

/**
 * An unlock which must reference a previous unlock which unlocks the NFT that the input is locked to.
 */
class NftUnlock extends Unlock {
    /**
     * The reference.
     */
    reference: number;

    constructor(reference: number) {
        super(UnlockType.Nft);
        this.reference = reference;
    }
}

export {
    UnlockType,
    Unlock,
    SignatureUnlock,
    ReferenceUnlock,
    AliasUnlock,
    NftUnlock,
};
