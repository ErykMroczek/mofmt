# Installation

## Precompiled binaries

Precompiled mofmt binaries are available on the [GitHub Releases
page](https://github.com/ErykMroczek/mofmt/releases). Binaries of Windows and
Linux are available.

## Cargo

To use this option you need to have
[Rust](https://www.rust-lang.org/tools/install) installed on your machine.

**mofmt** is written in Rust and is available in a form of the [Rust
crate](https://crates.io/crates/mofmt).

To download and install it use the `cargo install` command:

```bash
cargo install mofmt
```

It will download, build and install the binary to your `.cargo` directory.

## pre-commit

**mofmt** is available as a [pre-commit](https://pre-commit.com/) hook. If you
are using pre-commit, put this in your `.pre-commit-config.yaml` file:

```yaml
repos:
- repo: https://github.com/ErykMroczek/mofmt
  rev: vX.X.X
  hooks:
    - id: mofmt
```

Replace `vX.X.X` with a valid tag like `v0.5.0`.
