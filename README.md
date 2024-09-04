## About

Snake game build in rust and webassembly

Steps:

1. Follow the steps oulined in this [tutorial](https://rustwasm.github.io/docs/book/game-of-life/introduction.html)
2. Some of the packages generated are outdated, so in the `www` folder, update the package versions manually or do:

```
pm remove webpack webpack-cli webpack-dev-server copy-webpack-plugin
```

and then reinstall with

```
npm i -D webpack webpack-cli webpack-dev-server copy-webpack-plugin
```

3. In the `webpack.config.js` file, update the `CopyWebpackPlugin` plugin to:

```
 new CopyWebpackPlugin({
      patterns: [{ from: "./index.html", to: "./" }],
    })
```

To build the package run `wasm-pack build --target web`

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
- [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
- `LICENSE-APACHE` and `LICENSE-MIT`: most Rust projects are licensed this way, so these are included for you

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
