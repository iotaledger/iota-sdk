// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { plainToInstance, Type } from 'class-transformer';
import { NumericString } from '../../utils';
import { Address, AddressDiscriminator, AliasAddress } from '../address';

/**
 * All of the unlock condition types.
 */
enum UnlockConditionType {
    /** An address unlock condition. */
    Address = 0,
    /** A storage deposit return unlock condition. */
    StorageDepositReturn = 1,
    /** A timelock unlock condition. */
    Timelock = 2,
    /** An expiration unlock condition. */
    Expiration = 3,
    /** A state controller address unlock condition. */
    StateControllerAddress = 4,
    /** A governor address unlock condition. */
    GovernorAddress = 5,
    /** An immutable alias address unlock condition. */
    ImmutableAliasAddress = 6,
}

abstract class UnlockCondition {
    readonly type: UnlockConditionType;

    /**
     * @param type The type of the unlock condition.
     */
    constructor(type: UnlockConditionType) {
        this.type = type;
    }
    /**
     * Get the type of unlock condition.
     */
    getType(): UnlockConditionType {
        return this.type;
    }

    /**
     * Parse an unlock condition from a plain JS JSON object.
     */
    public static parse(data: any): UnlockCondition {
        if (data.type == UnlockConditionType.Address) {
            return plainToInstance(
                AddressUnlockCondition,
                data,
            ) as any as AddressUnlockCondition;
        } else if (data.type == UnlockConditionType.StorageDepositReturn) {
            return plainToInstance(
                StorageDepositReturnUnlockCondition,
                data,
            ) as any as StorageDepositReturnUnlockCondition;
        } else if (data.type == UnlockConditionType.Timelock) {
            return plainToInstance(
                TimelockUnlockCondition,
                data,
            ) as any as TimelockUnlockCondition;
        } else if (data.type == UnlockConditionType.Expiration) {
            return plainToInstance(
                ExpirationUnlockCondition,
                data,
            ) as any as ExpirationUnlockCondition;
        } else if (data.type == UnlockConditionType.StateControllerAddress) {
            return plainToInstance(
                StateControllerAddressUnlockCondition,
                data,
            ) as any as StateControllerAddressUnlockCondition;
        } else if (data.type == UnlockConditionType.GovernorAddress) {
            return plainToInstance(
                GovernorAddressUnlockCondition,
                data,
            ) as any as GovernorAddressUnlockCondition;
        } else if (data.type == UnlockConditionType.ImmutableAliasAddress) {
            return plainToInstance(
                ImmutableAliasAddressUnlockCondition,
                data,
            ) as any as ImmutableAliasAddressUnlockCondition;
        }
        throw new Error('Invalid JSON');
    }
}

/**
 * An address unlock condition.
 */
class AddressUnlockCondition extends UnlockCondition /*implements IAddressUnlockCondition*/ {
    /**
     * An address unlocked with a private key.
     */
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    readonly address: Address;

    /**
     * @param address The address that needs to be unlocked with a private key.
     */
    constructor(address: Address) {
        super(UnlockConditionType.Address);
        this.address = address;
    }

    /**
     * Get the address.
     */
    getAddress(): Address {
        return this.address;
    }
}
/**
 * A Storage Deposit Return Unlock Condition.
 */
class StorageDepositReturnUnlockCondition extends UnlockCondition /*implements IStorageDepositReturnUnlockCondition*/ {
    /**
     * The amount the consuming transaction must deposit to `returnAddress`.
     */
    readonly amount: NumericString;

    /**
     * The address to return the amount to.
     */
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    readonly returnAddress: Address;

    /**
     * @param returnAddress The address to return the amount to.
     * @param amount The amount the consuming transaction must deposit to `returnAddress`.
     */
    constructor(returnAddress: Address, amount: bigint | NumericString) {
        super(UnlockConditionType.StorageDepositReturn);
        if (typeof amount == 'bigint') {
            this.amount = amount.toString(10);
        } else {
            this.amount = amount;
        }
        this.returnAddress = returnAddress;
    }
    /**
     * Get the amount.
     */
    getAmount(): bigint {
        return BigInt(this.amount);
    }

    /**
     * Get the return address.
     */
    getReturnAddress(): Address {
        return this.returnAddress;
    }
}
/**
 * A Timelock Unlock Condition.
 */
