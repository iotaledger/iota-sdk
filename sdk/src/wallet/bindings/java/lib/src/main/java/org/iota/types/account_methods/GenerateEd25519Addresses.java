// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.account_methods;

import org.iota.types.AbstractObject;

public class GenerateEd25519Addresses extends AbstractObject implements AccountMethod {

    private int amount;
    private GenerateAddressOptions options;

    public GenerateEd25519Addresses withAmount(int amount) {
        this.amount = amount;
        return this;
    }

    public GenerateEd25519Addresses withGenerateAddressOptions(GenerateAddressOptions options) {
        this.options = options;
        return this;
    }
}
