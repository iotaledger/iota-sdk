// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Address, AliasAddress } from './address';

enum UnlockConditionType {
    Address = 0,
    StorageDepositReturn = 1,
    Timelock = 2,
    Expiration = 3,
    StateControllerAddress = 4,
    GovernorAddress = 5,
    ImmutableAliasAddress = 6,
}

class UnlockCondition {
    private _type: UnlockConditionType;

    constructor(type: UnlockConditionType) {
        this._type = type;
    }

    get type(): UnlockConditionType {
        return this._type;
    }
}

class AddressUnlockCondition extends UnlockCondition {
    private _address: Address;
    constructor(address: Address) {
        super(UnlockConditionType.Address);
        this._address = address;
    }

    get address(): Address {
        return this._address;
    }
}

class StorageDepositReturnUnlockCondition extends UnlockCondition {
    private _amount: number;
    private _returnAddress: Address;

    constructor(amount: number, returnAddress: Address) {
        super(UnlockConditionType.StorageDepositReturn);
        this._amount = amount;
        this._returnAddress = returnAddress;
    }

    get amount(): number {
        return this._amount;
    }

    get returnAddress(): Address {
        return this._returnAddress;
    }
}

class TimelockUnlockCondition extends UnlockCondition {
    private _unixTime: number;

    constructor(unixTime: number) {
        super(UnlockConditionType.Timelock);
        this._unixTime = unixTime;
    }

    get unixTime(): number {
        return this._unixTime;
    }
}

class ExpirationUnlockCondition extends UnlockCondition {
    private _returnAddress: Address;
    private _unixTime: number;

    constructor(unixTime: number, returnAddress: Address) {
        super(UnlockConditionType.Expiration);
        this._returnAddress = returnAddress;
        this._unixTime = unixTime;
    }

    get unixTime(): number {
        return this._unixTime;
    }

    get returnAddress(): Address {
        return this._returnAddress;
    }
}

class StateControllerAddressUnlockCondition extends UnlockCondition {
    private _address: Address;
    constructor(address: Address) {
        super(UnlockConditionType.StateControllerAddress);
        this._address = address;
    }

    get address(): Address {
        return this._address;
    }
}

class GovernorAddressUnlockCondition extends UnlockCondition {
    private _address: Address;
    constructor(address: Address) {
        super(UnlockConditionType.GovernorAddress);
        this._address = address;
    }

    get address(): Address {
        return this._address;
    }
}

class ImmutableAliasAddressUnlockCondition extends UnlockCondition {
    private _address: Address;
    constructor(address: AliasAddress) {
        super(UnlockConditionType.ImmutableAliasAddress);
        this._address = address;
    }

    get address(): Address {
        return this._address;
    }
}

export { UnlockCondition, AddressUnlockCondition, StorageDepositReturnUnlockCondition, TimelockUnlockCondition, ExpirationUnlockCondition, StateControllerAddressUnlockCondition, GovernorAddressUnlockCondition, ImmutableAliasAddressUnlockCondition };
