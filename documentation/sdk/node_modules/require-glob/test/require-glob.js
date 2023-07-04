'use strict';

const path = require('path');
const test = require('./utils/test.js');
const requireGlob = require('../src/require-glob');

test('should require nothing', async (t) => {
	const bogusA = await requireGlob('./fixtures/bogu*.js');
	const bogusB = requireGlob.sync('./fixtures/bogu*.js');

	t.deepEqual(bogusA, {});
	t.deepEqual(bogusB, {});
});

test('should require a module', async (t) => {
	const oneA = await requireGlob('./fixtures/rand*.js');
	const oneB = requireGlob.sync('./fixtures/rand*.js');

	t.equal(typeof oneA.random, 'number');
	t.equal(typeof oneB.random, 'number');
});

test('should require multiple modules', async (t) => {
	const shallowA = await requireGlob('./fixtures/shallow/**/*.js');
	const shallowB = requireGlob.sync('./fixtures/shallow/**/*.js');
	const expected = {
		a: 'a',
		b: 'b',
		c: 'c',
		d: {
			e: 'e',
		},
	};

	t.deepEqual(shallowA, expected);
	t.deepEqual(shallowB, expected);
});

test('should require nested modules', async (t) => {
	const deepA = await requireGlob('./fixtures/deep/**/*.js');
	const deepB = requireGlob.sync('./fixtures/deep/**/*.js');
	const expected = {
		a: {
			a1: 'a1',
			a2: 'a2',
		},
		b: {
			b_bB: {
				// eslint-disable-line camelcase
				_bB1: '_b.b1',
				bB2: 'b.b2',
			},
			b1: 'b1',
			b2: 'b2',
		},
	};

	t.deepEqual(deepA, expected);
	t.deepEqual(deepB, expected);
});

test('should require multiple patterns', async (t) => {
	const deep = await requireGlob([
		'./fixtures/{deep,shallow}/**/*.js',
		'!./**/a*',
	]);

	const expected = {
		deep: {
			b: {
				b_bB: {
					// eslint-disable-line camelcase
					_bB1: '_b.b1',
					bB2: 'b.b2',
				},
				b1: 'b1',
				b2: 'b2',
			},
		},
		shallow: {
			b: 'b',
			c: 'c',
			d: {
				e: 'e',
			},
		},
	};

	t.deepEqual(deep, expected);
});

test('should use custom cwd', async (t) => {
	const deep = await requireGlob('./test/**/deep/**/*.js', {
		cwd: path.dirname(__dirname),
	});

	const expected = {
		fixtures: {
			deep: {
				a: {
					a1: 'a1',
					a2: 'a2',
				},
				b: {
					b_bB: {
						// eslint-disable-line camelcase
						_bB1: '_b.b1',
						bB2: 'b.b2',
					},
					b1: 'b1',
					b2: 'b2',
				},
			},
		},
	};

	t.deepEqual(deep, expected);
});

test('should use custom base', async (t) => {
	const deep = await requireGlob('./fixtures/deep/**/*.js', {
		base: path.join(__dirname, 'fixtures'),
	});

	const expected = {
		deep: {
			a: {
				a1: 'a1',
				a2: 'a2',
			},
			b: {
				b_bB: {
					// eslint-disable-line camelcase
					_bB1: '_b.b1',
					bB2: 'b.b2',
				},
				b1: 'b1',
				b2: 'b2',
			},
		},
	};

	t.deepEqual(deep, expected);
});

test('should bust cache', async (t) => {
	const a = await requireGlob('./fixtures/rand*.js');
	const b = await requireGlob('./fixtures/rand*.js');
	const c = await requireGlob('./fixtures/rand*.js', { bustCache: true });
	const d = await requireGlob('./fixtures/rand*.js', { bustCache: true });
	const e = await requireGlob('./fixtures/rand*.js');

	t.equal(a.random, b.random);
	t.notEqual(b.random, c.random);
	t.notEqual(c.random, d.random);
	t.equal(d.random, e.random);
});

test('should use custom mapper', async (t) => {
	const deep = requireGlob.sync('./fixtures/deep/**/*.js', {
		mapper: function (options, filePath) {
			const base = path.basename(filePath);

			return {
				path: base.toUpperCase(),
				exports: base,
			};
		},
	});

	const expected = {
		A1: 'a1.js',
		A2: 'a2.js',
		B1: 'b1.js',
		B2: 'b2.js',
		BB2: 'b.b2.js',
		_BB1: '_b.b1.js',
	};

	t.deepEqual(deep, expected);
});

test('should use custom reducer', async (t) => {
	const deep = await requireGlob('./fixtures/deep/**/*.js', {
		reducer: function (options, tree, file) {
			const base = path.basename(file.path);

			tree[base.toUpperCase()] = file.exports;

			return tree;
		},
	});

	const expected = {
		'A1.JS': 'a1',
		'A2.JS': 'a2',
		'B1.JS': 'b1',
		'B2.JS': 'b2',
		'B.B2.JS': 'b.b2',
		'_B.B1.JS': '_b.b1',
	};

	t.deepEqual(deep, expected);
});

test('should use custom keygen', async (t) => {
	const deep = await requireGlob('./fixtures/deep/**/*.js', {
		keygen: function (options, file) {
			return file.path
				.replace(file.base + path.sep, '')
				.replace(/\\/g, '/');
		},
	});

	const expected = {
		'a/a1.js': 'a1',
		'a/a2.js': 'a2',
		'b/b_b-b/_b.b1.js': '_b.b1',
		'b/b_b-b/b.b2.js': 'b.b2',
		'b/b1.js': 'b1',
		'b/b2.js': 'b2',
	};

	t.deepEqual(deep, expected);
});

test('should use initial value', async (t) => {
	const result = await requireGlob([
		'./fixtures/{deep,shallow}/**/*.js',
		'!./**/a*',
	], {
		initialValue: [],
		reducer: (options, result, fileObject) => {
			result.push(fileObject.exports);
			return result;
		}
	});

	const expected = ['_b.b1','b.b2', 'b1', 'b2', 'b', 'c', { e: 'e' }];

	t.deepEqual(result.sort(), expected.sort());
});

test('should return initial value', async (t) => {
	const result = await requireGlob('./fixtures/bogu*.js', { initialValue: [] });

	const expected = [];

	t.deepEqual(result, expected);
});

test('should overwrite initial value', async (t) => {
	const oneA = await requireGlob('./fixtures/rand*.js', {
		initialValue: {
			fixed: 'a',
			random: 'b',
		}
	});
	const oneB = requireGlob.sync('./fixtures/rand*.js', {
		initialValue: {
			fixed: 'a',
			random: 'b',
		}
	});

	t.equal(typeof oneA.fixed, 'string')
	t.equal(typeof oneA.random, 'number');
	t.equal(typeof oneB.fixed, 'string')
	t.equal(typeof oneB.random, 'number');
})
