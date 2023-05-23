// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types;

public class MintNativeTokenParams extends AbstractObject {
    /// Bech32 encoded address. Needs to be an account address. Default will use the
    /// first address of the account
    private String accountAddress;
    /// Circulating supply
    private String circulatingSupply;
    /// Maximum supply
    private String maximumSupply;
    /// Foundry metadata, hex encoded bytes
    private String foundryMetadata;

    public String getAccountAddress() {
        return accountAddress;
    }

    public MintNativeTokenParams withAccountAddress(String accountAddress) {
        this.accountAddress = accountAddress;
        return this;
    }

    public String getCirculatingSupply() {
        return circulatingSupply;
    }

    public MintNativeTokenParams withCirculatingSupply(String circulatingSupply) {
        this.circulatingSupply = circulatingSupply;
        return this;
    }

    public String getMaximumSupply() {
        return maximumSupply;
    }

    public MintNativeTokenParams withMaximumSupply(String maximumSupply) {
        this.maximumSupply = maximumSupply;
        return this;
    }

    public String getFoundryMetadata() {
        return foundryMetadata;
    }

    public MintNativeTokenParams withFoundryMetadata(String foundryMetadata) {
        this.foundryMetadata = foundryMetadata;
        return this;
    }
}
