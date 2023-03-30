// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { internalInitLogger } from './bindings';
import type { LoggerConfig } from '../types';

export * from './createAccountManager';
export * from './createMessageHandler';
export * from './createAccount';
export * from '../types';

/** Function to create wallet logs */
const initLogger = (config: LoggerConfig) =>
    internalInitLogger(JSON.stringify(config));

export { initLogger };
