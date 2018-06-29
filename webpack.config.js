const path = require('path');
const nodeExternals = require('webpack-node-externals');
const FlowWebpackPlugin = require('flow-webpack-plugin');

const modules = {
  rules: [
    {
      test: /\.js$/,
      exclude: /node_modules/,
      loader: "babel-loader",
      options: {
        presets: ['@babel/preset-env', "@babel/preset-flow", "@babel/preset-react"],
      }
    },
    {
      enforce: "pre",
      test: /\.js$/,
      exclude: /node_modules/,
      loader: "eslint-loader",
      options: {
        emitError: true,
        emitWarning: true,
        failOnError: true,
      }
    },
    {
      enforce: 'pre',
      test: /\.js?$/,
      loader: 'prettier-loader',
      exclude: /node_modules/,
      options: {
        parser: "babylon",
      }
    }
  ],
};

const plugins = [
  new FlowWebpackPlugin(),
];

module.exports = [
	{
	  entry: './src/server.js',
    target: 'node',
    externals: [nodeExternals()],
    module: modules,
	  output: {
	    filename: 'server.js',
	    path: path.resolve(__dirname, 'dist')
	  },
    plugins
	},
	{
		entry: './src/client.js',
    module: modules,
		output: {
			filename: 'client.js',
			path: path.resolve(__dirname, 'dist')
		},
    plugins
	}
];