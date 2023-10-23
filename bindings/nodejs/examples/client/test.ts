// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Address, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./client/test.ts

// In this example we will get output from a known outputId.
async function run() {
    initLogger();
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }

    const json = "{\"type\":24,\"pubKeyHash\":\"0x0000000000000000000000000000000000000000000000000000000000000000\"}"
    const addr = Address.parse(JSON.parse(json));
    console.log(addr);

    const json_str = JSON.stringify(addr);
    console.log(json_str)
    if (json == json_str) {
        console.log("EQUAL")
    }
}

run().then(() => process.exit());
