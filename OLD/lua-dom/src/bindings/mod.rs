mod document;
mod element;
mod selection;
mod shared;

pub use self::{element::Element, shared::*};

pub fn register_module(vm: &mlua::Lua) -> mlua::Result<()> {
    let package = vm
        .globals()
        .get::<_, mlua::Table>("package")?
        .get::<_, mlua::Table>("preload")?;

    let preload = vm.create_function(|vm, ()| {
        let table = vm.create_table()?;

        let parse = vm.create_function(|_, (html,): (mlua::String,)| {
            let dom = crate::Document::parse(html.to_str()?);
            Ok(dom)
        })?;

        table.set("parse", parse)?;

        Ok(table)
    })?;

    package.set("dom", preload)?;

    Ok(())
}
