// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.account_methods;

import org.iota.types.MintNativeTokenParams;
import org.iota.types.TransactionOptions;

/// Mint native token.
public class MintNativeToken implements AccountMethod {

    private MintNativeTokenParams params;
    private TransactionOptions options;

    public MintNativeToken withParams(MintNativeTokenParams params) {
        this.params = params;
        return this;
    }

    public MintNativeToken withOptions(TransactionOptions options) {
        this.options = options;
        return this;
    }
}
