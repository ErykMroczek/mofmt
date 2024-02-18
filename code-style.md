# Code style

This document describes a code style as applied by the *mofmt*.

## Maximum line length

There is no line length limit. There are plenty of limits already.

## Horizontal spacing

### Spaces

**mofmt** uses a single space as a horizontal separator between tokens.

Spaces are not inserted before:

- semicolons (`;`), except when preceded by `within`
- commas (`,`)
- parentheses and brackets (`()[]{}`)
- dots (`.`) in type specifiers and names

Spaces are not inserted after:

- dots (`.`) in type specifiers and names
- opening brackets (`([{`)

##### Example

```modelica
parameter .Foo.Bar[10] Baz(start = 0, max = 100);
```
### Operators

#### Unary

Two kinds of Modelica unary operators are treated differently:

- `not` operator is always followed by a space
- `+`, `-`, `.+`, `.-` are not followed by a space

##### Example

```modelica
-4
.-A
not is_off
```

#### Binary

All binary operators are surrounded with spaces.

##### Example

```modelica
(-2) * (3 - 1.76)
a <= b or x
1 : 10
```

### Assignments and equalities

`:=` and `=` tokens are surrounded with spaces.

##### Example

```modelica
foo = bar * 2
Real foo(min = 0, max = 2) = 1 / 2
foo := bar(x)
foo := if x < 2 then bar else baz
```

### Arrays, modifications, function calls, expression lists

Every comma (`,`) and semicolon (`;`) is followed by a space. This includes
discarded slots in *output-expression-list*.

##### Example

```modelica
{1.0, 2.0, 3.0}
(foo, , bar) := baz(x, y) 
[x, y; a, b]
```

## Vertical spacing

### Indentation

Indentation is two spaces per level.

### Automatic wrapping

Line is automatically wrapped before every:

- class definition
- element
- equation
- statement
- description string
- annotation
- constraining clause
- enumeration item

In case of:

- class definition
- element
- equation
- statement

wrap can be doubled so there is a single blank line instead.

#### Automatic blank lines

Section keywords:

- `public`
- `protected`
- `equation`
- `algorithm`
- `external`

are always preceded and followed by a blank line. Those keywords are placed
without indentation.

Blank lines are also inserted:

- at the beginning and end of the *composition* rule
- before the "lass-wide annotation in the *composition* rule

##### Example

```modelica
within SomePackage;
final model Foo
                                    // AUTO BLANK
  Real x;
  Real y;
                                    // AUTO BLANK
protected
                                    // AUTO BLANK
  Real z;
                                    // AUTO BLANK
equation
                                    // AUTO BLANK
  x + y = z;
  x * 2 = y;
                                    // OPTIONAL BLANK
  z = 3 ^ 2;
                                    // AUTO BLANK
  annotation ();
                                    // AUTO BLANK
end foo;
                                    // OPTIONAL BLANK
partial record Bar
                                    // AUTO BLANK
  parameter Boolean is_off = false;
                                    // AUTO BLANK
end Bar;
```

#### Descriptions, annotations and constraining clauses

Indentation is increased only one per element.

##### Example

```modelica
replaceable package Medium = Modelica.Media.R134a.R134a_ph
  constrainedby Modelica.Media.Interfaces.PartialMedium
  "Fluid medium"
  annotation(Dialog(tab = "General", group = "Medium"));
```

But not:

```modelica
replaceable package Medium = Modelica.Media.R134a.R134a_ph
  constrainedby Modelica.Media.Interfaces.PartialMedium
    "Fluid medium"
      annotation(Dialog(tab = "General", group = "Medium"));
```

#### Enumeration items

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

#### Loops and ifs

Indentation is increased once per block.

##### Example

```modelica
if foo == bar then
  baz := bark;
  bam := bem;
else
  if some_condition then
    baz := 0;
    bam := 1;
  else
    baz := bam;
  end if;
end if;
```

### Manual wrapping

#### Function calls, arrays, matrices, modifications, lists

The main rule here is: be consistent. The following approach is applied:

1. If a line was originally wrapped at any argument in the specific construct,
   then wrap at every argument in this construct.
2. If a line was originally wrapped inside a nested construct, then wrap at
   every argument in every outer construct.

Indentation is increased accordingly to help visually identify the
scope.

##### Example

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

// Import lists
import Modelica.Constants.{
  pi,
  inf};
```

#### Expressions

Expressions like arithmetic, logical etc. are handled in a different way,
because **mofmt** doesn't apply autowrapping when it detects a wrap in the
original file. Original wraps are preserved, and indentation is adjusted.
Indentation is increased only once per expression.

Other difference is that instead of commas line may be wrapped before binary
operators.

##### Example

```Modelica
// No wrap
foo := 2 * (bar - baz) / 5

// Single wrap
foo := 2
  * (bar - baz) / 5

// Two wraps
foo := 2
  * (bar - baz)
  / 5
```
