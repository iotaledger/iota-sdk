const path = require('path');

module.exports = {
  mode: 'production',
  entry: './src/Collapsible.js',
  output: {
    filename: 'index.js',
    path: path.resolve(__dirname, 'dist'),
    libraryTarget: 'umd',
    /**
     * Makes UMD build available on both browsers and Node.js
     * https://webpack.js.org/configuration/output/#outputglobalobject
     */
    globalObject: 'this',
  },
  externals: ['react'],
  module: {
    rules: [
      {
        test: /\.m?js$/,
        exclude: /(node_modules|bower_components)/,
        use: {
          loader: 'babel-loader',
        },
      },
    ],
  },
};
