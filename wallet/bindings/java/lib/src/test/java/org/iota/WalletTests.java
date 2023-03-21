// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
package org.iota;

import org.iota.types.AccountAddress;
import org.iota.types.AccountHandle;
import org.iota.types.addresses.Address;
import org.iota.types.account_methods.GenerateAddresses;
import org.iota.types.account_methods.AddressGenerationOptions;
import org.iota.types.account_methods.SetAlias;
import org.iota.types.exceptions.WalletException;
import org.iota.types.ids.account.AccountAlias;
import org.iota.types.ids.account.AccountIndex;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;
public class WalletTests extends TestSettings {
    @Test
    public void testCreateAccount() throws WalletException {
        System.out.println(wallet.createAccount("Alice"));
    }

    @Test
    public void testCreateAccountsWithSameAlias() throws WalletException {
        System.out.println(wallet.createAccount("Alice"));
        try {
            System.out.println(wallet.createAccount("Alice"));
        } catch (WalletException expectedException) { ; }
    }

    @Test
    public void testGetAccountByAlias() throws WalletException {
        AccountHandle a = wallet.createAccount("Alice");
        AccountHandle b = wallet.getAccount(new AccountAlias("Alice"));
        assertEquals(a,b);
    }

    @Test
    public void testGetAccountByIndex() throws WalletException {
        AccountHandle a = wallet.createAccount("Alice");
        AccountHandle b = wallet.getAccount(new AccountIndex(0));
        assertEquals(a,b);
    }

    @Test
    public void testGetAccounts() throws WalletException {
        AccountHandle a = wallet.createAccount("Alice");
        AccountHandle b = wallet.createAccount("Bob");
        assertTrue(wallet.getAccounts().length == 2);
        for (AccountHandle x : wallet.getAccounts())
            System.out.println(x);
    }

    @Test
    public void testGenerateAddress() throws WalletException {
        AddressGenerationOptions.GenerateAddressOptions options = new AddressGenerationOptions.GenerateAddressOptions().withLedgerNanoPrompt(false);
        AddressGenerationOptions addressOptions = new AddressGenerationOptions().withOptions(options);

        String address = wallet.generateAddress(0, false, 0, options, "rms");
        assertEquals("rms1qpx0mcrqq7t6up73n4na0zgsuuy4p0767ut0qq67ngctj7pg4tm2ynsuynp", address);

        // generated account at line 77 has first 2 pre-made, so we check against the third
        address = wallet.generateAddress(0, false, 2, options, "rms");
        assertEquals("rms1qzjq2jwzp8ddh0gawgdskvtd6awlv82c8y0a9s6g7kgszn6ts95u6r4kx2n", address);

        String addressPublic = wallet.generateAddress(0, true, 0, options, "rms");
        assertEquals("rms1qqtjgttzh2dp5exzru94pddle5sqf0007q4smdsaycerff2hny5764xrkgk", addressPublic);

        String anotherAddress = wallet.generateAddress(10, true, 10, options, "rms");
        assertEquals("rms1qzu4a5ryj39h07z9atn2fza59wu2n5f295st5ehmjg5u8tyveaew65lw3yg", anotherAddress);

        AccountHandle account = wallet.createAccount("Alice");
        AccountAddress[] addresses = account.generateAddresses(new GenerateAddresses().withAmount(1).withAddressGenerationOptions(addressOptions));

        assertEquals(1, addresses.length);
        assertEquals("rms1qz8jdgvrerzv35s43pkdkawdr9x4t6xfnhcrt5tlgsyltgpwyx9ks4c5kct", addresses[0].getAddress());

        account = wallet.getAccounts()[0];
        addresses = account.generateAddresses(new GenerateAddresses().withAmount(1).withAddressGenerationOptions(addressOptions));
        assertEquals(address, addresses[0].getAddress());
    }

}
