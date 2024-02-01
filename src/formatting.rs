use crate::{markers::Marker, tree::Child, tree::Tree};
use moparse::*;

struct Formatter {
    markers: Vec<Marker>,
    groups: Vec<bool>,
    rules: Vec<SyntaxKind>,
}

impl Formatter {
    fn break_or_space(&mut self, is_multiline: bool) {
        if is_multiline {
            self.markers.push(Marker::Break);
        } else {
            self.markers.push(Marker::Space);
        }
    }
}

fn class_modification(f: &mut Formatter, tree: Tree) {}

fn expression(f: &mut Formatter, tree: Tree) {}

fn type_specifier(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(t) => name(f, t),
            Child::Token(tok) => f.markers.push(Marker::Token(tok.text)),
        }
    }
}

fn name(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(_) => unreachable!(),
            Child::Token(tok) => f.markers.push(Marker::Token(tok.text)),
        }
    }
}

fn array_subscripts(f: &mut Formatter, tree: Tree) {
    f.markers.push(Marker::Indent);
    let is_multiline = tree.is_multiline();
    for child in tree.children {
        match child {
            Child::Tree(t) => match t.kind {
                SyntaxKind::Subscript => subscript(f, t),
                _ => unreachable!(),
            },
            Child::Token(tok) => {
                f.markers.push(Marker::Token(tok.text));
                match tok.kind {
                    ModelicaToken::LBracket => {
                        if is_multiline {
                            f.markers.push(Marker::Break);
                        }
                    }
                    ModelicaToken::Comma => f.break_or_space(is_multiline),
                    _ => (),
                }
            }
        }
    }
    f.markers.push(Marker::Dedent);
}

fn subscript(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Tree(t) => match t.kind {
                SyntaxKind::Expression => expression(f, t),
                _ => unreachable!(),
            },
            Child::Token(tok) => f.markers.push(Marker::Token(tok.text)),
        }
    }
}

fn description(f: &mut Formatter, tree: Tree) {
    for (idx, child) in tree.children.into_iter().enumerate() {
        match child {
            Child::Tree(t) => match t.kind {
                SyntaxKind::DescriptionString => description_string(f, t),
                SyntaxKind::AnnotationClause => {
                    if idx > 0 {
                        f.markers.push(Marker::Break);
                    }
                    annotation_clause(f, t);
                }
                _ => unreachable!(),
            },
            Child::Token(_) => unreachable!(),
        }
    }
}

fn description_string(f: &mut Formatter, tree: Tree) {
    let is_multiline = tree.is_multiline();
    f.markers.push(Marker::Indent);
    for child in tree.children {
        match child {
            Child::Token(token) => match token.kind {
                ModelicaToken::Plus => {
                    f.break_or_space(is_multiline);
                    f.markers.push(Marker::Token(token.text));
                    f.markers.push(Marker::Space);
                }
                ModelicaToken::String => f.markers.push(Marker::Token(token.text)),
                _ => unreachable!(),
            },
            Child::Tree(_) => unreachable!(),
        }
    }
    f.markers.push(Marker::Dedent);
}

fn annotation_clause(f: &mut Formatter, tree: Tree) {
    for child in tree.children {
        match child {
            Child::Token(token) => {
                f.markers.push(Marker::Token(token.text));
                f.markers.push(Marker::Space);
            }
            Child::Tree(t) => {
                class_modification(f, t);
            }
        }
    }
}
