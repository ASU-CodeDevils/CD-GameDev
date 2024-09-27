# Contribution Guide
## Plugins and bundles
When you code, try to use bevy plugins and bundles whenever possible. Use Rust modules to isolate these plugins and bundles.

## Credits
If you have added assets from a 3rd party, ensure you also update the [`CREDITS.md`]("./CREDITS.md") file with the necessary information.

## Before submitting pull request
Run the following commands from the root directory of the project and fix any issues they find:
```bash
cargo clippy --workspace --all-targets --all-features -- -Dwarnings
cargo fmt --all -- --check
typos
```
Issues found by `cargo fmt --all -- --check` can be fixed by running `cargo fmt --all`. Issues found by `typos` can usually be fixed with `typos -w`.

If you do not have `typos` installed, you can install it first by running the
following command:
```bash
cargo install typos-cli
```
