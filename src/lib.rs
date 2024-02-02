use moparse::{SyntaxEvent, Token};

use self::printing::print;
use self::tree::build_tree;
use self::formatting::format;

pub fn pretty_print(tokens: Vec<Token>, comments: Vec<Token>, events: Vec<SyntaxEvent>) -> String {
    let tree = build_tree(tokens, events);
    let markers = format(tree, comments);
    print(markers)
}

mod formatting;
mod markers;
mod printing;
mod tree;
