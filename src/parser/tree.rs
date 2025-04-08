use super::{SyntaxEvent, SyntaxKind, Token};

/// Return the parse tree built from the collection returned from parser
pub fn build_tree(tokens: Vec<Token>, events: Vec<SyntaxEvent>) -> Tree {
    let mut stack = Vec::new();
    let mut tokens = tokens.into_iter();
    let mut events = events;

    assert!(matches!(events.pop(), Some(SyntaxEvent::Exit)));

    for event in events {
        match event {
            SyntaxEvent::Enter(kind) => stack.push(Tree::new(kind)),
            SyntaxEvent::Exit => {
                let tree = stack.pop().unwrap();
                if tree.len() > 0 {
                    stack.last_mut().unwrap().push(Child::Tree(tree));
                }
            }
            SyntaxEvent::Advance => {
                let token = tokens.next().unwrap();
                stack.last_mut().unwrap().push(Child::Token(token));
            }
        }
    }

    assert!(tokens.next().is_none());
    assert!(stack.len() == 1);

    stack.pop().unwrap()
}

#[derive(Debug)]
pub struct Tree {
    pub kind: SyntaxKind,
    pub children: Vec<Child>,
}

#[derive(Debug)]
pub enum Child {
    Token(Token),
    Tree(Tree),
}

impl Tree {
    pub fn new(kind: SyntaxKind) -> Self {
        Tree {
            kind,
            children: Vec::new(),
        }
    }

    pub fn push(&mut self, child: Child) {
        self.children.push(child);
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn start(&self) -> &Token {
        match self.children.first().unwrap() {
            Child::Token(token) => token,
            Child::Tree(tree) => tree.start(),
        }
    }

    pub fn end(&self) -> &Token {
        match self.children.last().unwrap() {
            Child::Token(token) => token,
            Child::Tree(tree) => tree.end(),
        }
    }

    pub fn is_multiline(&self) -> bool {
        let first = self.start();
        let last = self.end();
        first.start.line < last.end.line
    }

    pub fn contains(&self, kind: SyntaxKind) -> bool {
        let mut contains = false;
        for child in self.children.iter() {
            if let Child::Tree(tree) = child {
                if tree.kind == kind {
                    contains = true;
                } else {
                    contains = tree.contains(kind);
                }
            }
            if contains {
                return contains;
            }
        }
        contains
    }
}

#[cfg(test)]
mod tests {
    use super::super::{parse, TokenKind};

    use super::*;

    fn tree(source: &str, start: SyntaxKind) -> Tree {
        let parsed = parse("none", source, start);
        build_tree(parsed.tokens, parsed.events)
    }

    #[test]
    fn test_empty_rules() {
        const SOURCE: &str = r#"annotation (choices(choice = 0 "Foo", choice = 1 "Bar"))"#;
        let tree = tree(SOURCE, SyntaxKind::Description);
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn test_start_and_end() {
        const SOURCE: &str = r#""descr" annotation ()"#;
        let tree = tree(SOURCE, SyntaxKind::Description);
        assert_eq!(tree.len(), 2);
        if let Child::Tree(annotation) = tree.children.get(1).unwrap() {
            assert_eq!(annotation.len(), 2);
        } else {
            panic!("annotation not found");
        }
        assert_eq!(tree.start().kind, TokenKind::String);
        assert_eq!(tree.start().text, r#""descr""#);
        assert_eq!(tree.end().kind, TokenKind::RParen);
        assert_eq!(tree.end().end.offset, 21);
    }
}
