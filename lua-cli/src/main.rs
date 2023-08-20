use std::ffi::OsString;

use clap::{Command, Parser, Subcommand};

// #[derive(Parser)]
// #[command(author, version, about, long_about = None)]
// struct Cli {
//     #[command(subcommand)]
//     command: Commands,
// }

// #[derive(Debug, Subcommand)]
// enum Commands {
//     Run {
//         #[command(external_subcommand)]
//         args: Vec<String>,
//     },
// }

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let args = Cli::parse();

    let args = clap::Command::new("blur")
        .subcommand(Command::new("run").allow_external_subcommands(true))
        .get_matches();

    // let lua = lua_core::create_vm()?;

    // lua_core::util::search_path::append(&lua, "./lua-core/examples/?.lua")?;

    // lua_core::register_module(&lua)?;
    // lua_config::register_module(&lua)?;
    // lua_env::register_module(&lua)?;

    // let script = tokio::fs::read(&args.path).await?;

    // lua.load(&script).eval_async().await?;

    match args.subcommand() {
        Some(("run", args)) => {
            let (cmd, args) = args.subcommand().expect("subcommand");
            let args = args
                .get_many::<OsString>("")
                .unwrap()
                .map(|m| m.to_string_lossy().to_string())
                .collect::<Vec<_>>();

            run_command(cmd, args).await?;
        }
        _ => {
            panic!("")
        }
    };

    Ok(())
}

pub async fn run_command(path: &str, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    //println!("ARGS: {:?}", args);

    let lua = lua_core::create_vm()?;

    lua_env::settings::get_mut(&lua)?.args = args;

    lua_core::util::search_path::append(&lua, "./lua-core/examples/?.lua")?;

    lua_core::register_module(&lua)?;
    lua_config::register_module(&lua)?;
    lua_env::register_module(&lua)?;

    let script = tokio::fs::read(&path).await?;

    lua.load(&script).set_name(path).eval_async().await?;

    Ok(())
}
