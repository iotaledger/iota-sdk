// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types;

import org.iota.types.ids.OutputId;

public class AddressWithUnspentOutputs extends AbstractObject {

    /// The address.
    private String bech32Address;
    /// The address key index.
    private int keyIndex;
    /// Determines if an address is a public or an internal (change) address.
    private boolean internal;
    /// Output ids.
    private OutputId[] output_ids;

    public String getBech32Address() {
        return bech32Address;
    }

    public int getKeyIndex() {
        return keyIndex;
    }

    public boolean isInternal() {
        return internal;
    }

    public OutputId[] getOutput_ids() {
        return output_ids;
    }
}
