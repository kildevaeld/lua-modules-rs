use crate::{
    matcher::{MatchScope, Matcher, Matches},
    node::Node,
};
use ego_tree::{NodeId, Tree};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Selection {
    pub(crate) nodes: Vec<NodeId>,
    pub(crate) tree: Rc<Tree<Node>>,
}

impl Selection {
    pub(crate) fn new(tree: Rc<Tree<Node>>, nodes: Vec<NodeId>) -> Selection {
        Selection { tree, nodes }
    }

    pub fn select<S: AsRef<str>>(&self, sel: S) -> Selection {
        let matcher = Matcher::new(sel.as_ref()).expect("Invalid CSS selector");

        Selection::new(
            self.tree.clone(),
            Matches::from_list(
                self.nodes.iter().filter_map(|id| self.tree.get(*id)),
                matcher.clone(),
                MatchScope::IncludeNode,
            )
            .collect(),
        )
    }
}
