use ptree::item::TreeItem;
use ptree::style::Style;
use std::borrow::Cow;
use std::io::{Result, Write};

#[derive(Debug)]
pub struct TreeBuilder {
    nodes: Vec<Tree>,
}

// TODO: maybe get rid of unwrap
impl TreeBuilder {
    pub fn new(root: String) -> TreeBuilder {
        TreeBuilder {
            nodes: vec![Tree::new(root)],
        }
    }

    pub fn add_empty_child(self: &mut Self, file_stem: String) {
        self.nodes.last_mut().unwrap().add_child(file_stem);
    }

    pub fn begin_child(self: &mut Self, file_stem: String) {
        self.nodes.push(Tree {
            depth: self.nodes.len(),
            file_stem,
            children: vec![],
        });
    }

    pub fn end_child(self: &mut Self) {
        let child = self.nodes.pop().unwrap();
        self.nodes.last_mut().unwrap().children.push(child);
    }

    pub fn build(self: &mut Self) -> Tree {
        self.nodes.pop().unwrap()
    }
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Tree {
    file_stem: String,
    depth: usize,
    children: Vec<Tree>,
}

impl Tree {
    fn new(root: String) -> Tree {
        Tree {
            depth: 0,
            file_stem: root,
            children: vec![],
        }
    }

    fn add_child(self: &mut Self, file_stem: String) {
        self.children.push(Tree {
            depth: self.depth + 1,
            file_stem,
            children: vec![],
        });
    }

    pub fn sort(self: &mut Self) {
        for child in self.children.iter_mut() {
            child.sort();
        }
        self.children.sort_unstable();
    }
}

impl TreeItem for Tree {
    type Child = Self;
    fn write_self<W: Write>(&self, f: &mut W, style: &Style) -> Result<()> {
        write!(f, "{}", style.paint(&self.file_stem))
    }

    fn children(&self) -> Cow<[Self::Child]> {
        Cow::from(&self.children)
    }
}
