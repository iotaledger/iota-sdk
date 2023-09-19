// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { ILoggerConfig } from './internal';
import { initLoggerBinding } from './internal';

const defaultLoggerConfig: ILoggerConfig = {
    colorEnabled: true,
    name: './iota-sdk.log',
    levelFilter: 'debug',
};

/** Initialize logger, if no arguments are provided a default config will be used. */
export const initLogger = (config: ILoggerConfig = defaultLoggerConfig) =>
    initLoggerBinding(JSON.stringify(config));
