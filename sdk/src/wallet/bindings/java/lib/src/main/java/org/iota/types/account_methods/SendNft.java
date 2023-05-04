// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.account_methods;

import org.iota.types.NftParams;
import org.iota.types.TransactionOptions;

/// Send nft.
public class SendNft implements AccountMethod {

    private NftParams[] params;
    private TransactionOptions options;

    public SendNft withParams(NftParams[] params) {
        this.params = params;
        return this;
    }

    public SendNft withOptions(TransactionOptions options) {
        this.options = options;
        return this;
    }
}
