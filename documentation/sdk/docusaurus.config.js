const path = require('path');

module.exports = {
  plugins: [
    [
      '@docusaurus/plugin-content-docs',
      {
        id: 'iota-sdk',
        path: path.resolve(__dirname, 'docs'),
        routeBasePath: 'iota-sdk',
        sidebarPath: path.resolve(__dirname, 'sidebars.js'),
        editUrl: 'https://github.com/iotaledger/iota-sdk/edit/documentation/sdk',
        remarkPlugins: [require('remark-code-import'), require('remark-import-partial')],
      }
    ],
  ],
  staticDirectories: [path.resolve(__dirname, 'static')],
};
