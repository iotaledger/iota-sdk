// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

export * from './account';
export * from './wallet';
export * from './wallet-method-handler';
export * from '../types/wallet';
import { migrateDbChrysalisToStardust } from '../bindings';
export { migrateDbChrysalisToStardust };
