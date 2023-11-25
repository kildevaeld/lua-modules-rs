#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lua = mlua::Lua::new();

    lua_util::search_path::append(&lua, "./lua-fs/examples/?.lua")?;

    lua_fs::register_module(&lua)?;
    lua_util::register_modules(&lua)?;

    lua.load(include_str!("script.lua")).eval_async().await?;

    Ok(())
}
