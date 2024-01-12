within;
final encapsulated function Foo
  "Return something"

  extends .Modelica.Icons.Function;

  input Integer a
    "Aaa";
  output Real result
    "Length";

protected

  Real b
    "Bbb";

algorithm

  (A, B, C) := foo.bar.baz(a);
  (D,, E) := foo.bar.baz(b);
  (
    F,
    G,
    (
      H,
      J)) := foo.bar.baz(c);

  foo := {
    {
      bar[i] + j
        * (baz[i] - ber[i]) / n
        for i in 1:n}
    for j in 1:m};
  bar := {
    {
      foo[i] + j * (baz[i] - foo[i]) / n
      for i in 1:n}
    for j in 1:m};

  baz := aaa
    + bbb * (
      ccc + ddd
        - eee)
    - fff * ggg;

external "C" foo[1].bar[2] = baz(x, y, z)
  annotation (Library = "doesn't matter");

  annotation (smoothOrder = 2);

end Foo;

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

  for i in 1:5 loop
    foo[i] := bar[i] ^ 2
      * (
        4 - bar[i] / (6 * bar[i] + 6)
          + sum(
            {
              (
                4 - 2 * bar[i] / ((bar[i] + 1) * (k + 3))
                  * ((bar[i] - 1) / bar[i]) ^ k)
              for k in 1:10}));
  end for;

end Bar;
