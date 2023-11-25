use mlua::{ExternalResult, LuaSerdeExt};
use serde_json::Value;

pub struct Module;

impl mlua::UserData for Module {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("create", |_vm, _: ()| {
            Ok(LuaHandlebars(handlebars::Handlebars::new()))
        })
    }
}

pub struct LuaHandlebars(handlebars::Handlebars<'static>);

impl mlua::UserData for LuaHandlebars {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method(
            "render",
            |vm, this, (template, ctx): (mlua::String, mlua::Value)| {
                let ctx: Value = vm.from_value(ctx)?;

                let out = this
                    .0
                    .render_template(template.to_str()?, &ctx)
                    .into_lua_err()?;

                Ok(out)
            },
        );

        methods.add_method_mut(
            "registerTemplate",
            |_vm, this, (name, template): (mlua::String, mlua::String)| {
                this.0
                    .register_template_string(name.to_str()?, template.to_str()?)
                    .into_lua_err()?;
                Ok(())
            },
        );
    }
}
