// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

enum AddressType {
    Ed25519 = 0,
    Alias = 1,
    Nft = 2,
}

class Address {
    private _type: AddressType;
    private _hexEncodedString: String;

    constructor(type: AddressType, hexEncodedString: String) {
        this._type = type;
        this._hexEncodedString = hexEncodedString;
    }

    get type(): AddressType {
        return this._type;
    }

    get hexEncodedString(): String {
        return this._hexEncodedString;
    }
}

class Ed25519Address extends Address {
    constructor(address: String) {
        super(AddressType.Ed25519, address);
    }
}

class AliasAddress extends Address {
    constructor(address: String) {
        super(AddressType.Alias, address);
    }
}

class NftAddress extends Address {
    constructor(address: String) {
        super(AddressType.Nft, address);
    }
}

export { Address, Ed25519Address, AliasAddress, NftAddress };
