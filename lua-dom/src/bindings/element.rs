use crate::{element_ref::ElementRef, node::Node};
use ego_tree::{NodeId, Tree};
use mlua::UserData;
use std::rc::Rc;

use super::shared::{StringList, StringRef};

pub struct Element {
    pub tree: Rc<Tree<Node>>,
    pub node_id: NodeId,
}

impl UserData for Element {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("type", |_, this| {
            let node = this.tree.get(this.node_id).unwrap();
            match node.value() {
                Node::Comment(_) => Ok("comment".to_string()),
                Node::Doctype(_) => Ok("doctype".to_string()),
                Node::Element(el) => Ok(el.name().to_string()),
                Node::Fragment => Ok("fragment".to_owned()),
                Node::Text(_) => Ok("text".to_owned()),
                _ => Ok("".to_string()),
            }
        });
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("attr", |_, this, args: (mlua::String,)| {
            let node = this.tree.get(this.node_id).unwrap();

            if let Some(element) = node.value().as_element() {
                return Ok(element.attr(args.0.to_str()?).map(|m| StringRef(m.clone())));
            }

            Ok(None)
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

        methods.add_method("html", |_, this, _: ()| {
            let node = this.tree.get(this.node_id).unwrap();

            let text = ElementRef::new(node).html();

            Ok(text)
        });

        methods.add_method("innerHtml", |_, this, _: ()| {
            let node = this.tree.get(this.node_id).unwrap();

            let text = ElementRef::new(node).inner_html();

            Ok(text)
        });
    }
}
