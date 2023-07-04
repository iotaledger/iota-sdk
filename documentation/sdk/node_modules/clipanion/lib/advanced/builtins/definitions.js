'use strict';

Object.defineProperty(exports, '__esModule', { value: true });

var Command = require('../Command.js');

/**
 * A command that prints the clipanion definitions.
 */
class DefinitionsCommand extends Command.Command {
    async execute() {
        this.context.stdout.write(`${JSON.stringify(this.cli.definitions(), null, 2)}\n`);
    }
}
DefinitionsCommand.paths = [[`--clipanion=definitions`]];

exports.DefinitionsCommand = DefinitionsCommand;
