# Horizontal spacing

## Spaces

**mofmt** uses a single space as a horizontal separator between tokens.

Spaces are not inserted before:

- semicolons (`;`)
- commas (`,`)
- parentheses and brackets (`()[]{}`)
- dots (`.`) in type specifiers and names

Spaces are not inserted after:

- dots (`.`) in type specifiers and names
- opening brackets (`([{`)

#### Example

```modelica
parameter .Foo.Bar[10] Baz(start = 0, max = 100);
```

## Operators

### Unary

Two kinds of Modelica unary operators are treated differently:

- `not` operator is always followed by a space
- `+`, `-`, `.+`, `.-` are not followed by a space

#### Example

```modelica
-4
.-A
not is_off
```

### Binary

All binary operators are surrounded with spaces.

#### Example

```modelica
(-2) * (3 - 1.76)
a <= b or x
1 : 10
```

## Assignments and equalities

`:=` and `=` tokens are surrounded with spaces.

#### Example

```modelica
foo = bar * 2
Real foo(min = 0, max = 2) = 1 / 2
foo := bar(x)
foo := if x < 2 then bar else baz
```

## Arrays, modifications, function calls, expression lists

Every comma (`,`) and semicolon (`;`) is followed by a space. This includes
discarded slots in *output-expression-list*.

#### Example

```modelica
{1.0, 2.0, 3.0}
(foo, , bar) := baz(x, y) 
[x, y; a, b]
```
