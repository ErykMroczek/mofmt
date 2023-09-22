# Changelog

All important changes will be described in this file. Or rather I will
try to document them here.

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
