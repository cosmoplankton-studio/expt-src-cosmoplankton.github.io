const path = require('path');
const webpack = require('webpack');
const wasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

let pages = ["index", "webgl"]
let plugins = [
    new CleanWebpackPlugin(),
    new CopyWebpackPlugin({
        patterns: [
            { from: 'static' }
        ]
    }),
    new wasmPackPlugin({
        crateDirectory: path.resolve(__dirname, "app-core"),
        outDir: path.resolve(__dirname, "app-core/pkg"),
        outName: "app-core",
    })
];
plugins = plugins.concat(
    pages.map(
        (page) => 
            new HtmlWebpackPlugin({
                template: path.resolve(__dirname, `src/${page}-template.html`),
                filename: `${page}.html`,
                chunks: [page],
              })
      )
)

module.exports = {
    mode: 'production',
    // entry: {
    //    index: path.resolve(__dirname, 'src/index.js')
    // },
    entry: pages.reduce((config, page) => {
        config[page] = path.resolve(__dirname, `src/${page}.js`);
        return config;
      }, {}),
    output: {
        filename: '[name].bundle.js',
        path: path.resolve(__dirname, 'dist'),
        devtoolNamespace: 'devtool-namespace'
    },
    plugins: plugins,
    experiments: {
        // asyncWebAssembly: true,
        syncWebAssembly: true,
    },
    module: {
        rules: [
        {
            test: /\.?js$/,
            exclude: /node_modules/,
            use: {
            loader: "babel-loader",
            options: {
                presets: ['@babel/preset-env', '@babel/preset-react']
              }
            }
        },
        ]
    },
  optimization: {
    splitChunks: {
      chunks: "all",
    },
  },
};

