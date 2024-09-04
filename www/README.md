<div align="center">

<h1><code>create-wasm-app</code></h1>

<strong>An <code>npm init</code> template for kick starting a project that uses NPM packages containing Rust-generated WebAssembly and bundles them with Webpack.</strong>

<sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>

</div>

## About

This template is designed for depending on NPM packages that contain
Rust-generated WebAssembly and using them to create a Website.

- Want to create an NPM package with Rust and WebAssembly? [Check out
  `wasm-pack-template`.](https://github.com/rustwasm/wasm-pack-template)
- Want to make a monorepo-style Website without publishing to NPM? Check out
  [`rust-webpack-template`](https://github.com/rustwasm/rust-webpack-template)
  and/or
  [`rust-parcel-template`](https://github.com/rustwasm/rust-parcel-template).

## 🚴 Usage

```
npm init wasm-app
```

## 🔋 Batteries Included

- `.gitignore`: ignores `node_modules`
- `LICENSE-APACHE` and `LICENSE-MIT`: most Rust projects are licensed this way, so these are included for you
- `README.md`: the file you are reading now!
- `index.html`: a bare bones html document that includes the webpack bundle
- `index.js`: example js file with a comment showing how to import and use a wasm pkg
- `package.json` and `package-lock.json`:
  - pulls in devDependencies for using webpack:
    - [`webpack`](https://www.npmjs.com/package/webpack)
    - [`webpack-cli`](https://www.npmjs.com/package/webpack-cli)
    - [`webpack-dev-server`](https://www.npmjs.com/package/webpack-dev-server)
  - defines a `start` script to run `webpack-dev-server`
- `webpack.config.js`: configuration file for bundling your js with webpack

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

---

<h1><code>Snake Game Instructions</code></h1>

## Running in development mode

Cd into the `www` folder and run `npm run build:dev`. This is goig to compile the rust code in the `public` folder (files with the `.wasm` extension). This will also simultaneously start the development server as specified in the `webpack.config.js` file.

Open your browser on `http://localhost:8080/` and the game should show

## Running in production mode

Cd into the `www` folder and run `npm run build`. This is goig to build the application as specified in the `webpack.config.js` file. Then go to the root server and type `npm start`, which will start an express server (configured in the `server` folder) that will serve the application on `http://localhost:3000/`
