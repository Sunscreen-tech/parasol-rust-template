# Parasol Rust Monorepo Template

![Rust](https://github.com/Sunscreen-tech/parasol-rust-template/workflows/Rust/badge.svg)
![Solidity](https://github.com/Sunscreen-tech/parasol-rust-template/workflows/Solidity/badge.svg)

**A template for quickly getting started with developing Rust applications that
leverage FHE for smart contract development.**

Continuous Integration is already set up to test both your Rust and Solidity
code, as well as ensure formatting and that your Rust bindings match the
Solidity build artifacts.

## Directory Structure

The project is structured as a mixed Rust workspace with a Foundry project under
`contracts/` and typesafe auto-generated bindings to the contracts under
`bindings/`.

```
├── Cargo.toml
├── app // <-- Your Rust application logic
├── contracts // <- The smart contracts + tests using Foundry
├── bindings // <-- Generated bindings to the smart contracts' abis (like Typechain)
```

## Installing Foundry

First, you'll need `cargo`; if you don't have it, the easiest way is to install via `rustup`:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then you can install our foundry fork:

```sh
cargo install --git https://github.com/Sunscreen-tech/foundry --profile local forge cast anvil
```

For more info on foundry, see the official
[docs](https://book.getfoundry.sh/).

## Testing

Given the repository contains both Solidity and Rust code, there's 2 different
workflows.

### Solidity

Forge uses submodules to manage dependencies. Initialize the dependencies:

```bash
forge install
```

If you are in the root directory of the project, run:

```bash
forge test --root ./contracts
```

If you are in in `contracts/`:

```bash
forge test
```

### Rust

```
cargo test
```

## Generating Rust bindings to the contracts

Rust bindings to the contracts can be generated via `forge bind`, which requires
first building your contracts:

```
forge build --root ./contracts
forge bind --bindings-path ./bindings --root ./contracts --crate-name bindings
```

Any follow-on calls to `forge bind` will check that the generated bindings match
the ones under the build files. If you want to re-generate your bindings, pass
the `--overwrite` flag to your `forge bind` command.
