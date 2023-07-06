# Changelog

All important changes will be described in this file. Or rather I will
try to document them here.

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