class TimelockUnlockCondition extends UnlockCondition /*implements ITimelockUnlockCondition*/ {
    /**
     * The Unix time (seconds since Unix epoch) starting from which the output can be consumed.
     */
    readonly unixTime: number;

    /**
     * @param unixTime The Unix timestamp marking the end of the timelock.
     */
    constructor(unixTime: number) {
        super(UnlockConditionType.Timelock);
        this.unixTime = unixTime;
    }
    /**
     * Get the end of the timelock as Unix time.
     */
    getUnixTime(): number {
        return this.unixTime;
    }
}

/**
 * An Expiration Unlock Condition.
 */
class ExpirationUnlockCondition extends UnlockCondition /*implements IExpirationUnlockCondition*/ {
    /**
     * The return address if the output was not claimed in time.
     */
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    readonly returnAddress: Address;
    /**
     * Before this timestamp, the condition is allowed to unlock the output,
     * after that only the address defined in return address.
     */
    readonly unixTime: number;

    /**
     * @param returnAddress The address that can unlock the expired output.
     * @param unixTime The Unix timestamp marking the end of the claim period.
     */
    constructor(returnAddress: Address, unixTime: number) {
        super(UnlockConditionType.Expiration);
        this.returnAddress = returnAddress;
        this.unixTime = unixTime;
    }
    /**
     * Get the end of the expiration period as Unix time.
     */
    getUnixTime(): number {
        return this.unixTime;
    }

    /**
     * Get the return address.
     */
    getReturnAddress(): Address {
        return this.returnAddress;
    }
}
/**
 * A State Controller Address Unlock Condition.
 */
class StateControllerAddressUnlockCondition extends UnlockCondition /*implements IStateControllerAddressUnlockCondition*/ {
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    readonly address: Address;
    /**
     * @param address The State Controller address that is allowed to do state transitions.
     */
    constructor(address: Address) {
        super(UnlockConditionType.StateControllerAddress);
        this.address = address;
    }

    /**
     * Get the State Controller address.
     */
    getAddress(): Address {
        return this.address;
    }
}
/**
 * A Governor Address Unlock Condition.
 */
class GovernorAddressUnlockCondition extends UnlockCondition /*implements IGovernorAddressUnlockCondition*/ {
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    readonly address: Address;
    /**
     * @param address The governor address that is allowed to do governance transitions.
     */
    constructor(address: Address) {
        super(UnlockConditionType.GovernorAddress);
        this.address = address;
    }

    /**
     * Get the Governor address.
     */
    getAddress(): Address {
        return this.address;
    }
}
/**
 * An Immutable Alias Address Unlock Condition.
 */
class ImmutableAliasAddressUnlockCondition extends UnlockCondition /*implements IImmutableAliasAddressUnlockCondition*/ {
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    readonly address: Address;
    /**
     * @param address The Immutable Alias address that owns the output.
     */
    constructor(address: AliasAddress) {
        super(UnlockConditionType.ImmutableAliasAddress);
        this.address = address;
    }

    /**
     * Get the Immutable Alias address.
     */
    getAddress(): Address {
        return this.address;
    }
}

const UnlockConditionDiscriminator = {
    property: 'type',
    subTypes: [
        {
            value: AddressUnlockCondition,
            name: UnlockConditionType.Address as any,
        },
        {
            value: StorageDepositReturnUnlockCondition,
            name: UnlockConditionType.StorageDepositReturn as any,
        },
        {
            value: TimelockUnlockCondition,
            name: UnlockConditionType.Timelock as any,
        },
        {
            value: ExpirationUnlockCondition,
            name: UnlockConditionType.Expiration as any,
        },
        {
            value: StateControllerAddressUnlockCondition,
            name: UnlockConditionType.StateControllerAddress as any,
        },
        {
            value: GovernorAddressUnlockCondition,
            name: UnlockConditionType.GovernorAddress as any,
        },
        {
            value: ImmutableAliasAddressUnlockCondition,
            name: UnlockConditionType.ImmutableAliasAddress as any,
        },
    ],
};

export {
    UnlockConditionDiscriminator,
    UnlockCondition,
    UnlockConditionType,
    AddressUnlockCondition,
    StorageDepositReturnUnlockCondition,
    TimelockUnlockCondition,
    ExpirationUnlockCondition,
    StateControllerAddressUnlockCondition,
    GovernorAddressUnlockCondition,
    ImmutableAliasAddressUnlockCondition,
};
