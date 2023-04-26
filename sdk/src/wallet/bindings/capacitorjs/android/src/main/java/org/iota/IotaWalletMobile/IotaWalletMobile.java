// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.IotaWalletMobile;

import android.util.Log;

import com.getcapacitor.JSArray;
import com.getcapacitor.JSObject;
import com.getcapacitor.Plugin;
import com.getcapacitor.PluginCall;
import com.getcapacitor.PluginMethod;
import com.getcapacitor.annotation.CapacitorPlugin;

import java.util.Arrays;
import java.util.Objects;

import org.iota.Wallet;
import org.iota.api.WalletCommand;
import org.iota.types.ClientConfig;
import org.iota.types.CoinType;
import org.iota.types.WalletConfig;
import org.iota.types.events.Event;
import org.iota.types.events.EventListener;
import org.iota.types.events.wallet.WalletEventType;
import org.iota.types.exceptions.WalletException;
import org.iota.types.secret.StrongholdSecretManager;
import org.json.JSONArray;
import org.json.JSONException;
import org.json.JSONObject;

import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;

import static org.iota.api.NativeApi.callBaseApi;
import static org.iota.api.NativeApi.destroyHandle;


@CapacitorPlugin(name = "IotaWalletMobile")
public class IotaWalletMobile extends Plugin {

    Wallet wallet = null;

    @PluginMethod()
    public void messageHandlerNew(final PluginCall call) throws JSONException {

        if (!call.getData().has("clientOptions")
                || !call.getData().has("storagePath")
                || !call.getData().has("coinType")
                || !call.getData().has("secretManager")) {
            call.reject("clientOptions, storagePath, coinType and secretManager are required");
        }
        JSObject _clientOptions = call.getObject("clientOptions");
        String _storagePath = call.getString("storagePath");
        Integer _coinType = call.getInt("coinType");
        JSObject _secretManager = call.getObject("secretManager");
        String path = getContext().getFilesDir() + _storagePath;
        Log.d("_clientOptions", _clientOptions.get("nodes").toString());


        JSONArray _nodes = _clientOptions.getJSONArray("nodes");
        String[] nodes = new String[_nodes.length()];
        for (int i = 0; i < _nodes.length(); i++) {
            JSONObject _node = (JSONObject) _nodes.get(i);
            nodes[i] = _node.get("url").toString();
        }
        
        JSONObject secretManager = _secretManager.getJSONObject("stronghold");
        Log.d("_storagePath", path + secretManager.get("snapshotPath"));
        if (_coinType == null) {
            return;
        }
        CoinType coinType = CoinType.Shimmer;
        if (CoinType.Iota.getCoinTypeValue() == _coinType) {
            coinType = CoinType.Iota;
        }

        try {
            wallet = new Wallet(new WalletConfig()
                    .withClientOptions(new ClientConfig().withNodes(nodes))
                    .withStoragePath(path)
                    .withSecretManager(
                            new StrongholdSecretManager(
                                    null,
                                    null,
                                    path + "/wallet.stronghold"
                            )
                    )
                    .withCoinType(coinType)
            );
            JSObject ret = new JSObject();
            // 1 signals the id of the messageHandler returned by the rust side. 
            // This is irrelevant for the Java side, but required on the Swift and JS side
            Integer messageHandlerPointer = 1;
            ret.put("messageHandler", messageHandlerPointer);
            call.resolve(ret);
        } catch (Exception ex) {
            call.reject(ex.getMessage() + Arrays.toString(ex.getStackTrace()));
        }
    }

    @PluginMethod()
    public void sendMessage(final PluginCall call) {
        try {
            if (!call.getData().has("message")) {
                call.reject("message is required");
            }
            String message = call.getString("message");
            if (message == null) {
                return;
            }

            JsonElement element = JsonParser.parseString(message);
            JsonObject jsonObject = element.getAsJsonObject();
            WalletCommand walletCommand;
            if (jsonObject.has("payload") && jsonObject.has("cmd")) {
                        walletCommand = new WalletCommand(
                                jsonObject.get("cmd").getAsString(),
                                jsonObject.get("payload")
                        );
            }
            else {
                walletCommand = new WalletCommand(jsonObject.get("cmd").getAsString());

            }
            JsonElement jsonResponse = callBaseApi(walletCommand);
            JSObject ret = new JSObject();
            if (jsonResponse != null) {
                JsonObject clientResponse = new JsonObject();
                clientResponse.addProperty("type", jsonObject.get("cmd").getAsString());
                clientResponse.add("payload", jsonResponse);
                ret.put("result", clientResponse.toString());
            } else {
                ret.put("result", "ok");
            }
            call.resolve(ret);
        } catch (Exception ex) {
            String message = Objects.requireNonNull(ex.getMessage());
            JsonElement element = JsonParser.parseString(message);
            JsonObject jsonObject = element.getAsJsonObject();
            JSObject ret = new JSObject();
            JsonObject clientResponse = new JsonObject();
            clientResponse.addProperty("type", "error");
            clientResponse.add("payload", jsonObject);
            ret.put("result", clientResponse.toString());
            call.resolve(ret);
        }
    }

    @PluginMethod(returnType = PluginMethod.RETURN_CALLBACK)
    public void listen(final PluginCall call) throws WalletException, JSONException {
        if (!call.getData().has("eventTypes")) {
            call.reject("eventTypes are required");
        }

        JSArray eventTypes = call.getArray("eventTypes");
        WalletEventType[] types = new WalletEventType[eventTypes.length()];
        for (int i = 0; i < eventTypes.length(); i++) {
            types[i] = WalletEventType.valueOf(eventTypes.getString(i));
        }

        try {
            wallet.listen(new EventListener() {
                @Override
                public void receive(Event event) {
                    JSObject walletResponse = new JSObject();
                    walletResponse.put("result", event.toString());
                    call.resolve(walletResponse);
                }
            }, types);
        } catch (WalletException ex) {
            call.reject(ex.getMessage() + Arrays.toString(ex.getStackTrace()));
        }
        call.setKeepAlive(true);
    }


    @PluginMethod()
    public void destroy(final PluginCall call) {
        try {
            destroyHandle();
            call.release(bridge);
        } catch (Exception ex) {
            call.reject(ex.getMessage() + Arrays.toString(ex.getStackTrace()));
        }
    }

}
