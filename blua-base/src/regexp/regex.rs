
use regex::Regex;

use super::{captures::LuaCaptures, r#match::LuaMatch};

pub struct LuaRegex(pub Regex);

impl mlua::UserData for LuaRegex {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("is_match", |_vm, this, haystack: String| {
            Ok(this.0.is_match(&haystack))
        });

        methods.add_method("captures", |_vm, this, haystack: String| {
            Ok(LuaCaptures::new(&this.0, haystack))
        });

        methods.add_method("find_first", |_vm, this, haystack: mlua::String| {
            let found = this.0.find(haystack.to_str()?).map(|m| LuaMatch {
                start: m.start(),
                end: m.end(),
                string: m.as_str().to_string(),
            });

            Ok(found)
        });

        methods.add_method("find", |_vm, this, haystack: mlua::String| {
            let found = this
                .0
                .find_iter(haystack.to_str()?)
                .map(|m| LuaMatch {
                    start: m.start(),
                    end: m.end(),
                    string: m.as_str().to_string(),
                })
                .collect::<Vec<_>>();

            if found.is_empty() {
                Ok(None)
            } else {
                Ok(Some(found))
            }
        })
    }
}
