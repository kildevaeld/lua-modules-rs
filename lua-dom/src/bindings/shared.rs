use mlua::{MetaMethod, ToLuaMulti, UserData, Value};

#[derive(Clone, Debug)]
pub struct StringRef<S>(pub S);

impl<S: std::fmt::Display + AsRef<str>> UserData for StringRef<S> {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(_fields: &mut F) {}

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(this.0.to_string()));

        methods.add_meta_method(MetaMethod::Eq, |_, this, args: mlua::String| {
            Ok(this.0.as_ref() == args.to_str()?)
        });

        methods.add_method("trim", |_, this, ()| {
            Ok(StringRef(this.0.as_ref().trim().to_string()))
        });

        methods.add_method("str", |_, this, ()| Ok(StringRef(this.0.to_string())));
    }
}

impl<S: std::fmt::Display> std::fmt::Display for StringRef<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct StringList<S>(pub Vec<StringRef<S>>);

impl<S> StringList<S> {
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a StringRef<S>> {
        self.0.iter()
    }
}

impl<S: std::fmt::Display + AsRef<str> + Clone + 'static> UserData for StringList<S> {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(_fields: &mut F) {}

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("join", |_, this, (joiner,): (Option<mlua::String>,)| {
            let joiner = if let Some(joiner) = &joiner {
                joiner.to_str()?
            } else {
                ""
            };

            Ok(itertools::join(this.0.iter(), joiner))
        });

        #[cfg(any(
            feature = "lua54",
            feature = "lua53",
            feature = "lua52",
            feature = "luajit52"
        ))]
        methods.add_meta_method(MetaMethod::Pairs, |lua, data, ()| {
            let stateless_iter = lua.create_function(|lua, (data, i): (StringList<S>, i64)| {
                let i = i + 1;
                if (i as usize) <= data.0.len() {
                    let node_id = &data.0[(i - 1) as usize];

                    return Ok((i, node_id.clone()).to_lua_multi(lua)?);
                }
                return Ok(Value::Nil.to_lua_multi(lua)?);
            })?;
            Ok((stateless_iter, data.clone(), 0))
        });

        methods.add_method("trim", |_, this, ()| {
            let out = this
                .0
                .iter()
                .map(|m| StringRef(m.0.as_ref().trim().to_string()))
                .filter(|m| !m.0.is_empty())
                .collect();

            Ok(StringList(out))
        });
    }
}
