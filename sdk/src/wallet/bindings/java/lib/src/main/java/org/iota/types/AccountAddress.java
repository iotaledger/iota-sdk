// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types;

public class AccountAddress extends AbstractObject {

    /// The address.
    private String bech32Address;
    /// The address key index.
    private int keyIndex;
    /// Determines if an address is a public or an internal (change) address.
    private boolean isInternal;
    // do we want this field? Could be useful if we don't store spent output ids and
    // because of that wouldn't know if
    // an address was used or not just by looking at it
    private boolean used;

    public AccountAddress withBech32Address(String bech32Address) {
        this.bech32Address = bech32Address;
        return this;
    }

    public AccountAddress withKeyIndex(int keyIndex) {
        this.keyIndex = keyIndex;
        return this;
    }

    public AccountAddress withInternal(boolean internal) {
        isInternal = internal;
        return this;
    }

    public AccountAddress withUsed(boolean used) {
        this.used = used;
        return this;
    }

    public String getBech32Address() {
        return bech32Address;
    }

    public int getKeyIndex() {
        return keyIndex;
    }

    public boolean isInternal() {
        return isInternal;
    }

    public boolean isUsed() {
        return used;
    }
}
