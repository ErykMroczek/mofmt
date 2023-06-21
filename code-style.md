# Code style

This document describes code style as applied by *mofmt*.

## Maximum line length

There is no line length limit. There are plenty of limits already.

## Indentation

Indentation is 2 spaces per single level.

Modelica doesn't use indentation to define the scope, but it is
nonetheless useful to use it to enhance readability. Descriptions and
annotations are good examples. Overall, indentation shall be increased at:

* descriptions and annotations
* constraining clauses
* class sections (public, private, equation, algorithm etc.)
* elements, equations and statement lists
* enumeration items
* inside loops and if blocks

### Descriptions, annotations and constraining clauses

Indentation shall be increased only one time per element.

#### Example

```modelica
replaceable package Medium = Modelica.Media.R134a.R134a_ph
  constrainedby Modelica.Media.Interfaces.PartialMedium
  "Fluid medium"
  annotation(Dialog(tab = "General", group = "Medium"));
```

### Class sections and element lists

Indentation shall be increased before the section keyword and then
increased further inside section. In case of first public element list
(without explicit keyword) indentation simply shall be increased twice.

Indenting keywords is deliberate for it allows nice code wrapping in
text editors.


#### Example

```modelica
model BasicBoundary
  "Basic fluid boundary model"

    import Modelica.Fluid.Types.PortFlowDirection;
    import Modelica.Constants.inf;

    // SNAP

  protected

    parameter PortFlowDirection flowDirection = PortFlowDirection.Bidirectional
      "Allowed flow direction";

  equation

    // SNAP

end BasicBoundary;
```

### Enumeration items

Indentation shall be increased inside `enumeration()` and at every
description.

#### Example

```modelica
type BoundaryType = enumeration(
  Pressure
    "Pressure boundary",
  Flow
    "Flow boundary")
  "Enumeration of possible boundary types";
```

### Loops and if blocks

Indentation shall be increased before statement/equation list inside
loop or if-else branch block.

#### Example

```modelica
if boundaryType == BoundaryType.Pressure then
  medium.p = p_in;
else
  port.m_flow = -m_flow_in;
end if;
```

## Vertical spacing

Beside indented elements described in the previous section, a newline
shall be inserted after every semicolon. It is allowable to insert a
single blank line instead.

Additionally, single blank shall be inserted before:

* section keywords
* element, equations and statement lists inside class sections
* before the call to external function
* before class annotation
* before end clause signifying the class' scope

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
