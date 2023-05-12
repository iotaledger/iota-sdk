// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types;

public class MintNftParams extends AbstractObject {
    /// Bech32 encoded address to which the Nft will be minted. Default will use the
    /// first address of the account
    private String address;
    /// Immutable nft metadata, hex encoded bytes
    private String immutableMetadata;
    /// Nft metadata, hex encoded bytes
    private String metadata;

    public String getAddress() {
        return address;
    }

    public MintNftParams withAddress(String address) {
        this.address = address;
        return this;
    }

    public String getImmutableMetadata() {
        return immutableMetadata;
    }

    public MintNftParams withImmutableMetadata(String immutableMetadata) {
        this.immutableMetadata = immutableMetadata;
        return this;
    }

    public String getMetadata() {
        return metadata;
    }

    public MintNftParams withMetadata(String metadata) {
        this.metadata = metadata;
        return this;
    }
}
