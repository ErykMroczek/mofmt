within Bark.Baark;
model Foo "Foo description"

import Modelica.Constants;
import Modelica.SI;

    extends Bar(a=0, b= 1,
    c   = if x     then - 1   else 1 ,
    break uselessVar, anotherUselessVar = break, // USELESS!
    final d = true);

    // Bunch of elements
replaceable package Medium = .Modelica.Media.Interfaces.PartialMedium
      "Medium model" annotation (choicesAllMatching=true);


    final /* Unexpected comment! */ parameter Real foo(start = 0, min = -1000) = if d then Constants.inf
    else if y then 5*2+
    (2^4)
    +111 else 66
    "Parameter with some fairly complex modification expression"
    annotation (Dialog(tab = "General", group="Bunch"));

    SI.Length[3] 'length of "whatever"'(start = 0,
    min = -1, max = 1, nominal=0) = {  1* 0.25 for i in 1  :  3};
protected
/* It is too valuable to be public */
 Integer n_protected;
public
.Modelica.Blocks.Interfaces.BooleanInput b
annotation (Placement(
    transformation(
    extent={{-20,-10},{20,10}},rotation=90,
    origin={-98,4}), iconTransformation(extent={{-40,-10},{40,10}},
    rotation=90,
    origin={-68,0})));


initial equation
if foo == Types.Dynamics.FixedInitial then
    bar = bar_start;
   elseif foo == Types.Dynamics.SteadyStateInitial then
der(bar) = 0;
        end if;

    equation
a = -b*c;
 x *( - y) = 2^z/ (m-n);

A = toString([2.12,-4.34;-2.56, -1.67]);

/* Wrapped equations */

Q_flow = alpha * surfaceArea*
(T_a - T_b);
volume =height*pi
    * diameter^2/4;


/* If-else blocks */

if wetConditions then

 // Latent heat flow is present
  Q_flow_latent = Q_flow - Q_flow_sensible;

else
  Q_flow_latent=0; // Latent heat flow is absent
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

/* Softbreak after linewrap */

a_nominal = Z_factor * func_a(foo = b_nominal, bar = c)
* Av * Y * func_b(
x_nominal * p_nominal * d_nominal,
x_small = d_limit * d_small)
"Description";

volumes = {diameter[i] * diameter[i]
* 0.25 * length[i] for i in 1:n};

/* Matrices */

extent = [-10, 110; 10, 90];
extent = [
    -10, 110; 10, 90];
 a[:,:]=[1,1,1,1,1; 2,2,2,
  2,2];
m[:,:] = Math.Matrices.sort(
    [
      Math.Vectors.length(v1),
      Math.Vectors.length(v2);
      Math.Vectors.length(v1 + v2),
      Math.Vectors.length(v2 - v1)
    ]);

A = toString([2.12,-4.34;
    -2.56, -1.67]);

points={{-98,-60},{-64,
          -60},{-64,-63.4667},{-27.1111,-63.4667}};

end Foo;
