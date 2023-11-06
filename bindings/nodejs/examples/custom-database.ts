// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { implCustomDatabase, testCustomDatabase } from '@iota/sdk';

// Run with command:
// yarn run-example ./custom-database.ts

// In this example we will test a custom db
async function run() {
    try {
        let db = new Map<string, string>();

        const get_cb = async function (err: any, key: string) {
            console.log('JS side: get: ', key);
            db.get(key);
        };
        const set_cb = async function (err: any, key: string, value: string) {
            console.log('JS side: set', key, value);
            db.set(key, value);
        };
        const delete_cb = async function (err: any, key: string) {
            console.log('JS side: delete', key);
            db.delete(key);
        };

        let js_db = implCustomDatabase(get_cb, set_cb, delete_cb);

        await testCustomDatabase(js_db);
        console.log('Db in JS:', Object.fromEntries(db));
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
