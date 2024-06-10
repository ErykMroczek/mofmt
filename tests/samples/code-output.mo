// Some code samples
// to check if the applied style is correct
within foo.bar;

// Let's check some class with a quoted identifier
final encapsulated partial operator record 'Quoted record "whatever"'
  "Quoted record"

  /* Few imports */
  import Foo.Bar
    "Foo import"
    annotation (ignore = false);
  import Baz = Foo.Baz;
  import Bar.*;
  import Bark.{Foo, Bar, Baz};
  import Ark.{
    Bar,
    Baz};

  // Some extension
  extends .Bark.Bark()
    annotation ();

  // Now some other class specifiers!

  inner outer record Inner = der(.Foo.Baz, Foo, bar)
    "Der?";

  redeclare final inner package Foo = input Foo.Bar[1, 2](x = 2 + 3)
    "Foo";

protected // Now protected section

  flow constant Foo.Baz Bar = 2, Baar;
  parameter Real
    Foo(start = 2, fixed = false),
    Bar if false;

  annotation (Icon());

end 'Quoted record "whatever"';

// Now some model!

final partial model FooModel
  "Foo model"

  extends .Bark.Bark(
    break connect(a.b, c),
    break Baz,
    anotherUselessVar = break);

  // Some conditional expressions
  parameter Real[1] foo = if bar then 2 elseif baz then 3 else 4;
  Integer[1, 3, 4] bar =
    if true then
      1
    elseif baz < 2 then
      3
    else
      2;

  Real smallest = (Modelica.Math.Vectors.sort({4, 2, 5, 1}))[1];

protected

  // Here we have two comments

  /*
  And they are separated
  with a single blank line
  */
  String A = toString([2.12, -4.34; -2.56, -1.67]);
  SI.Length[3] 'length of "whatever"'(
    start = 0,
    min = -1,
    max = 1,
    nominal = 0) = {1 * 0.25 for i in 1 : 3};

public

  redeclare Foo x(y = z) = Bar
    annotation (
      Placement(
        transformation(
          extent = {{-20, -10}, {20, 10}},
          rotation = 90,
          origin = {-98, 4}),
        iconTransformation(
          extent = {{-40, -10}, {40, 10}},
          rotation = 90,
          origin = {-68, 0})));

initial equation

  if foo == Types.Dynamics.FixedInitial then
    bar = bar_start;
  elseif foo == Types.Dynamics.SteadyStateInitial then
    der(bar) = 0;
  end if;

equation

  a = -b * c
    "Equation";
  x * (-y) = 2 ^ z / (m - n);
  foo =
    if bar then
      0
    else
      3;

  foo = bar(
    x,
    y = y,
    z = baz(
      a,
      b));

  /* If-else blocks */

  if foo then
    // comment
    bar = baz * bar;
  else
    bar = 0; // another
  end if;
  if a < b then
    if x < 0 then
      m = n;
    elseif x < y then
      m = 2 * n;
    else
      m = 0;
    end if;
  else
    m = n ^ 2;
  end if;

  /* For loop */

  for i in 1 : n loop
    h[i] = c[i] * T[i];
  end for;

  for i in 1 : m loop
    for j in 1 : n loop
      connect(a[i], b[i, k]);
    end for;
  end for;

  /* Wrapped equations */

  foo = foo * pi
    * bar ^ 2 / 4;
  foo = bar * baz * (bark
    - bam);

  /* Nested wrapping */

  a_nominal = Z_factor * func_a(foo = b_nominal, bar = c)
    * Av * Y * func_b(
      x_nominal * p_nominal * d_nominal,
      x_small = d_limit
        * d_small)
    "Description";

  /* Arrays */

  volumes = {
    diameter[i] * diameter[i]
      * 0.25 * length[i]
    for i in 1 : n};
  foo = sum(
    bar[i] - baz[i]
    for i in 1 : 10);
  points = {
    {-98, -60},
    {
      -64,
      -60},
    {-64, -63.4667},
    {-27.1111, -63.4667}};
  foo = (bar - 1) ^ 3
    * (1 - (baz + 12) / (10 * (baz + 1)) + sum(
      (1 - 2 * (foo - k) / ((foo + 1) * k * (k + 1))) * 1 / (k - 1) * ((bar - 1) / r) ^ (k - 3)
      for k in 1 : 42));
  /* Matrices */

  extent = [-10, 110; 10, 90];
  extent = [
    -10, 110;
    10, 90];
  a[:, :] = [
    1, 1, 1, 1, 1;
    2,
    2,
    2,
    2,
    2];
  m[:, :] = Math.Matrices.sort(
    [
      Math.Vectors.length(v1),
      Math.Vectors.length(v2);
      Math.Vectors.length(v1 + v2),
      Math.Vectors.length(v2 - v1)]);

end FooModel;
// And now functions!
final pure function Foo
  "Return something"

  extends .Modelica.Icons.Function;

  input Integer a
    "Aaa";
  output Real result
    "Length";

protected

  Real b
    "Bbb";
  parameter Integer control = 0
    annotation (
      choices(
        choice = 0
          "Foo",
        choice = 1
          "Bar"));

algorithm

  (A, B, C) := foo.bar.baz(a);
  (D, , E) := foo.bar.baz(b);
  ( , G, (H, J)) := foo.bar.baz(c);

  foo := {
    {
      bar[i] + j
        * (baz[i] - ber[i]) / n
      for i in 1 : n}
    for j in 1 : m};
  bar := {
    {
      foo[i] + j * (baz[i] - foo[i]) / n
      for i in 1 : n}
    for j in 1 : m};

  baz := aaa
    + bbb * (ccc + ddd
    - eee)
    - fff * ggg;

external "C"

  foo[1].bar[2] = baz(
    x,
    y,
    z)
    annotation (Library = "doesn't matter");

  annotation (smoothOrder = 2);

end Foo;
impure function Baz
  "To check annotations after empty sections"

algorithm

  annotation ();

end Baz;
function Baz
  "To check annotations after decriptions"

  annotation ();

end Baz;
function Extern
  "To check annotations to external functions"

external
    annotation ();

  annotation ();

end Extern;
partial function Bar
  "Just in case"

initial algorithm

  x := y;

  /* If statement */
  foo :=
    if a == 1 then
      bar
    else
      baz
    "What is this about?";

  /* Multiline statements */
  y := u1 > 0
    and u2 > 0
    and u3 > 0
    and u4 > 0;

  y := u1 > 0
    or u2 > 0
    or u3 > 0
    or u4 > 0;

  Modelica.Utilities.Streams.print(
    "foo" + "bar"
      + "baz");

end Bar;

// And some enums

type Foo = enumeration(Foo1, foo2)
  "foo enum";
type Foo = enumeration(
  Foo1,
  foo2)
  "foo enum";
type Foo = enumeration(
  Foo1
    "foo1",
  foo2)
  "foo enum with description of one element";