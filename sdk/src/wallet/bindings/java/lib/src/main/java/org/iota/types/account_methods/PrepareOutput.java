// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.account_methods;

import org.iota.types.OutputParams;
import org.iota.types.TransactionOptions;

/// Prepare an output.
public class PrepareOutput implements AccountMethod {

    private OutputParams params;
    private TransactionOptions transactionOptions;

    public PrepareOutput withParams(OutputParams params) {
        this.params = params;
        return this;
    }

    public PrepareOutput withTransactionOptions(TransactionOptions transactionOptions) {
        this.transactionOptions = transactionOptions;
        return this;
    }
}
