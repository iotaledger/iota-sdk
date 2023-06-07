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
        //overriding default exclude array to include the python api's classes with _ at the beginning
        //but still exclude any _admonitions
        exclude: [
          // '**/_*.{js,jsx,ts,tsx,md}',
          // '**/_*/**',
          '**/*.test.{js,jsx,ts,tsx}',
          '**/__tests__/**',
          '**/_admonitions/_**',
        ],
      }
    ],
  ],
  staticDirectories: [path.resolve(__dirname, 'static')],
};
