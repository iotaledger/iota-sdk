// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.account_methods;

import org.iota.types.SendNftParams;
import org.iota.types.TransactionOptions;

/// Send nft.
public class SendNft implements AccountMethod {

    private SendNftParams[] params;
    private TransactionOptions options;

    public SendNft withParams(SendNftParams[] params) {
        this.params = params;
        return this;
    }

    public SendNft withOptions(TransactionOptions options) {
        this.options = options;
        return this;
    }
}
