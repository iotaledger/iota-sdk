// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { plainToInstance, Type } from 'class-transformer';
import { u64 } from '../../utils';
import { Address, AddressDiscriminator, AccountAddress } from '../address';
import { SlotIndex } from '../slot';

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
    /** An immutable account address unlock condition. */
    ImmutableAccountAddress = 6,
}

abstract class UnlockCondition {
    /**
     * The type of unlock condition.
     */
    readonly type: UnlockConditionType;

    /**
     * @param type The type of the unlock condition.
     */
    constructor(type: UnlockConditionType) {
        this.type = type;
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
        } else if (data.type == UnlockConditionType.ImmutableAccountAddress) {
            return plainToInstance(
                ImmutableAccountAddressUnlockCondition,
                data,
            ) as any as ImmutableAccountAddressUnlockCondition;
        }
        throw new Error('Invalid JSON');
    }
}

/**
 * An address unlock condition.
 */
class AddressUnlockCondition extends UnlockCondition {
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
}

/**
 * A Storage Deposit Return Unlock Condition.
 */
class StorageDepositReturnUnlockCondition extends UnlockCondition {
    // Getter transforms it into a proper number
    private amount: string;

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
     * The amount the consuming transaction must deposit to `returnAddress`.
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
 * Defines a slot index until which only Address, defined in Address Unlock Condition, is allowed to unlock the output.
 * After the slot index is reached/passed, only Return Address can unlock it.
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
     * Before this slot index, Address Unlock Condition is allowed to unlock the output, after that only the address defined in Return Address.
     */
    readonly slotIndex: SlotIndex;

    /**
     * @param returnAddress The address that can unlock the expired output.
     * @param slotIndex The slot index timestamp marking the end of the claim period.
     */
    constructor(returnAddress: Address, slotIndex: SlotIndex) {
        super(UnlockConditionType.Expiration);
        this.returnAddress = returnAddress;
        this.slotIndex = slotIndex;
    }
}

/**
 * A State Controller Address Unlock Condition.
 */
class StateControllerAddressUnlockCondition extends UnlockCondition {
    /**
     * The address.
     */
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
}

/**
 * A Governor Address Unlock Condition.
 */
class GovernorAddressUnlockCondition extends UnlockCondition {
    /**
     * The Governor address.
     */
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
}

/**
 * Immutable Account Unlock Condition.
 */
class ImmutableAccountAddressUnlockCondition extends UnlockCondition {
    /**
     * The Immutable Account address.
     */
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    readonly address: Address;

    /**
     * @param address The Immutable Account address that owns the output.
     */
    constructor(address: AccountAddress) {
        super(UnlockConditionType.ImmutableAccountAddress);
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
            value: ImmutableAccountAddressUnlockCondition,
            name: UnlockConditionType.ImmutableAccountAddress as any,
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
    ImmutableAccountAddressUnlockCondition,
};
