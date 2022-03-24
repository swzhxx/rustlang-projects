const { defineConfig } = require('@vue/cli-service')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')
const webpack = require('webpack')
const path = require('path')

module.exports = defineConfig({
  transpileDependencies: true,
  configureWebpack: {
    experiments: {
      syncWebAssembly: true
    },
    resolve: {
      alias: {
        '@pkg': path.resolve(__dirname, './pkg')
      }
    },

    plugins: [
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, './../image-process'),
        outDir: path.resolve(__dirname, './pkg'),
        outName: 'image-process',
        forceMode: 'release'
      }),
      new webpack.ProvidePlugin({
        TextDecoder: ['text-encoding', 'TextDecoder'],
        TextEncoder: ['text-encoding', 'TextEncoder']
      })
    ]
  }
})
