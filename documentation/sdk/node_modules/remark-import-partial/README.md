# `remark-import-partial`

📝 Populate `@import` syntax with partial files

## Installation

```sh
yarn add -D remark-import-partial
```

## Setup

See [**Using plugins**](https://github.com/remarkjs/remark/blob/master/doc/plugins.md#using-plugins) in the official documentation.

## Usage

Transform:

```md
Some content

{@import ./my-name.md}

Other content
```

into:

```md
Some content

Dotan

Other content
```

The file path is relative to the markdown file path.

## License

Dotan Simha
[MIT](LICENSE)