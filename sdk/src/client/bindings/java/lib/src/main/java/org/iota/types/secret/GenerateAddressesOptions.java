// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types.secret;

import com.google.gson.JsonObject;

public class GenerateAddressesOptions {
    private Integer coinType;
    private Integer accountIndex;
    private Range range;
    private String bech32Hrp;
    private GenerateAddressOptions options;

    public GenerateAddressesOptions withCoinType(Integer coinType) {
        this.coinType = coinType;
        return this;
    }

    public GenerateAddressesOptions withAccountIndex(Integer accountIndex) {
        this.accountIndex = accountIndex;
        return this;
    }

    public GenerateAddressesOptions withRange(Range range) {
        this.range = range;
        return this;
    }

    public GenerateAddressesOptions withBech32Hrp(String bech32Hrp) {
        this.bech32Hrp = bech32Hrp;
        return this;
    }

    public GenerateAddressesOptions withOptions(GenerateAddressOptions options) {
        this.options = options;
        return this;
    }

    public JsonObject getJson() {
        JsonObject o = new JsonObject();
        if (coinType != null)
            o.addProperty("coinType", coinType);
        if (accountIndex != null)
            o.addProperty("accountIndex", accountIndex);
        if (range != null)
            o.add("range", range.getAsJson());
        if (bech32Hrp != null)
            o.addProperty("bech32Hrp", bech32Hrp);
        if (options != null)
            o.add("options", options.getAsJson());

        return o;
    }

    public static class GenerateAddressOptions {
        private boolean internal;
        private boolean ledgerNanoPrompt;

        public GenerateAddressOptions withInternal(boolean internal) {
            this.internal = internal;
            return this;
        }

        public GenerateAddressOptions withLedgerNanoPrompt(boolean ledgerNanoPrompt) {
            this.ledgerNanoPrompt = ledgerNanoPrompt;
            return this;
        }

        public JsonObject getAsJson() {
            JsonObject o = new JsonObject();
            o.addProperty("internal", internal);
            o.addProperty("ledgerNanoPrompt", ledgerNanoPrompt);

            return o;
        }
    }

}
