const { override, addBabelPlugins } = require('customize-cra');

module.exports = override(addBabelPlugins('@babel/plugin-proposal-object-rest-spread'));
