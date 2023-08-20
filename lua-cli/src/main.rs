use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    path: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let lua = lua_core::create_vm()?;

    lua_core::util::search_path::append(&lua, "./lua-core/examples/?.lua")?;

    lua_core::register_module(&lua)?;
    lua_config::register_module(&lua)?;

    let script = tokio::fs::read(&args.path).await?;

    lua.load(&script).eval_async().await?;

    Ok(())
}
