const visit = require('unist-util-visit');
const htmlCommentRegex = require('html-comment-regex');

// Where the stuff actually happens!
const removeComments = (tree, file) => {
  const handler = (node, index, parent) => {
    const isComment = node.value.match(htmlCommentRegex);

    if (isComment) {
      // remove node
      parent.children.splice(index, 1);
      // Do not traverse `node`, continue at the node *now* at `index`. http://unifiedjs.com/learn/recipe/remove-node/
      return [visit.SKIP, index];
    }
  };

  visit(tree, 'html', handler);

  visit(tree, 'jsx', handler);
};

module.exports = removeComments;
