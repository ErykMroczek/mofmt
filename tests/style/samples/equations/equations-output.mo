/* Simple equations */
a = b * c;
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
