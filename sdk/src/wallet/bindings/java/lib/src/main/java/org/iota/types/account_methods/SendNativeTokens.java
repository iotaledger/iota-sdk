// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.account_methods;

import org.iota.types.SendNativeTokensParams;
import org.iota.types.TransactionOptions;

/// Send native tokens.
public class SendNativeTokens implements AccountMethod {

    private SendNativeTokensParams[] params;
    private TransactionOptions options;

    public SendNativeTokens withParams(SendNativeTokensParams[] params) {
        this.params = params;
        return this;
    }

    public SendNativeTokens withOptions(TransactionOptions options) {
        this.options = options;
        return this;
    }
}
