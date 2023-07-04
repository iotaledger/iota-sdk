# `to-gatsby-remark-plugin`

Convert remark plugins to gatsby remark plugins.

## Installation

```sh
yarn add to-gatsby-remark-plugin
# npm install to-gatsby-remark-plugin
```

## Why

**Remark plugins** are plugins that can be used with [`remark`](https://remark.js.org/) ([`unifiedjs`](https://unifiedjs.com/)), there are already tons of [plugins](https://github.com/remarkjs/remark/blob/master/doc/plugins.md) in the community. **Gatsby remark plugins** are plugins that can only be used in Gatsby, which is capable of using some of the Gatsby's specific APIs and graphql interfaces. However, we don't usually need all those APIs in our **remark plugins**, sometimes the plugins just have to manipulate the AST.

[`gatsby-transformer-remark`](https://www.gatsbyjs.org/packages/gatsby-transformer-remark/#how-to-use) only supports passing **Gatsby remark plugins**. [`gatsby-plugin-mdx`](https://www.gatsbyjs.org/packages/gatsby-plugin-mdx) supports both **Gatsby remark plugins** and **remark plugins** with [`gatsbyRemarkPlugins`](https://www.gatsbyjs.org/packages/gatsby-plugin-mdx/#gatsby-remark-plugins) and [`remarkPlugins`](https://www.gatsbyjs.org/packages/gatsby-plugin-mdx/#remark-plugins) options separately. However, `gatsbyRemarkPlugins` always have precedence over `remarkPlugins`, which makes it difficult to compose multiple plugins in specific order.

As a result, plugin authors tend to re-create a **Gatsby remark plugin** for every **remark plugin**. This is far from ideal. What if we can just convert our **remark plugins** into **Gatsby remark plugins** automatically? This is exactly what this package does!

## Usage

### For users

Create a file locally with the following code.

```js
// gatsby-my-remark-plugin.js
const toGatsbyRemarkPlugin = require('to-gatsby-remark-plugin');
const myRemarkPlugin = require('my-remark-plugin');

module.exports = toGatsbyRemarkPlugin(myRemarkPlugin);
```

Reference this file when specifying in the gatsby remark configs.

```js
// gatsby-config.js
module.exports = ({
  plugins: [
    {
      resolve: `gatsby-plugin-mdx`,
      options: {
        gatsbyRemarkPlugins: [
          {
            resolve: require.resolve(`./gatsby-my-remark-plugin.js`),
          }
        ]
      },
    },
  ],
});
```

### For library authors

We can focus on creating the **remark plugin**, then add a **sub-package** just for Gatsby. Create a `gatsby` directory under the root directory with 2 files inside: `package.json` and `index.js`.

```js
// gatsby/package.json
{
  "name": "gatsby-my-remark-plugin",
  "main": "index.js"
}
```

```js
// gatsby/index.js
const toGatsbyRemarkPlugin = require('to-gatsby-remark-plugin');
const myRemarkPlugin = require('..');

module.exports = toGatsbyRemarkPlugin(myRemarkPlugin);
```

Then the users can just reference the **Gatsby remark plugin** via `my-remark-plugin/gatsby`.

```js
// gatsby-config.js
module.exports = ({
  plugins: [
    {
      resolve: `gatsby-plugin-mdx`,
      options: {
        gatsbyRemarkPlugins: [
          {
            resolve: `my-remark-plugin/gatsby`
          }
        ]
      },
    },
  ],
});
```
