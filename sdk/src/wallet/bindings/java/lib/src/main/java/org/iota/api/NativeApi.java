// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.api;

import com.google.gson.Gson;
import com.google.gson.JsonElement;
import org.apache.commons.lang3.SystemUtils;
import org.iota.types.WalletConfig;
import org.iota.types.exceptions.InitializeWalletException;
import org.iota.types.exceptions.WalletException;
import org.iota.types.events.Event;
import org.iota.types.events.EventListener;
import org.iota.types.events.wallet.WalletEventType;

public class NativeApi {

    static {

        Throwable loadFromJavaPathThrowable = null;
        Throwable loadFromJarThrowable = null;

        try {
            loadFromJavaPath();
        } catch (Throwable t) {
            loadFromJavaPathThrowable = t;
        }

        if (loadFromJavaPathThrowable != null) {
            try {
                loadFromJar();
            } catch (Throwable t) {
                loadFromJarThrowable = t;
            }
        }

        if (loadFromJavaPathThrowable != null && loadFromJarThrowable != null) {
            loadFromJavaPathThrowable.printStackTrace();
            loadFromJarThrowable.printStackTrace();
            throw new RuntimeException("cannot load native library");
        }

    }

    private static void loadFromJavaPath() {
        System.loadLibrary("iota_wallet");
    }

    private static void loadFromJar() throws Throwable {
        String libraryName;

        if (SystemUtils.IS_OS_LINUX)
            libraryName = "libiota_wallet.so";
        else if (SystemUtils.IS_OS_MAC)
            libraryName = "libiota_wallet.dylib";
        else if (SystemUtils.IS_OS_WINDOWS)
            libraryName = "iota_wallet.dll";
        else
            throw new RuntimeException("OS not supported");

        NativeUtils.loadLibraryFromJar("/" + libraryName);
    }

    protected NativeApi(WalletConfig walletConfig) throws InitializeWalletException {
        try {
            // Must use a new Gson instance to not serialize null values.
            // CustomGson.get() would serialize null values and doesn't work here
            createMessageHandler(new Gson().toJsonTree(walletConfig).toString());
        } catch (Exception e) {
            throw new InitializeWalletException(e.getMessage());
        }
    }

    protected static native void initLogger(String config);

    private static native void createMessageHandler(String config) throws Exception;

    // Destroys account handle
    // For Firefly mobile, we sent clearListeners event by sendMessage
    // so we need to call destroyHandle manually from Capacitor binding plugin.
    public static native void destroyHandle();

    private static native String sendMessage(String command);

    private static native String listen(Integer[] events, EventListener listener);

    public static native String migrateStrongholdSnapshotV2ToV3(String currentPath, String currentPassword, String salt, int rounds, String newPath, String newPassword);

    private static JsonElement handleClientResponse(String methodName, String jsonResponse) throws WalletException {
        WalletResponse response = CustomGson.get().fromJson(jsonResponse, WalletResponse.class);

        switch (response.type) {
            case "panic":
                throw new RuntimeException(response.toString());
            case "error":
                throw new WalletException(methodName, response.payload.getAsJsonObject().toString());

            default:
                return response.payload;
        }
    }

    private static void handleCallback(String response, EventListener listener) throws WalletException {
        try {
            Event event = CustomGson.get().fromJson(response, Event.class);
            listener.receive(event);
        } catch (Exception e) {
            throw new WalletException("handleCallback", e.getMessage());
        }
    }

    public static void callListen(EventListener listener, WalletEventType... events) throws WalletException {
        Integer[] eventIds = new Integer[events.length];
        for (int i = 0; i < events.length; i++) {
            eventIds[i] = events[i].getValue();
        }

        // Check for errors, no interest in result
        handleClientResponse("listen", listen(eventIds, listener));
    }

    public static JsonElement callBaseApi(WalletCommand command) throws WalletException {
        // System.out.println("REQUEST: " + command);
        String jsonResponse = sendMessage(command.toString());
        // System.out.println("RESPONSE: " + jsonResponse);
        return handleClientResponse(command.getMethodName(), jsonResponse);
    }

    private class WalletResponse {
        String type;
        JsonElement payload;
    }

}