const vfile = require('to-vfile');

function toGatsbyRemarkPlugin(remarkPlugin) {
  return function({ markdownAST, markdownNode }, options) {
    const file = vfile(markdownNode.fileAbsolutePath);

    return remarkPlugin(options)(markdownAST, file);
  }
}

module.exports = toGatsbyRemarkPlugin;
