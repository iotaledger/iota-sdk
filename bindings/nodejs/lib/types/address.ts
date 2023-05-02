// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    ALIAS_ADDRESS_TYPE,
    ED25519_ADDRESS_TYPE,
    HexEncodedString,
    NFT_ADDRESS_TYPE,
} from '@iota/types';

enum AddressType {
    Ed25519 = ED25519_ADDRESS_TYPE,
    Alias = ALIAS_ADDRESS_TYPE,
    Nft = NFT_ADDRESS_TYPE,
}

abstract class Address {
    private _type: AddressType;
    protected _hexEncodedString: HexEncodedString;

    constructor(type: AddressType, hexEncodedString: HexEncodedString) {
        this._type = type;
        this._hexEncodedString = hexEncodedString;
    }
    /**
     * The type of address.
     */
    get type(): AddressType {
        return this._type;
    }
}
/**
 * Ed25519Address address.
 */
class Ed25519Address extends Address {
    constructor(address: HexEncodedString) {
        super(AddressType.Ed25519, address);
    }
    /**
     * The public key hash.
     */
    get pubKeyHash(): HexEncodedString {
        return this._hexEncodedString;
    }
}

class AliasAddress extends Address {
    constructor(address: HexEncodedString) {
        super(AddressType.Alias, address);
    }
    /**
     * The alias id.
     */
    get aliasId(): HexEncodedString {
        return this._hexEncodedString;
    }
}
/**
 * NFT address.
 */
class NftAddress extends Address {
    constructor(address: HexEncodedString) {
        super(AddressType.Nft, address);
    }
    /**
     * The NFT Id.
     */
    get nftId(): HexEncodedString {
        return this._hexEncodedString;
    }
}

export { Address, Ed25519Address, AliasAddress, NftAddress };
