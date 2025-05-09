use super::parsing::{SyntaxEvent, SyntaxKind};
use super::tokens::{TokenID, Tokens};

#[derive(Copy, Clone)]
/// A unique identifier for a tree node.
/// It acts as a opaque pointer to the node in the CST.
pub struct TreeID(usize);

//// A concrete syntax tree (CST) for Modelica.
/// The CST is a tree structure that represents the syntactic structure of the source code.
/// 
/// CST API is opaque and does not expose the internal structure of the tree.
/// It provides methods to navigate the tree and access its nodes.
pub struct ModelicaCST {
    tokens: Tokens,
    trees: Vec<Tree>,
    errors: Vec<Error>,
}

impl ModelicaCST {
    pub(super) fn new(tokens: Tokens, events: Vec<SyntaxEvent>) -> Self {
        let mut trees = Vec::new();
        let mut stack: Vec<TreeID> = Vec::new();
        let mut errors = Vec::new();
        for event in events {
            match event {
                SyntaxEvent::Enter(kind) => {
                    let tree = Tree::new(kind, stack.last().cloned());
                    stack.push(TreeID(trees.len()));
                    trees.push(tree);
                }
                SyntaxEvent::Exit => {
                    let id = stack.pop().unwrap();
                    if !trees[id.0].children.is_empty() {
                        if let Some(parent) = stack.last() {
                            trees[parent.0].push(Child::Tree(id));
                        }
                    }
                }
                SyntaxEvent::Advance(id) => {
                    trees[stack.last().unwrap().0].push(Child::Token(id));
                }
                SyntaxEvent::Error(i, msg) => {
                    errors.push(Error {
                        msg,
                        token: i,
                    });
                }
            }
        }
        ModelicaCST {
            tokens,
            trees,
            errors,
        }
    }

    /// Return the root node of the CST.
    pub fn root(&self) -> Option<TreeID> {
        if !self.trees.is_empty() {
            Some(TreeID(0))
        } else {
            None
        }
    }

    /// Return syntax error messages
    pub fn errors(&self) -> Vec<String> {
        self.errors
            .iter()
            .map(|e| {

                let pos = if e.token == self.tokens.last() {
                    self.tokens.end(e.token)
                } else {
                    self.tokens.start(e.token)
                };
                format!(
                    "{}:{}:{}: {}",
                    self.tokens().source(),
                    pos.line,
                    pos.col,
                    e.msg
                )
            })
            .collect()
    }

    /// Return the reference to the tokens API
    pub fn tokens(&self) -> &Tokens {
        &self.tokens
    }

    /// Return a syntax rule represented by the node of the given id.
    pub fn kind(&self, id: TreeID) -> SyntaxKind {
        self.trees[id.0].kind
    }

    /// Return the parent of the node of the given id.
    pub fn parent(&self, id: TreeID) -> Option<TreeID> {
        self.trees[id.0].parent
    }

    /// Return the children of the node of the given id.
    pub fn children(&self, id: TreeID) -> &[Child] {
        self.trees[id.0].children.as_slice()
    }

    /// Return a first token constituting the node of the given id.
    pub fn start(&self, id: TreeID) -> TokenID {
        match self.children(id).first().unwrap() {
            Child::Token(token) => *token,
            Child::Tree(tree) => self.start(*tree),
        }
    }

    /// Return a last token constituting the node of the given id.
    pub fn end(&self, id: TreeID) -> TokenID {
        match self.children(id).last().unwrap() {
            Child::Token(token) => *token,
            Child::Tree(tree) => self.end(*tree),
        }
    }

    /// Return `true` if the node of the given id is empty (has no children).
    pub fn is_empty(&self, id: TreeID) -> bool {
        self.children(id).is_empty()
    }

    /// Return `true` if the node spans multiple lines.
    pub fn is_multiline(&self, id: TreeID) -> bool {
        let first = self.tokens.get(self.start(id));
        let last = self.tokens.get(self.end(id));
        first.start.line < last.end.line
    }

    /// Return `true` if the node of the given id contains a child node of the given kind.
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

struct Error {
    msg: String,
    token: TokenID,
}

struct Tree {
    kind: SyntaxKind,
    parent: Option<TreeID>,
    children: Vec<Child>,
}

impl Tree {
    fn new(kind: SyntaxKind, parent: Option<TreeID>) -> Self {
        Tree {
            kind,
            parent,
            children: Vec::new(),
        }
    }

    fn push(&mut self, child: Child) {
        self.children.push(child);
    }
}

/// A child of a tree node.
/// It can be either a token or a tree node.
pub enum Child {
    Token(TokenID),
    Tree(TreeID),
}
