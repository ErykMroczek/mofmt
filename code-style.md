# *mofmt* code style

This document describes code style applied to Modelica files by *mofmt*.

## Maximum line length

There is no line length limit. There are plenty of limits already.

## Indentation

Indentation used by *mofmt* is 2 spaces per single level.

Modelica doesn't use indentation to define the scope, but it is
nonetheless useful to use it to enhance readability. Descriptions and annotation are good examples. Overall, *mofmt* increases indentation at:

* descriptions
* constraining clauses
* class sections (public, private, equation, algorithm etc.)
* elements, equations and statement lists
* enumerations
* at line wraps

## Vertical spacing

Maximum single blank line is allowed.

## Horizontal spacing

## Line wrapping

### Function calls, arrays, matrices, class modifications etc.

Main rule here is: be consistent.
To achieve this *mofmt* takes a following approach:

1. If line is wrapped at any argument, then wrap at every argument.
2. If line is wrapped inside nested construct, then wrap at every
   argument in every outer construct.

Indenation is increased to help visually identify the scope.

#### Example

```Modelica
// No wrap is fine!
h = enthalpy_pT(p, T);

// Outer array is wrapped, but inner ones are kept intact...
parameter Real A[2,3] = {
  {1.0, 2.0, 3.0},
  {5.0, 6.0, 7.0}};

// In nested function call, if inner call is wrapped, outer call is
// wrapped as well...
cp_a = specificHeat_pT(
  p = p_a,
  T = temperature_ph(
    p = p_a,
    h = h_a));

// But it is fine to wrap only outer call, keeping inner one intact!
cp_b = specificHeat_pT(
  p = p_b,
  T = temperature_ph(p = p_b, h = h_b));
```

#### Counterexample

```Modelica
// Ugly!
h = enthalpy_pT(p,
  T);

// Even more ugly!
cp_a = specificHeat_pT(p = p_a, T = temperature_ph(
  p = p_a,
  h = h_a));
```
