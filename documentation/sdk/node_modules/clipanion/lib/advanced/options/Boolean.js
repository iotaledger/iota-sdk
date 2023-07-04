'use strict';

Object.defineProperty(exports, '__esModule', { value: true });

var utils = require('./utils.js');

function Boolean(descriptor, initialValueBase, optsBase) {
    const [initialValue, opts] = utils.rerouteArguments(initialValueBase, optsBase !== null && optsBase !== void 0 ? optsBase : {});
    const optNames = descriptor.split(`,`);
    const nameSet = new Set(optNames);
    return utils.makeCommandOption({
        definition(builder) {
            builder.addOption({
                names: optNames,
                allowBinding: false,
                arity: 0,
                hidden: opts.hidden,
                description: opts.description,
                required: opts.required,
            });
        },
        transformer(builer, key, state) {
            let currentValue = initialValue;
            for (const { name, value } of state.options) {
                if (!nameSet.has(name))
                    continue;
                currentValue = value;
            }
            return currentValue;
        },
    });
}

exports.Boolean = Boolean;
