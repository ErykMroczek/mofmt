# mofmt - Modelica code formatter

*Mofmt* is a code formatter for [Modelica](https://modelica.org/)
language. It aims to enhance readability and provide style constistency
across different Modelica projects.

This is initial version of this formatter. Although functionality is
more or less finished, it is highly probable that it can exhibit buggy
behavior. Better double-check changed files.

## Installation and usage

### Installation

*Mofmt* can be installed from PYPI:

```shell
pip install mofmt
```

On top of that, repo contains a necessary `pre-commit-hooks.yaml` file,
so if you are using git, you can delegate running *mofmt* to
[pre-commit](https://pre-commit.com/) framework.

### Usage

*mofmt* takes one kind of argument - path that points to Modelica source
file or directory that is supposed to contain such files. *mofmt* can
accept multiple such arguments.

```shell
mofmt PATH ...
```

## Features and limitations

### Vertical spacing and grouping

*Mofmt* aims to ensure that Modelica code is vertically grouped and
indented in a intuitive way that additionally allows you to fold/unfold
specific sections in your text editor. Yet it will try to preserve
single blank lines that you have placed manually, unless they were
placed in places that *mofmt* considers prohibited.

### Comments

Modelica language specification allows you to place comments between any
lexical units, but at least some software, like Dymola, doesn't respect
that and displace your comments if it feels like it. *Mofmt* tries to
avoid that (but bugs may happen!). Both comments and whitespaces between
them are preserved. Additionally, *mofmt* preceeds your inline comments
with a single space to enhance readability.

### Line wrapping

*Mofmt* doesn't have a notion of maximum line length and doesn't wrap
lines automatically. This is a deliberate choice, for many expressions
in Modelica are written in a way that resembles textbook formulas. Such
formulas contain terms that have a specific meaning and probably are
kept on the same line by Modelica developers. Any (reasonably simple)
algorithm would probably be too stupid for that, so there is no wrapping
algorithm in *mofmt*. Instead, it will respect your wrapping inside
expressions (provided you wrapped at some operator):

```modelica
Q_flow = alpha * surfaceArea *
(T_a - T_b);
```

and only adjust it slightly:

```modelica
Q_flow = alpha * surfaceArea
  * (T_a - T_b);
```

If wrap is placed inside function call, array etc.:

```modelica
cp = specificHeat_pT(p = p, T = temperature_ph(p = p,
h = h));
```

*mofmt* will ensure that the whole argument list is formatted
consistently, including nested calls:

```modelica
cp = specificHeat_pT(
  p = p,
  T = temperature_ph(
    p = p,
    h = h));
```

### Strings

*Mofmt* disallows line wrapping inside strings. If it finds wrapped
string:

```modelica
string = "Some long string that someone
          wrapped, but shouldn't";
```

it converts it into two or more concatenated strings with `+` between
them:

```modelica
string = "Some long string that someone"
  + "wrapped, but shouldn't";
```

Explicit (escaped) newline chars are kept intact.

## Future plans

* create dedicated file with description of style applied by *mofmt*
* improve test coverage
* improve parsing performance
* (maybe) include HTML pretty-printer

## License

MIT

## Authors

Eryk Mroczek: <mroczek.eryk@gmail.com>
