// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { __UtilsMethods__ } from '../../types/utils';

// @ts-ignore: path is set to match runtime transpiled js path
import addon = require('../../../build/Release/index.node');

const { callUtilsMethod, returnJsString } = addon;

const callUtilsMethodJson = (method: __UtilsMethods__): any =>
    JSON.parse(callUtilsMethod(JSON.stringify(method))).payload;

export { callUtilsMethodJson, returnJsString };
