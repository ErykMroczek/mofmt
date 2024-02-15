# Vertical spacing

## Indentation

Indentation is two spaces per level.

## Automatic wrapping

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

### Automatic blank lines

Section keywords:

- `public`
- `protected`
- `equation`
- `algorithm`
- `external`

are always preceeded and followed by a blank line. Those keywords are placed
without indentation.

Blank lines are also inserted:

- at the beginning and end of the *composition* rule
- before the "class" annotation in the *composition* rule

#### Example

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

## Descriptions, annotations and constraining clauses

Indentation is increased only one per element.

#### Example

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

## Enumeration items

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
  if some_condition then
    baz := 0;
    bam := 1;
  else
    baz := bam;
  end if;
end if;
```

## Preserved wrapping

TODO
