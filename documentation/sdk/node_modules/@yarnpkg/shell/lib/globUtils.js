"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.isBraceExpansion = exports.match = exports.isGlobPattern = exports.fastGlobOptions = exports.micromatchOptions = void 0;
const tslib_1 = require("tslib");
const fslib_1 = require("@yarnpkg/fslib");
const fast_glob_1 = tslib_1.__importDefault(require("fast-glob"));
const fs_1 = tslib_1.__importDefault(require("fs"));
const micromatch_1 = tslib_1.__importDefault(require("micromatch"));
exports.micromatchOptions = {
    // This is required because we don't want ")/*" to be a valid shell glob pattern.
    strictBrackets: true,
};
exports.fastGlobOptions = {
    onlyDirectories: false,
    onlyFiles: false,
};
/**
 * Decides whether a string is a glob pattern, using micromatch.
 *
 * Required because `fastGlob.isDynamicPattern` doesn't have the `strictBrackets` option.
 */
function isGlobPattern(pattern) {
    // The scanner extracts globs from a pattern, but doesn't throw errors
    if (!micromatch_1.default.scan(pattern, exports.micromatchOptions).isGlob)
        return false;
    // The parser is the one that throws errors
    try {
        micromatch_1.default.parse(pattern, exports.micromatchOptions);
    }
    catch {
        return false;
    }
    return true;
}
exports.isGlobPattern = isGlobPattern;
function match(pattern, { cwd, baseFs }) {
    return (0, fast_glob_1.default)(pattern, {
        ...exports.fastGlobOptions,
        cwd: fslib_1.npath.fromPortablePath(cwd),
        fs: (0, fslib_1.extendFs)(fs_1.default, new fslib_1.PosixFS(baseFs)),
    });
}
exports.match = match;
function isBraceExpansion(pattern) {
    return micromatch_1.default.scan(pattern, exports.micromatchOptions).isBrace;
}
exports.isBraceExpansion = isBraceExpansion;
