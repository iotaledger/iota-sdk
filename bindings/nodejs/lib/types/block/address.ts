// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { plainToInstance } from 'class-transformer';
import { HexEncodedString } from '../utils';
import { AccountId, NftId } from './id';

/**
 * Address type variants.
 */
enum AddressType {
    /** An Ed25519 address. */
    Ed25519 = 0,
    /** An Account address. */
    Account = 8,
    /** An NFT address. */
    Nft = 16,
}

/**
 * The base class for addresses.
 */
abstract class Address {
    readonly type: AddressType;

    /**
     * @param type The type of the address.
     */
    constructor(type: AddressType) {
        this.type = type;
    }
    /**
     * Get the type of address.
     */
    getType(): AddressType {
        return this.type;
    }

    abstract toString(): string;

    /**
     * Parse an address from a JSON string.
     */
    public static parse(data: any): Address {
        if (data.type == AddressType.Ed25519) {
            return plainToInstance(
                Ed25519Address,
                data,
            ) as any as Ed25519Address;
        } else if (data.type == AddressType.Account) {
            return plainToInstance(
                AccountAddress,
                data,
            ) as any as AccountAddress;
        } else if (data.type == AddressType.Nft) {
            return plainToInstance(NftAddress, data) as any as NftAddress;
        }
        throw new Error('Invalid JSON');
    }
}
/**
 * An Ed25519 Address.
 */
class Ed25519Address extends Address {
    readonly pubKeyHash: HexEncodedString;

    /**
     * @param address An Ed25519 address as hex-encoded string.
     */
    constructor(address: HexEncodedString) {
        super(AddressType.Ed25519);
        this.pubKeyHash = address;
    }
    /**
     * Get the public key hash.
     */
    getPubKeyHash(): HexEncodedString {
        return this.pubKeyHash;
    }

    toString(): string {
        return this.getPubKeyHash();
    }
}

/**
 * An Account address.
 */
class AccountAddress extends Address {
    readonly accountId: AccountId;
    /**
     * @param address An account address as account ID.
     */
    constructor(address: AccountId) {
        super(AddressType.Account);
        this.accountId = address;
    }
    /**
     * Get the account ID.
     */
    getAccountId(): AccountId {
        return this.accountId;
    }

    toString(): string {
        return this.getAccountId();
    }
}
/**
 * An NFT address.
 */
class NftAddress extends Address {
    readonly nftId: NftId;
    /**
     * @param address An NFT address as NFT ID.
     */
    constructor(address: NftId) {
        super(AddressType.Nft);
        this.nftId = address;
    }
    /**
     * Get the NFT ID.
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
        { value: AccountAddress, name: AddressType.Account as any },
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
