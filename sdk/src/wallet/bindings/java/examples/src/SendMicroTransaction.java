// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import org.iota.Wallet;
import org.iota.types.*;
import org.iota.types.account_methods.SyncAccount;
import org.iota.types.exceptions.InitializeWalletException;
import org.iota.types.exceptions.WalletException;
import org.iota.types.ids.account.AccountAlias;
import org.iota.types.secret.StrongholdSecretManager;

public class SendMicroTransaction {
    public static void main(String[] args) throws WalletException, InterruptedException, InitializeWalletException {
        // This example assumes that a wallet has already been created using the
        // ´SetupWallet.java´ example.
        // If you haven't run the ´SetupWallet.java´ example yet, you must run it first
        // to be able to load the wallet as shown below:
        Wallet wallet = new Wallet(new WalletConfig()
                .withClientOptions(new ClientConfig().withNodes(Env.NODE))
                .withSecretManager(
                        new StrongholdSecretManager(Env.STRONGHOLD_PASSWORD, null, Env.STRONGHOLD_VAULT_PATH))
                .withCoinType(CoinType.Shimmer)
                .withStoragePath(Env.STORAGE_PATH));

        wallet.setStrongholdPassword(Env.STRONGHOLD_PASSWORD);

        // Get account and sync it with the registered node to ensure that its balances
        // are up-to-date.
        AccountHandle account = wallet.getAccount(Env.ACCOUNT_NAME);

        AddressWithAmount[] outputs = new AddressWithAmount[] {
                new AddressWithAmount()
                        .withAddress("rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu")
                                        .withAmount("1")
        };

        // Configure outputs
        Transaction transaction = account.sendAmount(
                new org.iota.types.account_methods.SendAmount().withAddressesWithAmount(outputs)
                        .withOptions(new TransactionOptions().withAllowMicroAmount(true)));

        // Print transaction
        System.out.println(
                "Transaction: " + transaction.getTransactionId() +
                        " Block sent: " + Env.NODE + "/api/core/v2/blocks/" +
                        transaction.getBlockId());

        // In case you are done and don't need the wallet instance anymore you can
        // destroy the instance to clean up memory.
        // For this, check out the ´DestroyWallet.java´ example.
    }

}
