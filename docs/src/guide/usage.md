# Usage

**mofmt** tries to avoid switches and options, so using it is straightforward.

This is a help message that summarizes the interface:

```none
mofmt: Modelica code formatter

Usage: mofmt SRC ...

Options:
-h, --help: display this message and exit
-v, --version: display a version number and exit
```

So running the formatter on some Modelica file only requires path of this file:

```bash
mofmt ./SomeModelicaCode.mo
```

You can format multiple files with one command:

```bash
mofmt SomeModelicaCode_1.mo SomeModelicaCode_2.mo
```

You can format whole directories (recursively):

```bash
mofmt ./SomeDirectoryWithModelicaFiles
```

or format everything in the current directory:

```bash
mofmt .
```

You can also mix files and directories in the argument list:

```bash
mofmt SomeModelicaCode_1.mo ./SomeDirectoryWithModelicaFiles SomeModelicaCode_2.mo
```

If **mofmt** meets any syntax error, it notifies you without touching this file.
