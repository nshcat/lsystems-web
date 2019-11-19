const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');
const webpack = require('webpack');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  module: {
   rules: [
    {
	  test: /\.css$/,
	  use: ['style-loader', 'css-loader']
    }
   ]
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['index.html']),
	new webpack.ProvidePlugin({ // inject ES5 modules as global vars
	  $: 'jquery',
	  jQuery: 'jquery',
	  'window.jQuery': 'jquery',
	  Tether: 'tether'
	})
  ],
};
