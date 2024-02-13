# Code style

This document describes a code style as applied by the *mofmt*.

## Maximum line length

There is no line length limit. There are plenty of limits already.

## Horizontal spacing

Single space is used as a horizontal separator.

```modelica
parameter Real Foo(start = 0); // Comment
```

### Binary operators

All binary operators are surrounded with spaces.

```modelica
2 * (3 - 1.76)
a <= b or x
1 : 10
```

### Unary operators

`not` operator is followed by a space. Unary addition or subtraction,
including element-wise, are not.

```modelica
-4
.-(2 + 3)
not isOff
```

### Assignments and equalities

`:=` and `=` tokens are surrounded with spaces.
Exception occurs when equality or assignment is
followed by a multiline if-expression. In such case the symbol is only
preceeded with a space.

```modelica
foo = bar * 2
Real foo(min = 0, max = 2) = 1 / 2
foo := bar(x)
foo := if x < 2 then bar else baz
```

But:

```modelica
foo :=
  if x < 2 then
    bar
  else
    baz
```

### Arrays, modifications, function calls, expression lists

If there is no line wrap every comma and semicolon is followed by a
space. This includes discarded slots in *output-expression-list*.

```modelica
{1.0, 2.0, 3.0} // Array
enthalpy_pT(p, T) // Function call
(foo, ,bar, baz) // Output expression list with a discared second element
[x, y; a, b] // Matrix
```

## Indentation

Indentation is 2 spaces per level.

Modelica doesn't use indentation to define the scope, but it is
nonetheless useful to use it to enhance readability. Descriptions and
annotations are good examples. Overall, indentation is increased at:

* descriptions and annotations
* constraining clauses
* elements, equations, statements and external function calls
* enumeration items
* inside loops and if blocks

### Descriptions, annotations and constraining clauses

Indentation is increased only one time per element.

```modelica
replaceable package Medium = Modelica.Media.R134a.R134a_ph
  constrainedby Modelica.Media.Interfaces.PartialMedium
  "Fluid medium"
  annotation(Dialog(tab = "General", group = "Medium"));
```

### Element, statement and equation lists

Indentation is increased once per section.

```modelica
model Model
  "Some model"

  import FooPackage;
  import Modelica.Constants.inf;

  // SNAP

protected

  parameter FooPackage foo
    "foo parameter";

equation

  // SNAP
  foo = bar;

end Model;
```

### Enumeration items

Indentation is increased inside `enumeration()` and at every
description.

```modelica
type BoundaryType = enumeration(
  Pressure
    "Pressure boundary",
  Flow
    "Flow boundary")
  "Enumeration of possible boundary types";
```

### Loops and ifs

Indentation is increased once per block.

```modelica
if foo == bar then
  baz := bark;
  bam := bem;
else
  baz := 0;
  bam := 1;
end if;
```

## Vertical spacing

Beside indented elements described in the previous section, a newline or
two are inserted after every element, statement or equation.

Additionally, a single blank is inserted:

* before and after section keywords like `equation` or `protected`
* before and after the *composition* production
* before class-wide annotation in the *long-class-specifier*

## Line wrapping

### Function calls, arrays, matrices, modifications, lists

The main rule here is: be consistent. The following approach is applied:

1. If line is wrapped at any argument, then wrap at every argument.
2. If line is wrapped inside a nested construct, then wrap at every
   argument in every outer construct.

Indentation is increased accordingly to help visually identify the
scope.

```Modelica
// No wrap is fine
h = enthalpy_pT(p, T)

// Outer array is wrapped, but inner ones are kept intact
parameter Real A[2, 3] = {
  {1.0, 2.0, 3.0},
  {5.0, 6.0, 7.0}}

// In nested function call, if inner call is wrapped, outer call is
// wrapped as well
cp_a = specificHeat_pT(
  p = p_a,
  T = temperature_ph(
    p = p_a,
    h = h_a))

// But it is fine to wrap only outer call, keeping inner one intact
cp_b = specificHeat_pT(
  p = p_b,
  T = temperature_ph(p = p_b, h = h_b));

// Wrapped comprehensions are little bit special
{
  a[i] * b[i]
  for i in 1 : n}

// Output expression lists
(
  foo,
  bar) := baz(x, y)

// Import lists
import Modelica.Constants.{
  pi,
  inf};
```

### Expressions

Expressions (arithmetic, logical etc.) are handled in a very same way as
function calls and arrays. The difference is that instead of commas line
is wrapped before binary operators. Wrapping is applied to all operators
of the same precedence (as defined by the grammar). If wrap occured
inside higher precedence (inner) part, lower precedence (outer) parts
are wrapped as well.

```Modelica
// No wrap
foo = bar + baz

// Outer rule wrapped
foo := 2
  * (bar - baz)
  / 5

// Some inner rule wrapped
foo := -2
  + bam
    / baz ^ 2
    * bark
  - 33
```
