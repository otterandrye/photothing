const path = require('path');
const nodeExternals = require('webpack-node-externals');

const modules = {
  rules: [
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
      test: /\.js$/,
      exclude: /node_modules/,
      loader: "babel-loader",
      options: {
        presets: ['@babel/preset-env', "@babel/preset-flow", "@babel/preset-react"],
      }
    },
  ],
};


module.exports = [
	{
	  entry: './src/server.js',
    target: 'node',
    externals: [nodeExternals()],
    module: modules,
	  output: {
	    filename: 'server.js',
	    path: path.resolve(__dirname, 'dist')
	  }
	},
	{
		entry: './src/client.js',
    module: modules,
		output: {
			filename: 'client.js',
			path: path.resolve(__dirname, 'dist')
		}
	}
];