use crate::files;
use ptree::item::TreeItem;
use ptree::style::Style;
use std::borrow::Cow;
use std::io::{Result, Write};
use std::path::Path;
use walkdir::WalkDir;

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

pub fn build_tree(path: &Path, input: Option<String>) -> Result<Tree> {
    let mut builder =
        TreeBuilder::new(input.unwrap_or_else(|| String::from("Point Guard Password Store")));
    let walker = WalkDir::new(&path).into_iter();
    let mut depth = 1;
    for entry in walker.filter_entry(files::is_not_hidden) {
        let entry = match entry {
            Ok(entry) => entry,
            // TODO: should this return an error?
            Err(_e) => continue,
        };
        if entry.depth() == 0 {
            continue;
        }
        let path = entry.path();
        if entry.depth() == depth {
            if path.is_dir() {
                builder.begin_child(files::display_stem(path)?.to_string());
                depth += 1;
            } else {
                builder.add_empty_child(files::display_stem(path)?.to_string());
            }
        } else {
            builder.end_child();
            depth -= 1;
            if path.is_dir() {
                builder.begin_child(files::display_stem(path)?.to_string());
                depth += 1;
            } else {
                builder.add_empty_child(files::display_stem(path)?.to_string());
            }
        }
    }
    let mut root = builder.build();
    root.sort();
    Ok(root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn build_tree_test() {
        let tree = build_tree(&PathBuf::from("test-store-enc"), None).unwrap();
        assert_eq!(tree.file_stem, "Point Guard Password Store");
        assert_eq!(tree.children.len(), 6);
        assert_eq!(tree.children[0].file_stem, "empty");
        assert_eq!(tree.children[1].file_stem, "empty1");
        assert_eq!(tree.children[2].file_stem, "pointguard.dev");
        assert_eq!(tree.children[3].file_stem, "same");
        assert_eq!(tree.children[3].children.len(), 0);
        assert_eq!(tree.children[4].file_stem, "same");
        assert_eq!(tree.children[4].children.len(), 2);
        assert_eq!(tree.children[4].children[0].file_stem, "test");
        assert_eq!(tree.children[4].children[0].children.len(), 0);
    }
}
