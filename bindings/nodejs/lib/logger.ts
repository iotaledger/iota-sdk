// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { LoggerConfig } from './types/logger-config';
import { initLogger as initLoggerBinding } from './bindings';

const defaultLoggerConfig: LoggerConfig = {
    colorEnabled: true,
    name: './iota-sdk.log',
    levelFilter: 'debug',
};

/** Initialize logger, if no arguments are provided a default config will be used. */
export const initLogger = (config: LoggerConfig = defaultLoggerConfig) =>
    initLoggerBinding(JSON.stringify(config));
