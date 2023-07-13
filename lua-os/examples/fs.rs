#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lua = mlua::Lua::new();

    lua_os::register_module(&lua)?;

    lua.load(include_str!("script.lua")).eval_async().await?;

    Ok(())
}
