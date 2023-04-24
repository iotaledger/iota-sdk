// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.account_methods;
import org.iota.types.AbstractObject;

public class GenerateAddressOptions extends AbstractObject {
    private boolean internal;
    private boolean ledgerNanoPrompt;

    public GenerateAddressOptions withInternal(boolean internal) {
        this.internal = internal;
        return this;
    }

    public GenerateAddressOptions withLedgerNanoPrompt(boolean ledgerNanoPrompt) {
        this.ledgerNanoPrompt = ledgerNanoPrompt;
        return this;
    }
}
