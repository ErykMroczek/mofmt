# Changelog

All important changes will be described in this file. Or rather I will
try to document them here.

## [0.5.0] - 2024-02-18

This release is probably the last that introduces any significant changes to the
produced output.

It brings back the previous, better, approach to line wrapping inside
expressions. From now on expressions like arithmetical, logical etc. can only be
wrapped manually, and **mofmt** only ensures that the wrapped expression is
nicely indented.

Moreover, **mofmt** no longer allows wrapping of *output-expression-list* rule
in the similar fashion as function args etc. It looked strange when they
included discarded outputs.

With this release **mofmt** supports the check mode option, so it can be used as
a part of CI pipelines.

### Added

- check mode (`--check`)

### Changed

- expressions wrapped at binary operators are indented only once, and wrapping
  is not applied automatically to remaining operators

### Removed

- line wrapping in *output-expression-list*

## [0.4.2] - 2024-02-16

### Fixed

- fix incorrect indent of "class" annotations that follow section
  keywords like `algorithm`
- fix the bug that caused mofmt to panic in element redeclared as
  `replaceable`
- fix the issue of changed line endings when mofmt was used on Windows

## [0.4.1] - 2024-02-05

### Fixed

- *mofmt* no longer inserts a blank line before the `within`

## [0.4.0] - 2024-02-05

This release marks an important milestone. From now on `mofmt` will be
implemented in Rust. The migration was motivated by very poor
performance of Python implementation. This was caused partially by the
ANTLR parser having a general runtime that couldn't be tailored in any
way, and mainly because of the inherent slowness of Python.

New Rust implementation was around 50x faster. After including better
and more detailed heuristics it is still between 30-40x faster without
any performance tweaks. This both creates significant performance
savings that can be spent in the future on more advanced features like
HTML formatting, and opens possibilities of performance gains through
multithreading etc.

Few long existing bugs were fixed along the way. There are some minor
changes in the style applied by `mofmt`. Refer to the `code-style.md`.

### Added

- support for new syntactical constructs from Modelica 3.7
- cover *output-expression-list* production

### Changed

- expressions are now wrapped at every operator inside the precedence
  level where the original wrap occured as well as in all outer levels

### Fixed

- enum lists and argument lists are now wrapped if they contain
  descriptions without the need to run formatter twice

## [0.3.6] - 2024-01-09

Bugfixes and performance improvements.

### Changed

- Improve the performance up to 50%

### Fixed

- Remove the false syntax error caused by two comments separated with a blank line

## [0.3.5] - 2023-12-12

### Fixed

- Fix the issue with increasing indentation in expressions that are wrapped multiple times
- Remove unnecessary whitespaces inserted before matrices in function arguments
- Correct indentation in if-expressions

## [0.3.4] - 2023-12-06

### Added

- Add CLI options to print help message and `mofmt` version number

### Fixed

- Remove the parsing error when parsing function calls with mixed
  positional and named arguments

## [0.3.3] - 2023-11-30

### Fixed

- Fix increased indentation and empty lines in wrapped matrices
- Fix additional spaces placed in lines before type specifiers that use
  global paths

## [0.3.2] - 2023-11-23

### Fixed

- Fix line wrapping at inheritance modifiers (`break` statements inside `extends` clauses)
- Fix missing whitespaces before type specifiers that use global paths (starts with a dot)
- Fix errors caused by quoted identifiers containing spaces and double quotes

## [0.3.1] - 2023-10-13

### Fixed

- Fix line wrapping inside external function calls and matrix rows

## [0.3.0] - 2023-09-23

### Added

- Add info about formatted file path to error messages

### Changed

- Prevent formatter from formatting files that contain syntax errors

## [0.2.3] - 2023-09-12

### Fixed

- Fix increased indentation inside wrapped expressions
- Fix unwanted line breaks inside matrices
- Fix additional space inserted before the array argument that was
  broken by the user

## [0.2.2] - 2023-08-04

### Fixed

- Fix issue with incorrect indentation when function call is broken
  inside wrapped expression

## [0.2.1] - 2023-07-06

### Fixed

- Fix issue with additional blank lines when handling line comments
  between elements

## [0.2.0] - 2023-06-21

This release introduces changes to *mofmt* formatting style that makes
it more similar to Dymola's.

### Changed

#### Style

- Reduce indentation in class sections
- Add space between `annotation` keyword and class modification

### Removed

- Remove string token splitting and indentation

## [0.1.2] - 2023-06-06

### Fixed

- Fix issue with pre-commit hook running on multiple files

## [0.1.1] - 2023-05-28

### Fixed

- Fix recursive directory walking
- Fix grammar in some error messages

## [0.1.0] - 2023-05-27

_Initial version._
