# Introduction

**mofmt** is a code formatter for the [Modelica](https://modelica.org/)
language. It aims to enhance readability and provide style consistency across
different Modelica projects.

mofmt is fairly opinionated, but it doesn't have, nor it will have, any option
to define a maximum line length. Instead, it detects legal line wraps in the
original files, and preserves them, adding additional wraps to achieve visual
consistency.

Detailed style description can be found in the **Style** section of this book.

## License

The mofmt source and documentation are released under the [Mozilla Public
License v2.0.](https://www.mozilla.org/en-US/MPL/2.0/).
