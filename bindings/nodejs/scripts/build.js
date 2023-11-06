const { resolve } = require('path');
const { spawnSync } = require('child_process');

// Passing "--prepack 'yarn build'" causes problems on Windows, so this is a workaround

const { status } = spawnSync(process.platform === 'win32' ? 'yarn.cmd' : 'yarn', ['build'], {
    stdio: 'inherit',
    cwd: resolve(__dirname, '../'),
});

if (status === null) {
    process.exit(1);
} else if (status > 0) {
    process.exit(status);
}
