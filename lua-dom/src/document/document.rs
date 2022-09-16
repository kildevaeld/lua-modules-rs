use std::rc::Rc;

use ego_tree::Tree;

use crate::{
    matcher::{MatchScope, Matcher, Matches},
    node::Node,
    selection::Selection,
};

#[derive(Debug, Clone)]
pub struct Document {
    pub(crate) tree: Rc<Tree<Node>>,
}

impl Document {
    pub fn select(&self, sel: &str) -> Selection {
        let matcher = Matcher::new(sel).expect("Invalid CSS selector");
        let root = self.tree.root();
        Selection::new(
            self.tree.clone(),
            Matches::from_one(root, matcher.clone(), MatchScope::IncludeNode).collect(),
        )
    }
}
