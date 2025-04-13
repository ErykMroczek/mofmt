use super::tokens::{Tokenized, Token, TokenID};
use super::parsing::{SyntaxEvent, SyntaxKind};

#[derive(Copy, Clone)]
pub struct TreeID(usize);

pub struct ModelicaCST {
    tokens: Tokenized,
    trees: Vec<Tree>,
}

impl ModelicaCST {
    pub fn new(tokens: Tokenized, mut events: Vec<SyntaxEvent>) -> Self {
        let mut trees = Vec::new();
        let mut stack: Vec<TreeID> = Vec::new();
        let mut all = tokens.all();
        for event in events {
            match event {
                SyntaxEvent::Enter(kind) => {
                    let tree = Tree::new(kind, stack.last().cloned());
                    stack.push(TreeID(trees.len()));
                    trees.push(tree);
                }
                SyntaxEvent::Exit => {
                    let id = stack.pop().unwrap();
                    if let Some(parent) = stack.last() {
                        trees[parent.0].push(Child::Tree(id));
                    }
                }
                SyntaxEvent::Advance(tokid) => {
                    todo!("advance {:?}", tokid);
                }
            }
        }
        ModelicaCST { tokens, trees }
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
    kind: SyntaxKind,
    parent: Option<TreeID>,
    children: Vec<Child>,
}

impl Tree {
    pub fn new(kind: SyntaxKind, parent: Option<TreeID>) -> Self {
        Tree {
            kind,
            parent,
            children: Vec::new(),
        }
    }

    pub fn push(&mut self, child: Child) {
        self.children.push(child);
    }

}

pub enum Child {
    Token(TokenID),
    Tree(TreeID),
}
