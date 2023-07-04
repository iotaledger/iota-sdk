"use strict";
/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.base64 = void 0;
const tslib_1 = require("tslib");
const path_1 = tslib_1.__importDefault(require("path"));
const logger_1 = tslib_1.__importDefault(require("@docusaurus/logger"));
const sharp_1 = tslib_1.__importDefault(require("sharp"));
// eslint-disable-next-line @typescript-eslint/no-var-requires, global-require
const { version } = require('../package.json');
const ERROR_EXT = `Error: Input file is missing or uses unsupported image format, lqip v${version}`;
const SUPPORTED_MIMES = {
    jpeg: 'image/jpeg',
    jpg: 'image/jpeg',
    png: 'image/png',
};
/**
 * It returns a Base64 image string with required formatting to work on the web
 * (<img src=".." /> or in CSS url('..'))
 */
const toBase64 = (extMimeType, data) => `data:${extMimeType};base64,${data.toString('base64')}`;
async function base64(file) {
    let extension = path_1.default.extname(file);
    extension = extension.split('.').pop();
    const mime = SUPPORTED_MIMES[extension];
    if (!mime) {
        throw new Error(ERROR_EXT);
    }
    try {
        const data = await (0, sharp_1.default)(file).resize(10).toBuffer();
        return toBase64(mime, data);
    }
    catch (err) {
        logger_1.default.error `Generation of base64 failed for image path=${file}.`;
        throw err;
    }
}
exports.base64 = base64;
