## reversi-wasm

- reversi AI written in Rust
- can be compiled to WebAssembly
- runs in most browsers
- contains three algorithms
  - simple min-max with the tree depths of 2, 4 and 8
  - random
  - naive (selects the right-most and top-most)
- deployed at https://reversi.kkty.jp

![screenshot](https://user-images.githubusercontent.com/23012087/60339994-512d6200-99e5-11e9-83f1-caa682b2b827.gif)

*sizes of constructed game trees are logged in console (min-max only)*

## running locally

### ui only

```console
$ git clone https://github.com/kkty/reversi-wasm
$ cd reversi-wasm/ui
$ npm install
$ npm run build
$ npm run start
```

### core + ui

```console
$ git clone https://github.com/kkty/reversi-wasm
$ cd ./reversi-wasm/ui
$ sh -c "cd core && wasm-pack build && cd pkg && npm link"
$ sh -c "cd ui && npm i && npm link reversi-wasm-core && npm run dev"
```