// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import org.iota.types.Account;
import org.iota.types.OutputParams;
import org.iota.types.Transaction;
import org.iota.types.account_methods.PrepareOutput;
import org.iota.types.account_methods.SendOutputs;
import org.iota.types.account_methods.SyncAccount;
import org.iota.types.exceptions.WalletException;
import org.iota.types.outputs.Output;

import java.util.Date;

public class ExampleUtils {

    public static void fundAccount(Account a) throws WalletException, InterruptedException {
        while (a.syncAccount(new SyncAccount()).getBaseCoin().getAvailable() < 1000000) {
            System.out.println("Please fund following address: " + a.getPublicAddresses()[0]);
            Thread.sleep(5000);
        }
    }

    public static void setUpOutputToClaim(Account a) throws WalletException, InterruptedException {
        Transaction t = a.sendOutputs(new SendOutputs().withOutputs(new Output[] {
                a.prepareOutput(new PrepareOutput().withParams(new OutputParams()
                        .withAmount("1000000")
                        .withRecipientAddress(a.getPublicAddresses()[0].getAddress())
                        .withUnlocks(new OutputParams.Unlocks()
                                .withExpirationUnixTime(Math.round(new Date().getTime() / 1000) + 1))))
        }));
        Thread.sleep(5);
        System.out.println(t);
    }

}
