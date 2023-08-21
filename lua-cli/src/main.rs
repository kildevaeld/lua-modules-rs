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

    match args.subcommand() {
        Some(("run", args)) => {
            let (cmd, args) = match args.subcommand() {
                Some(ret) => ret,
                None => {
                    eprintln!("usage: {} run <path> [args]", "blur");
                    return Ok(());
                }
            };
            let args = args
                .get_many::<OsString>("")
                .unwrap()
                .map(|m| m.to_string_lossy().to_string())
                .collect::<Vec<_>>();

            run_command(cmd, args).await?;
        }
        _ => {}
    };

    Ok(())
}

pub async fn run_command(path: &str, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let lua = lua_core::create_vm()?;

    let mut lua_args = vec![path.to_string()];

    lua_args.extend(args);

    lua_env::settings::get_mut(&lua)?.args = lua_args;

    lua_core::util::search_path::append(&lua, "./?.lua")?;

    lua_core::register_module(&lua)?;
    lua_config::register_module(&lua)?;
    lua_env::register_module(&lua)?;

    let mut script = &*tokio::fs::read(&path).await?;

    if script.len() >= 2 && script[0] as char == '#' && script[1] as char == '!' {
        if let Some(pos) = script.iter().position(|&x| x as char == '\n') {
            script = &script[pos..];
        }
    }

    lua.load(script).set_name(path).eval_async().await?;

    Ok(())
}
