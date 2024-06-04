# Konbaato・コンバート

Convert between different Japanese scripts.

Ready for WASM 🌀

## Use the crate in Rust

Include the latest version of `konbaato` in your `Cargo.toml`.

```toml
[dependencies]
konbaato = "1.0"
```

## Install nix

Use the following command to install nix if you don't already have it.

```bash
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install
```

_You might need to open a new terminal for the changes to take effect._

## Use the crate in JavaScript

Build the WASM package inside the developer shell for deployments.

```bash
nix develop .#deploy
just wasm
```

Import the output in `./pkg` in JavaScript, as shown in [index.html](https://github.com/uncomputable/konbaato/blob/master/index.html).

```js
import init, { hira_to_kata } from './pkg/konbaato.js';

async function run() {
    await init();
    const input = "ひらがな";
    const output = hira_to_kata(input);
}

run()
```

## Develop the crate

Run CI inside the default developer shell.

```bash
nix develop
just check
```

Deploy a local website.

```bash
nix develop
just serve
```

Run the fuzzer inside the develper shell for fuzzing.

```bash
nix develop .#fuzz
just fuzz
```
