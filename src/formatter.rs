mod formatting;
mod printing;

use crate::parser::ModelicaCST;

/// Return string containing formatted Modelica code
pub fn pretty_print(cst: &ModelicaCST) -> String {
    let markers = formatting::format(cst);
    printing::print(cst, markers)
}
