fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lua = mlua::Lua::new();

    lua_dom::register_module(&lua)?;

    lua.load(include_str!("script.lua")).eval()?;

    Ok(())
}
