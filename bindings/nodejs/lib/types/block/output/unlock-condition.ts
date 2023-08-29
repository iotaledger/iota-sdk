// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { plainToInstance, Type } from 'class-transformer';
import { u64 } from '../../utils';
import { Address, AddressDiscriminator, AliasAddress } from '../address';
import { SlotIndex } from '../slot';

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
    readonly type: UnlockConditionType;

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

class AddressUnlockCondition extends UnlockCondition {
    /**
     * The address.
     */
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    readonly address: Address;
    constructor(address: Address) {
        super(UnlockConditionType.Address);
        this.address = address;
    }
}
/**
 * Storage Deposit Return Unlock Condition.
 */
class StorageDepositReturnUnlockCondition extends UnlockCondition {
    private amount: string;

    /**
     * The return address.
     */
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    readonly returnAddress: Address;

    constructor(returnAddress: Address, amount: u64 | string) {
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
    getAmount(): u64 {
        return BigInt(this.amount);
    }
}
/**
 * Defines a slot index until which the output can not be unlocked.
 */
class TimelockUnlockCondition extends UnlockCondition {
    /**
     * Slot index starting from which the output can be consumed.
     */
    readonly slotIndex: SlotIndex;

    constructor(slotIndex: SlotIndex) {
        super(UnlockConditionType.Timelock);
        this.slotIndex = slotIndex;
    }
}
/**
 * Defines an expiration slot index. Before the slot index is reached, only the Address defined in the Address
 * Unlock Condition is allowed to unlock the output. Afterward, only the Return Address can unlock it.
 */
class ExpirationUnlockCondition extends UnlockCondition {
    /**
     * The return address.
     */
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    readonly returnAddress: Address;
    /**
     * Before this slot index, the condition is allowed to unlock the output,
     * after that only the address defined in return address.
     */
    readonly slotIndex: SlotIndex;

    constructor(returnAddress: Address, slotIndex: SlotIndex) {
        super(UnlockConditionType.Expiration);
        this.returnAddress = returnAddress;
        this.slotIndex = slotIndex;
    }
}
/**
 * State Controller Address Unlock Condition.
 */
class StateControllerAddressUnlockCondition extends UnlockCondition {
    /**
     * The address.
     */
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    readonly address: Address;
    constructor(address: Address) {
        super(UnlockConditionType.StateControllerAddress);
        this.address = address;
    }
}
/**
 * Governor Unlock Condition.
 */
class GovernorAddressUnlockCondition extends UnlockCondition {
    /**
     * The address.
     */
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    readonly address: Address;
    constructor(address: Address) {
        super(UnlockConditionType.GovernorAddress);
        this.address = address;
    }
}
/**
 * Immutable Alias Unlock Condition.
 */
class ImmutableAliasAddressUnlockCondition extends UnlockCondition {
    /**
     * The address.
     */
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    readonly address: Address;
    constructor(address: AliasAddress) {
        super(UnlockConditionType.ImmutableAliasAddress);
        this.address = address;
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
