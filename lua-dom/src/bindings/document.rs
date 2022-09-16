use mlua::UserData;

use crate::Document;

impl UserData for Document {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(_fields: &mut F) {}

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("select", |_, this, (sel,): (mlua::String,)| {
            let sel = this.select(sel.to_str()?);
            Ok(sel)
        });
    }
}
