// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota;

import org.iota.types.exceptions.ClientException;
import org.iota.types.exceptions.InitializeClientException;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.Range;
import org.iota.types.secret.MnemonicSecretManager;
import org.junit.jupiter.api.Test;
import org.iota.types.addresses.Ed25519Address;
import org.iota.types.signature.Ed25519Signature;
import org.iota.types.UnsignedByte;

public class SignAndVerifyEd25519 extends ApiTest {

    @Test
    public void testSignAndVerifyEd25519() throws ClientException, InitializeClientException {

        MnemonicSecretManager secretManager = new MnemonicSecretManager(client.generateMnemonic());

        // `IOTA` hex encoded
        String message = "0x494f5441";
        // [44, 4218, 0, 0, 0] IOTA coin type, first account, first public address
        Long[] chain = new Long[]{44l, 4218l, 0l, 0l, 0l};
        Ed25519Signature signature = client.signEd25519(secretManager, message, chain);

        String bech32Address = client.hexPublicKeyToBech32Address(signature.getPublicKey(), "atoi");
        String pubKeyHash = client.bech32ToHex(bech32Address);

        Boolean validSignature = client.verifyEd25519Signature(signature, message, new Ed25519Address(pubKeyHash));
        assert(validSignature);
    }
}
