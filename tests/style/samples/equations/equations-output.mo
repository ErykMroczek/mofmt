/* Simple equations */
a = -b * c;
x * (-y) = 2 ^ z / (m - n);

/* Wrapped equations */

Q_flow = alpha * surfaceArea
  * (T_a - T_b);
volume = height * pi
  * diameter ^ 2 / 4;

/* If-else blocks */

if wetConditions then
  // Latent heat flow is present
  Q_flow_latent = Q_flow - Q_flow_sensible;
else
  Q_flow_latent = 0; // Latent heat flow is absent
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

for i in 1:n loop
  h[i] = cp[i] * T[i];
end for;

for i in 1:passes loop
  for j in 1:n loop
    connect(refrigerant.heatPort[i], wallWithConv.heatPorts_a_vap[i, k]);
  end for;
end for;

/* Softbreak after linewrap */

a_nominal = Z_factor * func_a(b_nominal, c)
  * Av * Y * func_b(
    x = x_nominal * p_nominal * d_nominal,
    x_small = d_limit * d_small)
  "Description";

volumes = {
  diameter[i] * diameter[i]
    * 0.25 * length[i]
  for i in 1:n};

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

points = {
  {-98, -60},
  {
    -64,
    -60},
  {-64, -63.4667},
  {-27.1111, -63.4667}};
