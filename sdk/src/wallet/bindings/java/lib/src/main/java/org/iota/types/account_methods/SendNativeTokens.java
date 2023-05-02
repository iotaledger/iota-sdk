// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.account_methods;

import org.iota.types.AddressNativeTokens;
import org.iota.types.TransactionOptions;

/// Send native tokens.
public class SendNativeTokens implements AccountMethod {

    private AddressNativeTokens[] addressesAndNativeTokens;
    private TransactionOptions options;

    public SendNativeTokens withAddressesAndNativeTokens(AddressNativeTokens[] addressesAndNativeTokens) {
        this.addressesAndNativeTokens = addressesAndNativeTokens;
        return this;
    }

    public SendNativeTokens withOptions(TransactionOptions options) {
        this.options = options;
        return this;
    }
}
