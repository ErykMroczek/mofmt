mod formatting;
mod printing;

use crate::parser::ModelicaCST;

impl ModelicaCST {
    /// Return string containing formatted Modelica code represented by the CST.
    pub fn pretty_print(&self) -> String {
        let markers = formatting::format(self);
        printing::print(self, markers)
    }
}
