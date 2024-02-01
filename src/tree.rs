use moparse::{ParsedModelica, SyntaxEvent, SyntaxKind, Token};

pub fn build_tree(parsed: ParsedModelica) -> Tree {
    let mut stack = Vec::new();
    let mut tokens = parsed.tokens.into_iter();
    let mut events = parsed.events;

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

    stack.pop().unwrap()
}

pub struct Tree {
    pub kind: SyntaxKind,
    pub children: Vec<Child>,
}

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
}

#[cfg(test)]
mod tests {
    use moparse::{parse, ModelicaToken};

    use super::*;

    fn tree(source: &str, start: SyntaxKind) -> Tree {
        let parsed = parse(source, start);
        build_tree(parsed)
    }

    #[test]
    fn test_empty_rules() {
        const SOURCE: &str = "annotation ()";
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
        assert_eq!(tree.start().kind, ModelicaToken::String);
        assert_eq!(tree.start().text, r#""descr""#);
        assert_eq!(tree.end().kind, ModelicaToken::RParen);
        assert_eq!(tree.end().end.pos, 21);
    }
}
