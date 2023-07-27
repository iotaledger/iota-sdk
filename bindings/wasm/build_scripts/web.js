const path = require('path')
const fs = require('fs')
const { lintAll } = require('./lints')
const generatePackage = require('./utils/generatePackage');

const rustPackageName = "iota_sdk_wasm";
const wasmFilename = "iota_sdk_wasm_bg.wasm";

const RELEASE_FOLDER = path.join(__dirname, '../web/wasm/');
const entryFilePath = path.join(RELEASE_FOLDER, rustPackageName + '.js');
const entryFile = fs.readFileSync(entryFilePath).toString();

lintAll(entryFile);

let changedFile = entryFile
    // Comment out generated code as a workaround for webpack (does not recognise import.meta).
    // Regex to avoid hard-coding 'sdk_wasm_bg.wasm'.
    .replace(
        /input = new URL\((.*), import\.meta\.url\);/i,
        "// input = new URL($1, import.meta.url);"
    )
    // Create an init function which imports the wasm file.
    .concat(
        "let __initializedIotaWasm = false\r\n\r\nexport function init(path) {\r\n    if (__initializedIotaWasm) {\r\n        return Promise.resolve(wasm)\r\n    }\r\n    return __wbg_init(path || \'" + wasmFilename + "\').then(() => {\r\n        __initializedIotaWasm = true\r\n        return wasm\r\n    })\r\n}\r\n",
    );


fs.writeFileSync(
    entryFilePath,
    changedFile
);

const entryFilePathTs = path.join(RELEASE_FOLDER, rustPackageName + '.d.ts');
const entryFileTs = fs.readFileSync(entryFilePathTs).toString();

let changedFileTs = entryFileTs.concat(`
/**
* Loads the Wasm file so the lib can be used, relative path to Wasm file
*
* @param {string | undefined} path
*/
export function init(path?: string): Promise<void>;`
);

fs.writeFileSync(
    entryFilePathTs,
    changedFileTs
);

const newPackage = generatePackage({
    module: 'lib/index.js',
    types: 'lib/index.d.ts',
});

fs.writeFileSync(path.join(RELEASE_FOLDER + "../", 'package.json'), JSON.stringify(newPackage, null, 2));

// Export the Wasm init() function from index.ts.
const indexFile = path.join(__dirname, "..", "out", "lib", "index.ts");
fs.writeFileSync(indexFile, "// @ts-ignore\nimport { init } from '../wasm/iota_sdk_wasm';\n export default init;", { flag: 'a' });
