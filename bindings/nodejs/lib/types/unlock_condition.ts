// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    ADDRESS_UNLOCK_CONDITION_TYPE,
    EXPIRATION_UNLOCK_CONDITION_TYPE,
    GOVERNOR_ADDRESS_UNLOCK_CONDITION_TYPE,
    IMMUTABLE_ALIAS_UNLOCK_CONDITION_TYPE,
    STATE_CONTROLLER_ADDRESS_UNLOCK_CONDITION_TYPE,
    STORAGE_DEPOSIT_RETURN_UNLOCK_CONDITION_TYPE,
    TIMELOCK_UNLOCK_CONDITION_TYPE,
} from '@iota/types';
import { Address, AliasAddress } from './address';
/**
 * All of the unlock condition types.
 */
enum UnlockConditionType {
    Address = ADDRESS_UNLOCK_CONDITION_TYPE,
    StorageDepositReturn = STORAGE_DEPOSIT_RETURN_UNLOCK_CONDITION_TYPE,
    Timelock = TIMELOCK_UNLOCK_CONDITION_TYPE,
    Expiration = EXPIRATION_UNLOCK_CONDITION_TYPE,
    StateControllerAddress = STATE_CONTROLLER_ADDRESS_UNLOCK_CONDITION_TYPE,
    GovernorAddress = GOVERNOR_ADDRESS_UNLOCK_CONDITION_TYPE,
    ImmutableAliasAddress = IMMUTABLE_ALIAS_UNLOCK_CONDITION_TYPE,
}

abstract class UnlockCondition {
    private _type: UnlockConditionType;

    constructor(type: UnlockConditionType) {
        this._type = type;
    }
    /**
     * The type of unlock condition.
     */
    get type(): UnlockConditionType {
        return this._type;
    }
}

class AddressUnlockCondition extends UnlockCondition /*implements IAddressUnlockCondition*/ {
    private _address: Address;
    constructor(address: Address) {
        super(UnlockConditionType.Address);
        this._address = address;
    }

    /**
     * The address.
     */
    get address(): Address {
        return this._address;
    }
}
/**
 * Storage Desposit Return Unlock Condition.
 */
class StorageDepositReturnUnlockCondition extends UnlockCondition /*implements IStorageDepositReturnUnlockCondition*/ {
    private _amount: number;
    private _returnAddress: Address;

    constructor(returnAddress: Address, amount: number) {
        super(UnlockConditionType.StorageDepositReturn);
        this._amount = amount;
        this._returnAddress = returnAddress;
    }
    /**
     * Amount of IOTA tokens the consuming transaction should deposit to the address defined in return address.
     */
    get amount(): number {
        return this._amount;
    }

    /**
     * The return address.
     */
    get returnAddress(): Address {
        return this._returnAddress;
    }
}
/**
 * Timelock Unlock Condition.
 */
class TimelockUnlockCondition extends UnlockCondition /*implements ITimelockUnlockCondition*/ {
    private _unixTime: number;

    constructor(unixTime: number) {
        super(UnlockConditionType.Timelock);
        this._unixTime = unixTime;
    }
    /**
     * Unix time (seconds since Unix epoch) starting from which the output can be consumed.
     */
    get unixTime(): number {
        return this._unixTime;
    }
}

class ExpirationUnlockCondition extends UnlockCondition /*implements IExpirationUnlockCondition*/ {
    private _returnAddress: Address;
    private _unixTime: number;

    constructor(returnAddress: Address, unixTime: number) {
        super(UnlockConditionType.Expiration);
        this._returnAddress = returnAddress;
        this._unixTime = unixTime;
    }
    /**
     * Unix time (seconds since Unix epoch) starting from which the output can be consumed.
     */
    get unixTime(): number {
        return this._unixTime;
    }

    /**
     * The return address.
     */
    get returnAddress(): Address {
        return this._returnAddress;
    }
}
/**
 * State Controller Address Unlock Condition.
 */
class StateControllerAddressUnlockCondition extends UnlockCondition /*implements IStateControllerAddressUnlockCondition*/ {
    private _address: Address;
    constructor(address: Address) {
        super(UnlockConditionType.StateControllerAddress);
        this._address = address;
    }

    /**
     * The address.
     */
    get address(): Address {
        return this._address;
    }
}
/**
 * Governor Unlock Condition.
 */
class GovernorAddressUnlockCondition extends UnlockCondition /*implements IGovernorAddressUnlockCondition*/ {
    private _address: Address;
    constructor(address: Address) {
        super(UnlockConditionType.GovernorAddress);
        this._address = address;
    }

    /**
     * The address.
     */
    get address(): Address {
        return this._address;
    }
}
/**
 * Immutable Alias Unlock Condition.
 */
class ImmutableAliasAddressUnlockCondition extends UnlockCondition /*implements IImmutableAliasAddressUnlockCondition*/ {
    private _address: Address;
    constructor(address: AliasAddress) {
        super(UnlockConditionType.ImmutableAliasAddress);
        this._address = address;
    }

    /**
     * The address.
     */
    get address(): Address {
        return this._address;
    }
}

export {
    UnlockCondition,
    AddressUnlockCondition,
    StorageDepositReturnUnlockCondition,
    TimelockUnlockCondition,
    ExpirationUnlockCondition,
    StateControllerAddressUnlockCondition,
    GovernorAddressUnlockCondition,
    ImmutableAliasAddressUnlockCondition,
};
