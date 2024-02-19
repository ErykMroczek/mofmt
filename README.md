# mofmt - Modelica code formatter

*mofmt* is a code formatter for [Modelica](https://modelica.org/)
language. It aims to enhance readability and provide style constistency
across different Modelica projects.

Code style applied by the *mofmt* is described in `code-style.md`.

**mofmt** assumes that files are encoded as UTF-8 (without BOM). Other encoding
will cause it to panic.

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

**mofmt** expects one or more paths that point to either Modelica source files,
or to directories with such files:

```shell
mofmt <PATHS>
```

So you can format a single file:

```shell
mofmt foo.mo
```

or multiple files:

```shell
mofmt foo.mo bar.mo baz.mo
```

or all files inside the directory (**mofmt** searches for files recursively):

```shell
mofmt ./foo-dir
```

or all files inside multiple directories

```shell
mofmt ./foo-dir ./bar-dir
```

or you can mix both files and directories:

```shell
mofmt ./foo-dir foo.mo bar.mo ./bar-dir baz.mo
```

**mofmt** can run in *check mode*. In this mode files are not modified, instead
**mofmt** checks the formatting in the original file, and reports an error in
case of a failure:

```shell
mofmt --check <PATHS>
```

## TODO

[ ] include HTML pretty-printer

## License

MPL-2.0

## Authors

Eryk Mroczek: <mroczek.eryk@gmail.com>
