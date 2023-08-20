use lua_util::{iter::LuaIter, types::Lrc};
use mlua::{IntoLua, MetaMethod};
use std::{cell::RefCell, collections::HashMap};

#[derive(Debug, Default, Clone)]
pub struct Environ {
    env: Lrc<RefCell<HashMap<String, String>>>,
}

impl Environ {
    pub fn from_env() -> Environ {
        let env = std::env::vars().collect::<HashMap<_, _>>();
        Environ {
            env: Lrc::new(RefCell::new(env)),
        }
    }
}

impl mlua::UserData for Environ {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {}

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Index, |vm, this, name: mlua::String| {
            //
            let ret = match this.env.borrow().get(name.to_str()?).cloned() {
                Some(ret) => ret.into_lua(vm)?,
                None => mlua::Value::Nil,
            };

            Ok(ret)
        });

        methods.add_meta_method(
            MetaMethod::NewIndex,
            |vm, this, (name, value): (mlua::String, mlua::String)| {
                //
                this.env
                    .borrow_mut()
                    .insert(name.to_str()?.to_string(), value.to_str()?.to_string());

                Ok(())
            },
        );

        methods.add_method("iter", |vm, this, _: ()| {
            let iter = this.env.borrow().clone().into_iter();
            Ok(LuaIter::new(iter))
        });
    }
}
