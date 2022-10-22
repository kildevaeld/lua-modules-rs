fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lua = mlua::Lua::new();

    let out: mlua::Value = lua
        .load(
            r#"
local test = {
    "Hello, World"
}
return {
    title = test,
    date = "rere"
}
    "#,
        )
        .eval()?;

    println!("out {:?}", lua_value::from_lua(out));

    Ok(())
}
