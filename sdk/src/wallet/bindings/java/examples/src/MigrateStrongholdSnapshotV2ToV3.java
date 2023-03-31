// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import org.iota.Wallet;
import org.iota.types.*;
import org.iota.types.account_methods.GenerateAddresses;
import org.iota.types.exceptions.InitializeWalletException;
import org.iota.types.exceptions.WalletException;
import org.iota.types.secret.StrongholdSecretManager;
import org.iota.api.NativeApi;
import org.iota.types.ids.account.AccountAlias;

public class MigrateStrongholdSnapshotV2ToV3 {
    public static void main(String[] args) throws WalletException, InitializeWalletException {
        try {
                Wallet wallet = new Wallet(new WalletConfig()
                    .withClientOptions(new ClientConfig().withNodes(Env.NODE))
                    .withSecretManager(new StrongholdSecretManager("current_password", null, "src/res/v2.stronghold"))
                    .withCoinType(CoinType.Shimmer)
                    .withStoragePath(Env.STORAGE_PATH));
        } catch (Exception e) {
            System.out.println(e);
        }

        NativeApi.migrateStrongholdSnapshotV2ToV3("src/res/v2.stronghold", "current_password", "src/res/v3.stronghold", "new_password");

        Wallet wallet = new Wallet(new WalletConfig()
            .withClientOptions(new ClientConfig().withNodes(Env.NODE))
            .withSecretManager(new StrongholdSecretManager("new_password", null, "src/res/v3.stronghold"))
            .withCoinType(CoinType.Shimmer)
            .withStoragePath(Env.STORAGE_PATH));

        AccountHandle account = wallet.getAccount(new AccountAlias(Env.ACCOUNT_NAME));

        AccountAddress[] addresses = account.generateAddresses(new GenerateAddresses().withAmount(1));

        System.out.println(addresses[0].getAddress());

    }
}