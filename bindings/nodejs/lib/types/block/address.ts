// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { plainToInstance, Type } from 'class-transformer';
import { HexEncodedString } from '../utils';
import { AccountId, AnchorId, NftId } from './id';

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
    /** An Anchor address. */
    Anchor = 24,
    /** An implicit account creation address. */
    ImplicitAccountCreation = 32,
    /** A Multi address. */
    Multi = 40,
    /** An address with restricted capabilities. */
    Restricted = 48,
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
        } else if (data.type == AddressType.Anchor) {
            return plainToInstance(AnchorAddress, data) as any as AnchorAddress;
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
     * @param pubKeyHash BLAKE2b-256 hash of an Ed25519 public key as hex-encoded string.
     */
    constructor(pubKeyHash: HexEncodedString) {
        super(AddressType.Ed25519);
        this.pubKeyHash = pubKeyHash;
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
 * An Anchor address.
 */
class AnchorAddress extends Address {
    /**
     * The anchor ID.
     */
    readonly anchorId: AnchorId;
    /**
     * @param address An anchor address as anchor ID.
     */
    constructor(address: AnchorId) {
        super(AddressType.Anchor);
        this.anchorId = address;
    }

    toString(): string {
        return this.anchorId;
    }
}

/**
 * An implicit account creation address that can be used to convert a Basic Output to an Account Output.
 */
class ImplicitAccountCreationAddress extends Address {
    private pubKeyHash: HexEncodedString;
    /**
     * @param pubKeyHash BLAKE2b-256 hash of an Ed25519 public key as hex-encoded string.
     */
    constructor(pubKeyHash: HexEncodedString) {
        super(AddressType.ImplicitAccountCreation);
        this.pubKeyHash = pubKeyHash;
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
        keepDiscriminatorProperty: true,
    })
    readonly address: Address;
    /**
     * The allowed capabilities bitflags.
     */
    private allowedCapabilities?: HexEncodedString;
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
                Buffer.from(
                    allowedCapabilities.buffer,
                    allowedCapabilities.byteOffset,
                    allowedCapabilities.byteLength,
                ).toString('hex');
        } else {
            this.allowedCapabilities = undefined;
        }
    }

    withAllowedCapabilities(
        allowedCapabilities: Uint8Array,
    ): RestrictedAddress {
        this.setAllowedCapabilities(allowedCapabilities);
        return this;
    }

    getAllowedCapabilities(): Uint8Array {
        return this.allowedCapabilities !== undefined
            ? Uint8Array.from(
                  Buffer.from(this.allowedCapabilities.substring(2), 'hex'),
              )
            : new Uint8Array();
    }

    toString(): string {
        return (
            this.address.toString() +
            (this.allowedCapabilities !== undefined
                ? this.allowedCapabilities.substring(2)
                : '')
        );
    }
}

/**
 * A weighted address.
 */
class WeightedAddress {
    /**
     * The unlocked address.
     */
    @Type(() => Address, {
        discriminator: {
            property: 'type',
            subTypes: [
                { value: Ed25519Address, name: AddressType.Ed25519 as any },
                { value: AccountAddress, name: AddressType.Account as any },
                { value: NftAddress, name: AddressType.Nft as any },
                { value: AnchorAddress, name: AddressType.Anchor as any },
            ],
        },
    })
    readonly address: Address;
    /**
     * The weight of the unlocked address.
     */
    readonly weight: number;

    /**
     * @param address The unlocked address.
     * @param weight The weight of the unlocked address.
     */
    constructor(address: Address, weight: number) {
        this.address = address;
        this.weight = weight;
    }
}

/**
 * An address that consists of addresses with weights and a threshold value.
 * The Multi Address can be unlocked if the cumulative weight of all unlocked addresses is equal to or exceeds the
 * threshold.
 */
class MultiAddress extends Address {
    /**
     * The weighted unlocked addresses.
     */
    readonly addresses: WeightedAddress[];
    /**
     * The threshold that needs to be reached by the unlocked addresses in order to unlock the multi address.
     */
    readonly threshold: number;

    /**
     * @param addresses The weighted unlocked addresses.
     * @param threshold The threshold that needs to be reached by the unlocked addresses in order to unlock the multi address.
     */
    constructor(addresses: WeightedAddress[], threshold: number) {
        super(AddressType.Multi);
        this.addresses = addresses;
        this.threshold = threshold;
    }

    toString(): string {
        return JSON.stringify(this);
    }
}

const AddressDiscriminator = {
    property: 'type',
    subTypes: [
        { value: Ed25519Address, name: AddressType.Ed25519 as any },
        { value: AccountAddress, name: AddressType.Account as any },
        { value: NftAddress, name: AddressType.Nft as any },
        { value: AnchorAddress, name: AddressType.Anchor as any },
        {
            value: ImplicitAccountCreationAddress,
            name: AddressType.ImplicitAccountCreation as any,
        },
        { value: MultiAddress, name: AddressType.Multi as any },
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
    AnchorAddress,
    ImplicitAccountCreationAddress,
    WeightedAddress,
    MultiAddress,
    RestrictedAddress,
};
