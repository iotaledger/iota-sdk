// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.account_methods;

import org.iota.types.MintNftParams;
import org.iota.types.TransactionOptions;

/// Mint nft.
public class MintNfts implements AccountMethod {

    private MintNftParams[] params;
    private TransactionOptions options;

    public MintNfts withParams(MintNftParams[] params) {
        this.params = params;
        return this;
    }

    public MintNfts withOptions(TransactionOptions options) {
        this.options = options;
        return this;
    }
}
