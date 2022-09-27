use super::shared::{StringList, StringRef};
use crate::element_ref::ElementRef;
use crate::{
    matcher::{MatchScope, Matcher, Matches},
    node::Node,
    selection::Selection,
};
use ego_tree::{NodeId, Tree};
use mlua::{MetaMethod, UserData};
use std::rc::Rc;

pub struct Element {
    pub tree: Rc<Tree<Node>>,
    pub node_id: NodeId,
}

impl Element {
    pub fn element_ref<'a>(&'a self) -> ElementRef<'a> {
        let node = unsafe { self.tree.get_unchecked(self.node_id) };
        ElementRef { node }
    }
}

impl UserData for Element {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("type", |_, this| Ok(this.element_ref().node_type()));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            Ok(this.element_ref().html())
        });

        methods.add_method("attr", |_, this, args: (mlua::String,)| {
            Ok(this
                .element_ref()
                .attr(args.0.to_str()?)
                .cloned()
                .map(StringRef))
        });

        methods.add_method("select", |_, this, args: (mlua::String,)| {
            let matcher = Matcher::new(args.0.to_str()?).expect("Invalid CSS selector");
            let root = this.tree.get(this.node_id).unwrap();
            Ok(Selection::new(
                this.tree.clone(),
                Matches::from_one(root, matcher.clone(), MatchScope::IncludeNode).collect(),
            ))
        });

        methods.add_method("classes", |_, this, _: ()| {
            let node = this.tree.get(this.node_id).unwrap();

            if let Some(element) = node.value().as_element() {
                let classes = element
                    .classes()
                    .map(|args| StringRef(args.to_string()))
                    .collect();
                return Ok(StringList(classes));
            }

            Ok(StringList(vec![]))
        });

        methods.add_method("text", |_, this, _: ()| {
            let node = this.tree.get(this.node_id).unwrap();

            let text = ElementRef::new(node)
                .text()
                .map(|s| StringRef(s.clone()))
                .collect::<Vec<_>>();

            Ok(StringList(text))
        });

        methods.add_method("html", |_, this, _: ()| Ok(this.element_ref().html()));

        methods.add_method("innerHtml", |_, this, _: ()| {
            Ok(this.element_ref().inner_html())
        });
    }
}
