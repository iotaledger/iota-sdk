'use strict';

Object.defineProperty(exports, '__esModule', { value: true });

var errors = require('../../errors.js');

const isOptionSymbol = Symbol(`clipanion/isOption`);
function makeCommandOption(spec) {
    // We lie! But it's for the good cause: the cli engine will turn the specs into proper values after instantiation.
    return { ...spec, [isOptionSymbol]: true };
}
function rerouteArguments(a, b) {
    if (typeof a === `undefined`)
        return [a, b];
    if (typeof a === `object` && a !== null && !Array.isArray(a)) {
        return [undefined, a];
    }
    else {
        return [a, b];
    }
}
function cleanValidationError(message, lowerCase = false) {
    let cleaned = message.replace(/^\.: /, ``);
    if (lowerCase)
        cleaned = cleaned[0].toLowerCase() + cleaned.slice(1);
    return cleaned;
}
function formatError(message, errors$1) {
    if (errors$1.length === 1) {
        return new errors.UsageError(`${message}: ${cleanValidationError(errors$1[0], true)}`);
    }
    else {
        return new errors.UsageError(`${message}:\n${errors$1.map(error => `\n- ${cleanValidationError(error)}`).join(``)}`);
    }
}
function applyValidator(name, value, validator) {
    if (typeof validator === `undefined`)
        return value;
    const errors = [];
    const coercions = [];
    const coercion = (v) => {
        const orig = value;
        value = v;
        return coercion.bind(null, orig);
    };
    const check = validator(value, { errors, coercions, coercion });
    if (!check)
        throw formatError(`Invalid value for ${name}`, errors);
    for (const [, op] of coercions)
        op();
    return value;
}

exports.applyValidator = applyValidator;
exports.cleanValidationError = cleanValidationError;
exports.formatError = formatError;
exports.isOptionSymbol = isOptionSymbol;
exports.makeCommandOption = makeCommandOption;
exports.rerouteArguments = rerouteArguments;
