use value::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lua = mlua::Lua::new();

    let value = lua_value::to_lua(
        &lua,
        value::Value::List(vec![Value::Bool(true), Value::Bool(false)]),
    )?;

    println!("VAL {:?}", lua_value::from_lua(value));

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
