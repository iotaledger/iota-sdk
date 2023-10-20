// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { plainToInstance, Type } from 'class-transformer';
import { HexEncodedString } from '../utils';
import { AccountId, NftId } from './id';

/**
 * An address prepended by its network type.
 */
type Bech32Address = string;

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
    /** An implicit account creation address. */
    ImplicitAccountCreation = 24,
    /** An address with restricted capabilities. */
    Restricted = 40,
}

/**
 * The base class for addresses.
 */
abstract class Address {
    /**
     * The type of address.
     */
    readonly type: AddressType;

    /**
     * @param type The type of the address.
     */
    constructor(type: AddressType) {
        this.type = type;
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
        } else if (data.type == AddressType.ImplicitAccountCreation) {
            return plainToInstance(
                ImplicitAccountCreationAddress,
                data,
            ) as any as ImplicitAccountCreationAddress;
        } else if (data.type == AddressType.Restricted) {
            return plainToInstance(
                RestrictedAddress,
                data,
            ) as any as RestrictedAddress;
        }
        throw new Error('Invalid JSON');
    }
}
/**
 * An Ed25519 Address.
 */
class Ed25519Address extends Address {
    /**
     * The public key hash.
     */
    readonly pubKeyHash: HexEncodedString;

    /**
     * @param address An Ed25519 address as hex-encoded string.
     */
    constructor(address: HexEncodedString) {
        super(AddressType.Ed25519);
        this.pubKeyHash = address;
    }

    toString(): string {
        return this.pubKeyHash;
    }
}

/**
 * An Account address.
 */
class AccountAddress extends Address {
    /**
     * The account ID.
     */
    readonly accountId: AccountId;
    /**
     * @param address An account address as account ID.
     */
    constructor(address: AccountId) {
        super(AddressType.Account);
        this.accountId = address;
    }

    toString(): string {
        return this.accountId;
    }
}

/**
 * An NFT address.
 */
class NftAddress extends Address {
    /**
     * The NFT ID.
     */
    readonly nftId: NftId;
    /**
     * @param address An NFT address as NFT ID.
     */
    constructor(address: NftId) {
        super(AddressType.Nft);
        this.nftId = address;
    }

    toString(): string {
        return this.nftId;
    }
}

/**
 * An implicit account creation address.
 */
class ImplicitAccountCreationAddress extends Address {
    private pubKeyHash: HexEncodedString;
    /**
     * @param address An Ed25519 address.
     */
    constructor(address: Ed25519Address) {
        super(AddressType.ImplicitAccountCreation);
        this.pubKeyHash = address.pubKeyHash;
    }

    address(): Ed25519Address {
        return new Ed25519Address(this.pubKeyHash);
    }

    toString(): string {
        return this.address.toString();
    }
}

const RestrictedAddressDiscriminator = {
    property: 'type',
    subTypes: [
        { value: Ed25519Address, name: AddressType.Ed25519 as any },
        { value: AccountAddress, name: AddressType.Account as any },
        { value: NftAddress, name: AddressType.Nft as any },
    ],
};

/**
 * An address with restricted capabilities.
 */
class RestrictedAddress extends Address {
    /**
     * The inner address.
     */
    @Type(() => Address, {
        discriminator: RestrictedAddressDiscriminator,
    })
    readonly address: Address;
    /**
     * The allowed capabilities bitflags.
     */
    private allowedCapabilities: HexEncodedString = '0x00';
    /**
     * @param address An address.
     */
    constructor(address: Address) {
        super(AddressType.Restricted);
        this.address = address;
    }

    setAllowedCapabilities(allowedCapabilities: Uint8Array) {
        if (allowedCapabilities.some((c) => c != 0)) {
            this.allowedCapabilities =
                '0x' +
                allowedCapabilities.length.toString(16) +
                Buffer.from(
                    allowedCapabilities.buffer,
                    allowedCapabilities.byteOffset,
                    allowedCapabilities.byteLength,
                ).toString('hex');
        } else {
            this.allowedCapabilities = '0x00';
        }
    }

    withAllowedCapabilities(
        allowedCapabilities: Uint8Array,
    ): RestrictedAddress {
        this.setAllowedCapabilities(allowedCapabilities);
        return this;
    }

    getAllowedCapabilities(): Uint8Array {
        return Uint8Array.from(
            Buffer.from(this.allowedCapabilities.substring(2), 'hex'),
        );
    }

    toString(): string {
        return this.address.toString() + this.allowedCapabilities.substring(2);
    }
}

const AddressDiscriminator = {
    property: 'type',
    subTypes: [
        { value: Ed25519Address, name: AddressType.Ed25519 as any },
        { value: AccountAddress, name: AddressType.Account as any },
        { value: NftAddress, name: AddressType.Nft as any },
        {
            value: ImplicitAccountCreationAddress,
            name: AddressType.ImplicitAccountCreation as any,
        },
        { value: RestrictedAddress, name: AddressType.Restricted as any },
    ],
};

export {
    AddressDiscriminator,
    Bech32Address,
    Address,
    AddressType,
    Ed25519Address,
    AccountAddress,
    NftAddress,
};
