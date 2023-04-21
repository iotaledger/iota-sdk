// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.account_methods;
import org.iota.types.AbstractObject;

public class GenerateAddresses extends AbstractObject implements AccountMethod {

    private int amount;
    private GenerateAddressOptions options;

    public GenerateAddresses withAmount(int amount) {
        this.amount = amount;
        return this;
    }

    public GenerateAddresses withGenerateAddressOptions(GenerateAddressOptions options) {
        this.options = options;
        return this;
    }
}
