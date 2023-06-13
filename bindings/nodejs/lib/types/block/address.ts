// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { HexEncodedString } from '../utils';

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

    abstract toString(): string;
}
/**
 * Ed25519 Address.
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

    toString(): string {
        return this.getPubKeyHash();
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

    toString(): string {
        return this.getAliasId();
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

    toString(): string {
        return this.getNftId();
    }
}

const AddressDiscriminator = {
    property: 'type',
    subTypes: [
        { value: Ed25519Address, name: AddressType.Ed25519 as any },
        { value: AliasAddress, name: AddressType.Alias as any },
        { value: NftAddress, name: AddressType.Nft as any },
    ],
};

export {
    AddressDiscriminator,
    Address,
    Ed25519Address,
    AliasAddress,
    NftAddress,
};
