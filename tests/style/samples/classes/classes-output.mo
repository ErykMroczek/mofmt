within;
function length
  "Return length of a vector"

  extends .Modelica.Icons.Function;

  input Complex v[:]
    "Vector";
  output Real result
    "Length";

algorithm

  result := sqrt(sum({v[i].re ^ 2 + v[i].im ^ 2 for i in 1:size(v, 1)}));

end length;

function h_pT
  "Enthalpy by pressure and temperature"

  input Real P;
  input Real T;

  output Real H;

protected

  RefdllHandle h = dllHandle;

  external "C" H = h_pT(
    h,
    P,
    T)
    annotation (Library = "doesn't matter");

end h_pT;

model Tank
  "Tank model"

  import Modelica.Constants.pi;

  // Geometry
  SIunits.Height level(
    stateSelect = StateSelect.prefer,
    start = level_start)
    "liquid level";
  SIunits.Volume V
    "Tank volume";
  parameter SIunits.Height height
    "Height of tank";
  parameter SIunits.Area crossArea
    "Area of tank";

  extends Modelica.Fluid.Vessels.BaseClasses.PartialLumpedVessel(
    break someUselessVar,
    anotherUselessVar = break, // Who need that?
    final fluidVolume = V,
    final fluidLevel = level,
    final fluidLevel_max = height,
    final vesselArea = crossArea,
    heatTransfer(surfaceAreas = {crossArea + 2 * sqrt(crossArea * pi) * level}));

  // Initialization
  parameter SI.Height level_start(min = 0) = 0.5 * height
    "Start value of tank level"
    annotation (Dialog(tab = "Initialization"));

protected

  final parameter SIunits.Height level_start = max(level_start, Modelica.Constants.eps);

equation

  // Total quantities
  V = crossArea * level
    "Volume of fluid";
  medium.p = p_ambient;

  // Source terms
  if Medium.singleState or energyDynamics == Types.Dynamics.SteadyState then
    Wb_flow = 0;
  else
    Wb_flow = -p_ambient * der(V);
  end if;

initial equation

  if massDynamics == Types.Dynamics.FixedInitial then
    level = level_start;
  elseif massDynamics == Types.Dynamics.SteadyStateInitial then
    der(level) = 0;
  end if;

  annotation (
    Documentation(
      info = "<html>
<p>Some documentation</p>
</html>"));

end Tank;

partial record 'Quoted "record"'

  import It.is.empty;

end 'Quoted "record"';

block Foo

  import Modelica.Units.SI;
  .Modelica.Blocks.Interfaces.BooleanInput b;

end Foo;

model TestComments

// Single-line comment

/*
Multi-line comment 1
Multi-line comment 2
*/

end TestComments;
