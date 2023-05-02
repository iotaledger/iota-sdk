// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { HexEncodedString } from '@iota/types';

enum AddressType {
    Ed25519 = 0,
    Alias = 8,
    Nft = 16,
}

abstract class Address {
    private type: AddressType;

    constructor(type: AddressType) {
        this.type = type;
    }
    /**
     * The type of address.
     */
    getType(): AddressType {
        return this.type;
    }
}
/**
 * Ed25519Address address.
 */
class Ed25519Address extends Address {
    private pubKeyHash: HexEncodedString;
    constructor(address: HexEncodedString) {
        super(AddressType.Ed25519);
        this.pubKeyHash = address;
    }
    /**
     * The public key hash.
     */
    getPubKeyHash(): HexEncodedString {
        return this.pubKeyHash;
    }
}

class AliasAddress extends Address {
    private aliasId: HexEncodedString;
    constructor(address: HexEncodedString) {
        super(AddressType.Alias);
        this.aliasId = address;
    }
    /**
     * The alias id.
     */
    getAliasId(): HexEncodedString {
        return this.aliasId;
    }
}
/**
 * NFT address.
 */
class NftAddress extends Address {
    private nftId: HexEncodedString;
    constructor(address: HexEncodedString) {
        super(AddressType.Nft);
        this.nftId = address;
    }
    /**
     * The NFT Id.
     */
    getNftId(): HexEncodedString {
        return this.nftId;
    }
}

export { Address, Ed25519Address, AliasAddress, NftAddress };
