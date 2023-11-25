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
    pub(crate) fn value(&self) -> Option<&'a Element> {
        self.node.value().as_element()
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

    pub fn node_type(&self) -> String {
        match self.node.value() {
            Node::Comment(_) => "comment".to_string(),
            Node::Doctype(_) => "doctype".to_string(),
            Node::Element(el) => el.name().to_string(),
            Node::Fragment => "fragment".to_owned(),
            Node::Text(_) => "text".to_owned(),
            _ => "".to_string(),
        }
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

    pub fn attr(&self, str: impl AsRef<str>) -> Option<&StrTendril> {
        if let Some(element) = self.node.value().as_element() {
            element.attr(str.as_ref())
        } else {
            None
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
