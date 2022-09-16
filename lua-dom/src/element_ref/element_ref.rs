use crate::node::{Element, Node};
use ego_tree::iter::{Edge, Traverse};
use ego_tree::NodeRef;
use html5ever::serialize::{serialize, SerializeOpts, TraversalScope};
use html5ever::tendril::StrTendril;

#[derive(Debug, Clone)]
pub struct ElementRef<'a> {
    pub(crate) node: NodeRef<'a, Node>,
}

impl<'a> std::ops::Deref for ElementRef<'a> {
    type Target = NodeRef<'a, Node>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl<'a> ElementRef<'a> {
    pub(crate) fn new(node: NodeRef<'a, Node>) -> Self {
        ElementRef { node }
    }

    /// Wraps a `NodeRef` only if it references a `Node::Element`.
    pub(crate) fn wrap(node: NodeRef<'a, Node>) -> Option<Self> {
        if node.value().is_element() {
            Some(ElementRef::new(node))
        } else {
            None
        }
    }

    /// Returns the `Element` referenced by `self`.
    pub(crate) fn value(&self) -> &'a Element {
        self.node.value().as_element().unwrap()
    }

    fn serialize(&self, traversal_scope: TraversalScope) -> String {
        let opts = SerializeOpts {
            scripting_enabled: false, // It's not clear what this does.
            traversal_scope,
            create_missing_parent: false,
        };
        let mut buf = Vec::new();
        serialize(&mut buf, self, opts).unwrap();
        String::from_utf8(buf).unwrap()
    }

    /// Returns the HTML of this element.
    pub fn html(&self) -> String {
        self.serialize(TraversalScope::IncludeNode)
    }

    /// Returns the inner HTML of this element.
    pub fn inner_html(&self) -> String {
        self.serialize(TraversalScope::ChildrenOnly(None))
    }

    /// Returns an iterator over descendent text nodes.
    pub fn text(&self) -> Text<'a> {
        Text {
            inner: self.traverse(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Text<'a> {
    inner: Traverse<'a, Node>,
}

impl<'a> Iterator for Text<'a> {
    type Item = &'a StrTendril;

    fn next(&mut self) -> Option<&'a StrTendril> {
        for edge in &mut self.inner {
            if let Edge::Open(node) = edge {
                if let Node::Text(ref text) = node.value() {
                    return Some(&text.text);
                }
            }
        }
        None
    }
}
