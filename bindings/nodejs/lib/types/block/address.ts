// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { plainToInstance } from 'class-transformer';
import { HexEncodedString } from '../utils';
import { AccountId, NftId } from './id';

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

    public static parse(data: any): Address {
        if (data.type == AddressType.Ed25519) {
            return plainToInstance(
                Ed25519Address,
                data,
            ) as any as Ed25519Address;
        } else if (data.type == AddressType.Alias) {
            return plainToInstance(AccountAddress, data) as any as AccountAddress;
        } else if (data.type == AddressType.Nft) {
            return plainToInstance(NftAddress, data) as any as NftAddress;
        }
        throw new Error('Invalid JSON');
    }
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

class AccountAddress extends Address {
    private accountId: AccountId;
    constructor(address: AccountId) {
        super(AddressType.Alias);
        this.accountId = address;
    }
    /**
     * The alias id.
     */
    getAccountId(): AccountId {
        return this.accountId;
    }

    toString(): string {
        return this.getAccountId();
    }
}
/**
 * NFT address.
 */
class NftAddress extends Address {
    private nftId: NftId;
    constructor(address: NftId) {
        super(AddressType.Nft);
        this.nftId = address;
    }
    /**
     * The NFT Id.
     */
    getNftId(): NftId {
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
        { value: AccountAddress, name: AddressType.Alias as any },
        { value: NftAddress, name: AddressType.Nft as any },
    ],
};

export {
    AddressDiscriminator,
    Address,
    AddressType,
    Ed25519Address,
    AccountAddress,
    NftAddress,
};
