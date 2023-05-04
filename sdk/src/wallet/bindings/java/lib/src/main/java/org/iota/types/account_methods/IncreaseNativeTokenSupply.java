// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.account_methods;

import org.iota.types.TransactionOptions;
import org.iota.types.ids.TokenId;

/// Mint more native token.
public class IncreaseNativeTokenSupply implements AccountMethod {

    private TokenId tokenId;
    private String mintAmount;
    private TransactionOptions transactionOptions;

    public IncreaseNativeTokenSupply withTokenId(TokenId tokenId) {
        this.tokenId = tokenId;
        return this;
    }

    public IncreaseNativeTokenSupply withMintAmount(String mintAmount) {
        this.mintAmount = mintAmount;
        return this;
    }

    public IncreaseNativeTokenSupply withTransactionOptions(TransactionOptions transactionOptions) {
        this.transactionOptions = transactionOptions;
        return this;
    }
}
