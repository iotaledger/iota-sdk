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
    public static final String v2Path = "../../../../../tests/wallet/fixtures/v2.stronghold";
    public static final String v3Path = "./v3.stronghold";

    public static void main(String[] args) throws WalletException, InitializeWalletException {
        try {
                // This should fail with error, migration required.
                Wallet wallet = new Wallet(new WalletConfig()
                    .withClientOptions(new ClientConfig().withNodes(Env.NODE))
                    .withSecretManager(new StrongholdSecretManager("current_password", null, v2Path))
                    .withCoinType(CoinType.Shimmer)
                    .withStoragePath(Env.STORAGE_PATH));
        } catch (Exception e) {
            System.out.println(e);
        }

        NativeApi.migrateStrongholdSnapshotV2ToV3(v2Path, "current_password", "wallet.rs", 100, v3Path, "new_password");

        // This shouldn't fail anymore as snapshot has been migrated.
        Wallet wallet = new Wallet(new WalletConfig()
            .withClientOptions(new ClientConfig().withNodes(Env.NODE))
            .withSecretManager(new StrongholdSecretManager("new_password", null, v3Path))
            .withCoinType(CoinType.Shimmer)
            .withStoragePath(Env.STORAGE_PATH));

        Account account = wallet.createAccount(Env.ACCOUNT_NAME);

        AccountAddress[] addresses = account.generateAddresses(new GenerateAddresses().withAmount(1));

        System.out.println(addresses[0].getAddress());

    }
}