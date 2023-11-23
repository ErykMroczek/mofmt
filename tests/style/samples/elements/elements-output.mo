

  replaceable package Medium = .Modelica.Media.Interfaces.PartialMedium
    "Medium model"
    annotation (choicesAllMatching = true);

  parameter /* Unexpected comment! */ Integer n = 1
    "Discretization number"
    annotation (tab = "General");
  Modelica.Fluid.Interfaces.HeatPorts_a[n] heatPorts_a /* Where is b? */
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

  extends BaseClasses.PartialTwoPortFlow(
    final lengths = fill(length / n, n),
    final crossAreas = fill(crossArea, n),
    final dimensions = fill(4 * crossArea / perimeter, n),
    final roughnesses = fill(roughness, n),
    final dheights = height_ab * dxs);

  final Real dimensions = {
    4 * crossArea / perimeter,
    4 * crossArea / perimeter};

  final parameter Integer nFM = if useLumpedPressure then nFMLumped else nFMDistributed;
  final parameter Integer nFMLumped =
    if modelStructure == Types.ModelStructure.a_v_b then
      2 + a
    else
      1
    "Number of lumped flow models";

  final parameter SIunits.Height[n, m] height = {
    {pass_bottom[i] + j * (pass[i] - pass_bottom[i]) / n for i in 1:n}
    for j in 1:m};
  final parameter SIunits.Height[n, m] height = {
    {
      pass_bottom[i] + j * (pass[i] - pass_bottom[i]) / n
      for i in 1:n}
    for j in 1:m};
