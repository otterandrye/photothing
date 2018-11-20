/* eslint-disable flowtype/require-valid-file-annotation */

const path = require('path');
const webpack = require('webpack');
const nodeExternals = require('webpack-node-externals');
const FlowWebpackPlugin = require('flow-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

const rules = [
  {
    test: /\.js$/,
    exclude: /node_modules|editor-backend/,
    loader: "babel-loader",
    options: {
      presets: ['@babel/preset-env', "@babel/preset-flow", "@babel/preset-react"],
      plugins: ['@babel/plugin-proposal-class-properties', '@babel/plugin-syntax-dynamic-import'],
    }
  },
  {
    enforce: "pre",
    test: /\.js$/,
    exclude: /node_modules|editor-backend/,
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
    exclude: /node_modules|editor-backend/,
    options: {
      parser: "babylon",
    }
  },
  {
    enforce: 'pre',
    test: /\.(css)$/,
    loader: 'prettier-loader',
    exclude: /node_modules|editor-backend/,
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

const resolve = {
  alias: {
    'editor-backend': path.resolve(__dirname, './editor-backend/editor_backend.js'),
    'editor-backend-wasm': path.resolve(__dirname, './editor-backend/editor_backend_bg.wasm'),
  }
}

module.exports = [
	{
	  entry: './src/server.js',
    resolve,
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
    ],
    devtool: 'source-map',
	},
	{
		entry: './src/client.js',
    resolve,
    module: {
      rules: clientRules,
    },
		output: {
			filename: 'client.js',
			path: path.resolve(__dirname, 'dist'),
      publicPath: 'static/',
		},
    plugins: [
      ...plugins,
      new webpack.DefinePlugin({
        'SERVER': JSON.stringify(false),
      }),
      new MiniCssExtractPlugin()
    ],
    devtool: 'source-map',
	},
  {
		entry: './src/public_gallery/client.js',
    resolve,
    module: {
      rules: clientRules,
    },
		output: {
			filename: 'gallery-client.js',
			path: path.resolve(__dirname, 'dist'),
      publicPath: 'static/',
		},
    plugins: [
      ...plugins,
      new webpack.DefinePlugin({
        'SERVER': JSON.stringify(false),
      }),
      new MiniCssExtractPlugin()
    ],
    devtool: 'source-map',
	},
];
