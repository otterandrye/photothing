const path = require('path');
const webpack = require('webpack');
const nodeExternals = require('webpack-node-externals');
const FlowWebpackPlugin = require('flow-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

const rules = [
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
    test: /\.(js)$/,
    loader: 'prettier-loader',
    exclude: /node_modules/,
    options: {
      parser: "babylon",
    }
  },
  {
    enforce: 'pre',
    test: /\.(css)$/,
    loader: 'prettier-loader',
    exclude: /node_modules/,
    options: {
      parser: "css",
    }
  }
];

const serverRules = rules.concat([{
  test: /\.css$/,
  use: [
    {
      loader: "css-loader/locals",
      options: {
        modules: true
      }
    },
    //'postcss-loader',
  ],
}]);

const clientRules = rules.concat([{
  test: /\.css$/,
  use: [
    MiniCssExtractPlugin.loader,
    {
      loader: "css-loader",
      options: {
        modules: true
      }
    },
    //'postcss-loader',
  ],
}]);

const plugins = [
  new FlowWebpackPlugin(),
];

module.exports = [
	{
	  entry: './src/server.js',
    target: 'node',
    externals: [nodeExternals()],
    module: {
      rules: serverRules,
    },
	  output: {
	    filename: 'server.js',
	    path: path.resolve(__dirname, 'dist')
	  },
    plugins: [
      ...plugins,
      new webpack.DefinePlugin({
        'SERVER': JSON.stringify(true),
      }),
    ]
	},
	{
		entry: './src/client.js',
    module: {
      rules: clientRules,
    },
		output: {
			filename: 'client.js',
			path: path.resolve(__dirname, 'dist')
		},
    plugins: [
      ...plugins,
      new webpack.DefinePlugin({
        'SERVER': JSON.stringify(false),
      }),
      new MiniCssExtractPlugin()
    ]
	}
];