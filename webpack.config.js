const path = require('path');
module.exports = {
  mode: 'development',
  entry: path.join(__dirname, 'ts', 'index.tsx'),
  output: {
    path: __dirname,
    filename: "index.js"
  },
  module: {
    rules: [
      {
        test: /\.([cm]?ts|tsx)$/,
        loader: "ts-loader"
      }
    ]
  },
  resolve: {
    extensions: ['.ts', '.tsx', ".js"],
    extensionAlias: {
      ".js": [".js", ".ts"]
    }
  },
  devtool: 'source-map'
};