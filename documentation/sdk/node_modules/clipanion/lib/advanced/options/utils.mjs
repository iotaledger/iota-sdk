import { UsageError } from '../../errors.mjs';

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
function formatError(message, errors) {
    if (errors.length === 1) {
        return new UsageError(`${message}: ${cleanValidationError(errors[0], true)}`);
    }
    else {
        return new UsageError(`${message}:\n${errors.map(error => `\n- ${cleanValidationError(error)}`).join(``)}`);
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

export { applyValidator, cleanValidationError, formatError, isOptionSymbol, makeCommandOption, rerouteArguments };
