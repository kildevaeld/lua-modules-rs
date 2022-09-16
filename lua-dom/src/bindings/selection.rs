use crate::element_ref::ElementRef;
use crate::Selection;
use mlua::{MetaMethod, ToLuaMulti, UserData, Value};

use super::element::Element;
use super::shared::{StringList, StringRef};

impl UserData for Selection {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(_fields: &mut F) {}

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        #[cfg(any(
            feature = "lua54",
            feature = "lua53",
            feature = "lua52",
            feature = "luajit52"
        ))]
        methods.add_meta_method(MetaMethod::Pairs, |lua, data, ()| {
            let stateless_iter = lua.create_function(|lua, (data, i): (Selection, i64)| {
                let i = i + 1;
                if (i as usize) <= data.nodes.len() {
                    let node_id = data.nodes[(i - 1) as usize];

                    return Ok((
                        i,
                        Element {
                            tree: data.tree.clone(),
                            node_id,
                        },
                    )
                        .to_lua_multi(lua)?);
                }
                return Ok(Value::Nil.to_lua_multi(lua)?);
            })?;
            Ok((stateless_iter, data.clone(), 0))
        });

        methods.add_method("map", |_, this, (cb,): (mlua::Function,)| {
            for (idx, next) in this.nodes.iter().enumerate() {
                cb.call((
                    Element {
                        tree: this.tree.clone(),
                        node_id: *next,
                    },
                    idx,
                ))?;
            }

            let out = this
                .nodes
                .iter()
                .enumerate()
                .map(|(idx, next)| {
                    let ret: mlua::Value = cb.call((
                        Element {
                            tree: this.tree.clone(),
                            node_id: *next,
                        },
                        idx,
                    ))?;

                    mlua::Result::Ok(ret)
                })
                .collect::<Result<Vec<_>, _>>()?;

            Ok(out)
        });

        methods.add_method("text", |_, this, _: ()| {
            let text = this
                .nodes
                .iter()
                .map(|next| this.tree.get(*next).unwrap())
                .map(|m| ElementRef::new(m).text())
                .flatten()
                .map(|m| StringRef(m.clone()))
                .collect();

            Ok(StringList(text))
        });
    }
}
