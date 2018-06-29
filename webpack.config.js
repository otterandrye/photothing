const path = require('path');
const nodeExternals = require('webpack-node-externals');

module.exports = [
	{
	  entry: './src/server.js',
    target: 'node',
    externals: [nodeExternals()],
	  output: {
	    filename: 'server.js',
	    path: path.resolve(__dirname, 'dist')
	  }
	},
	{
		entry: './src/client.js',
		output: {
			filename: 'client.js',
			path: path.resolve(__dirname, 'dist')
		}
	}
];