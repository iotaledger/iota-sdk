const transformer = require('./transformer');

function attacher() {
  return transformer;
}

module.exports = attacher;
