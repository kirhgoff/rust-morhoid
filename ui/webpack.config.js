const path = require("path");

module.exports = {
  entry: "./src/index.js",
  output: {
    filename: "bundle.js",
    path: path.resolve(__dirname, "build")
  },
  module: {
    rules: [
      {
        test: /\.(js|jsx)$/,
        exclude: /node_modules/,
        use: {
          loader: "babel-loader"
        }
      },
      {
        test:/\.css$/,
        use: ['style-loader','css-loader']
      },
      {
        test: /\.(htm|html)$/,
        use: {
          loader: "file-loader",
          options: {
            name: '[name].[ext]',
            outputPath: '/'
          }
        }
      }
    ]
  },
  mode: "development"
};
