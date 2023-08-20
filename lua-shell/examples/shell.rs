#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lua = mlua::Lua::new();

    lua_util::search_path::append(&lua, "./lua-core/examples/?.lua")?;

    lua_shell::register_module(&lua)?;

    lua.load(include_str!("script.lua")).eval_async().await?;

    Ok(())
}
