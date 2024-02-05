# mofmt - Modelica code formatter

*mofmt* is a code formatter for [Modelica](https://modelica.org/)
language. It aims to enhance readability and provide style constistency
across different Modelica projects.

*mofmt* doesn't modify files if it meets syntax errors.

Code style applied by the *mofmt* is described in `code-style.md`.

## Installation and usage

### Installation

*mofmt* can be installed with `cargo`:

```shell
cargo install mofmt
```

or you can just grab one of the released libraries.

On top of that, repo contains a necessary `pre-commit-hooks.yaml` file,
so if you are using Git, you can delegate running *mofmt* to
[pre-commit](https://pre-commit.com/) framework.

### Usage

*mofmt* takes one kind of argument - path that points to Modelica source
file or directory that is supposed to contain such files. *mofmt* can
accept multiple such arguments.

```shell
mofmt PATH ...
```

## TODO

* include HTML pretty-printer
* include simple syntactical simplifications that don't affect the code
  semantics (removing redundant parentheses etc.)

## License

MPL-2.0

## Authors

Eryk Mroczek: <mroczek.eryk@gmail.com>
