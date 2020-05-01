const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = {
  plugins: [
    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ],
};
