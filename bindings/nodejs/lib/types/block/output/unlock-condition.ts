// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { plainToInstance, Type } from 'class-transformer';
import { Address, AddressDiscriminator, AliasAddress } from '../address';

/**
 * All of the unlock condition types.
 */
enum UnlockConditionType {
    Address = 0,
    StorageDepositReturn = 1,
    Timelock = 2,
    Expiration = 3,
    StateControllerAddress = 4,
    GovernorAddress = 5,
    ImmutableAliasAddress = 6,
}

abstract class UnlockCondition {
    private type: UnlockConditionType;

    constructor(type: UnlockConditionType) {
        this.type = type;
    }
    /**
     * The type of unlock condition.
     */
    getType(): UnlockConditionType {
        return this.type;
    }

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

class AddressUnlockCondition extends UnlockCondition /*implements IAddressUnlockCondition*/ {
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    private address: Address;
    constructor(address: Address) {
        super(UnlockConditionType.Address);
        this.address = address;
    }

    /**
     * The address.
     */
    getAddress(): Address {
        return this.address;
    }
}
/**
 * Storage Deposit Return Unlock Condition.
 */
class StorageDepositReturnUnlockCondition extends UnlockCondition /*implements IStorageDepositReturnUnlockCondition*/ {
    private amount: string;

    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    private returnAddress: Address;

    constructor(returnAddress: Address, amount: bigint | string) {
        super(UnlockConditionType.StorageDepositReturn);
        if (typeof amount == 'bigint') {
            this.amount = amount.toString(10);
        } else {
            this.amount = amount;
        }
        this.returnAddress = returnAddress;
    }
    /**
     * Amount of tokens the consuming transaction must deposit to the address defined in return address.
     */
    getAmount(): bigint {
        return BigInt(this.amount);
    }

    /**
     * The return address.
     */
    getReturnAddress(): Address {
        return this.returnAddress;
    }
}
/**
 * Timelock Unlock Condition.
 */
class TimelockUnlockCondition extends UnlockCondition /*implements ITimelockUnlockCondition*/ {
    private unixTime: number;

    constructor(unixTime: number) {
        super(UnlockConditionType.Timelock);
        this.unixTime = unixTime;
    }
    /**
     * Unix time (seconds since Unix epoch) starting from which the output can be consumed.
     */
    getUnixTime(): number {
        return this.unixTime;
    }
}

class ExpirationUnlockCondition extends UnlockCondition /*implements IExpirationUnlockCondition*/ {
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    private returnAddress: Address;
    private unixTime: number;

    constructor(returnAddress: Address, unixTime: number) {
        super(UnlockConditionType.Expiration);
        this.returnAddress = returnAddress;
        this.unixTime = unixTime;
    }
    /**
     * Before this unix time, the condition is allowed to unlock the output,
     * after that only the address defined in return address.
     */
    getUnixTime(): number {
        return this.unixTime;
    }

    /**
     * The return address.
     */
    getReturnAddress(): Address {
        return this.returnAddress;
    }
}
/**
 * State Controller Address Unlock Condition.
 */
class StateControllerAddressUnlockCondition extends UnlockCondition /*implements IStateControllerAddressUnlockCondition*/ {
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    private address: Address;
    constructor(address: Address) {
        super(UnlockConditionType.StateControllerAddress);
        this.address = address;
    }

    /**
     * The address.
     */
    getAddress(): Address {
        return this.address;
    }
}
/**
 * Governor Unlock Condition.
 */
class GovernorAddressUnlockCondition extends UnlockCondition /*implements IGovernorAddressUnlockCondition*/ {
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    private address: Address;
    constructor(address: Address) {
        super(UnlockConditionType.GovernorAddress);
        this.address = address;
    }

    /**
     * The address.
     */
    getAddress(): Address {
        return this.address;
    }
}
/**
 * Immutable Alias Unlock Condition.
 */
class ImmutableAliasAddressUnlockCondition extends UnlockCondition /*implements IImmutableAliasAddressUnlockCondition*/ {
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    private address: Address;
    constructor(address: AliasAddress) {
        super(UnlockConditionType.ImmutableAliasAddress);
        this.address = address;
    }

    /**
     * The address.
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
