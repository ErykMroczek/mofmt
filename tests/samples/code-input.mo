// Some code samples
// to check if the applied style is correct
within foo.bar;

        // Let's check some class with a quoted identifier
final
encapsulated partial operator record
'Quoted record "whatever"'  "Quoted record"

/* Few imports */
import Foo.Bar "Foo import"

        annotation(ignore     = false);
import Baz = Foo.Baz;
import Bar.*;
import Bark.{Foo,Bar,       Baz};
import Ark.{
  Bar,       Baz};

// Some extension
     extends .Bark.Bark()
annotation();

// Now some other class specifiers!


inner     outer record Inner=der(.Foo.Baz, Foo,bar) "Der?";


redeclare final inner package Foo = input
Foo.Bar[1,2](x=2+3) "Foo";
protected    // Now protected section

flow constant Foo.Baz Bar=2, Baar; parameter Real Foo(start=2, fixed=false),
Bar if false;
annotation(Icon());
end 'Quoted record';

// Now some model!

final partial model FooModel
      "Foo model"
     extends .Bark.Bark(break connect(a.b  ,c),
     anotherUselessVar = break);

// Some conditional expressions
parameter Real[1] foo = if bar then 2 elseif baz then 3 else 4;
Integer[1, 3, 4] bar =if true then
1 elseif baz<2 then 3 else 2;

protected

String A = toString([2.12,-4.34;-2.56,    -    1.67] );
public

redeclare Foo x(y=z) = Bar;

initial equation
  if foo == Types.Dynamics.FixedInitial then
bar = bar_start; elseif foo == Types.Dynamics.SteadyStateInitial then
    der(bar) = 0; end if;
equation
  a =     -b*c   "Equation";
  x* ( -y) =2^z / (m- n);
  foo =if bar then 0 else
  3;

  foo = bar(x,y=y,z=baz(
      a,
      b));

/* If-else blocks */

  if foo then
    // comment
    bar = baz * bar;
else
    bar = 0;// another
  end if;
if a<b then
if x < 0 then



                m = n;
                elseif x <y then
                m = 2*n;
                else
                m=0;
                end if;
else
  m = n^2;
end if;

/* For loop */

for i in 1:n loop
      h[i]=c[i] * T[i];
  end for;

for i in 1:m loop
                for j in 1:n loop

connect(a[i], b[i, k]);

end for;
end for;
end FooModel;