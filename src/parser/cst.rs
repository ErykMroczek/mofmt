use super::tokens::{Tokenized, Token, TokenID};
use super::parsing::{SyntaxEvent, SyntaxKind};

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

#[derive(Copy, Clone)]
pub struct TreeID(usize);

pub struct ModelicaCST {
    tokens: Tokenized,
    trees: Vec<Tree>,
}

impl ModelicaCST {
    pub fn new(tokens: Tokenized, events: Vec<SyntaxEvent>) -> Self {
        todo!();
    }

    pub fn root(&self) -> Option<TreeID> {
        if !self.trees.is_empty() {
            Some(TreeID(0))
        } else {
            None
        }
    }

    pub fn kind(&self, id: TreeID) -> SyntaxKind {
        self.trees[id.0].kind
    }

    pub fn parent(&self, id: TreeID) -> Option<TreeID> {
        self.trees[id.0].parent
    }

    pub fn children(&self, id: TreeID) -> &[Child] {
        self.trees[id.0].children.as_slice()
    }

    pub fn start(&self, id: TreeID) -> Token {
        match self.children(id).first().unwrap() {
            Child::Token(token) => self.tokens.get(*token),
            Child::Tree(tree) => self.start(*tree),
        }
    }
    pub fn end(&self, id: TreeID) -> Token {
        match self.children(id).last().unwrap() {
            Child::Token(token) => self.tokens.get(*token),
            Child::Tree(tree) => self.end(*tree),
        }
    }
    pub fn is_multiline(&self, id: TreeID) -> bool {
        let first = self.start(id);
        let last = self.end(id);
        first.start.line < last.end.line
    }
    pub fn contains(&self, id: TreeID, kind: SyntaxKind) -> bool {
        let mut contains = false;
        for child in self.children(id).iter() {
            if let Child::Tree(tree) = child {
                if self.kind(*tree) == kind {
                    contains = true;
                } else {
                    contains = self.contains(*tree, kind);
                }
            }
            if contains {
                return contains;
            }
        }
        contains
    }
}

struct Tree {
    pub kind: SyntaxKind,
    pub parent: Option<TreeID>,
    pub children: Vec<Child>,
}

pub enum Child {
    Token(TokenID),
    Tree(TreeID),
}
