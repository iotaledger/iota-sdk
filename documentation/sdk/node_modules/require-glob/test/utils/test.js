'use strict';

const assert = require('assert');
const tests = [];

function test(msg, fn) {
	tests.push([msg, fn]);
}

process.nextTick(async function run() {
	for (const [msg, fn] of tests) {
		try {
			await fn(assert);
			console.log(`pass - ${msg}`);
		} catch (error) {
			console.error(`fail - ${msg}`, error);
			process.exit(1);
		}
	}
});

module.exports = test;
