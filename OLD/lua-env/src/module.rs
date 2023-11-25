use crate::{env::Environ, settings};

pub struct Module;

pub fn work_dir(vm: &mlua::Lua) -> mlua::Result<String> {
    let settings = settings::get(vm)?;
    Ok(settings.work_dir.to_string())
}

pub fn argv(vm: &mlua::Lua) -> mlua::Result<Vec<String>> {
    let settings = settings::get(vm)?;
    Ok(settings.args.clone())
}

pub fn env(vm: &mlua::Lua) -> mlua::Result<Environ> {
    let settings = settings::get(vm)?;
    Ok(settings.env.clone())
}

impl mlua::UserData for Module {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_function_get("cwd", |vm, _| work_dir(vm));
        fields.add_field_function_get("args", |vm, _| argv(vm));
        fields.add_field_function_get("env", |vm, _| env(vm));
    }
}
