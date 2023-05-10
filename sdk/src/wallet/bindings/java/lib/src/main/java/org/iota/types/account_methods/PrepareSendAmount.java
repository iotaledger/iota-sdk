// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.account_methods;

import org.iota.types.SendAmountParams;
import org.iota.types.TransactionOptions;

/// Prepare send amount.
public class PrepareSendAmount implements AccountMethod {

    private SendAmountParams[] params;
    private TransactionOptions options;

    public PrepareSendAmount withParams(SendAmountParams[] params) {
        this.params = params;
        return this;
    }

    public PrepareSendAmount withOptions(TransactionOptions options) {
        this.options = options;
        return this;
    }
}
