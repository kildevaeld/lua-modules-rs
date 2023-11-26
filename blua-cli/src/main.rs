use clap::{Arg, Command};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = clap::Command::new("blur")
        .subcommand(
            Command::new("run")
                .arg(
                    Arg::new("search-paths")
                        .short('s')
                        .action(clap::ArgAction::Append)
                        .value_parser(clap::value_parser!(PathBuf))
                        .help("where to look for modules"),
                )
                .allow_external_subcommands(true),
        )
        .subcommand(Command::new("types").arg(Arg::new("path").required(true)))
        .get_matches();

    match args.subcommand() {
        Some(("run", args)) => {
            let search_paths = args
                .get_many::<PathBuf>("search-paths")
                .into_iter()
                .flat_map(|m| m.map(|m| &**m));

            let (cmd, args) = match args.subcommand() {
                Some(ret) => ret,
                None => {
                    eprintln!("usage: {} run <path> [args]", "blur");
                    return Ok(());
                }
            };

            let lua_args = args
                .get_many::<OsString>("")
                .unwrap()
                .map(|m| m.to_string_lossy().to_string())
                .collect::<Vec<_>>();

            run_command(cmd, lua_args, search_paths).await?;
        }
        Some(("types", args)) => {
            let path = args.get_one::<String>("path").expect("should not be empty");
            types_command(path).await?;
        }
        _ => {}
    };

    Ok(())
}

async fn run_command(
    path: &str,
    args: Vec<String>,
    search_paths: impl Iterator<Item = &Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let lua = blua::create_vm(mlua::LuaOptions::default())?;

    let mut lua_args = vec![path.to_string()];

    lua_args.extend(args);

    blua::os::env::settings::get_mut(&lua)?.args = lua_args;
    blua::util::search_path::append(&lua, "./?.lua")?;

    for sp in search_paths {
        let sp = sp.join("?.lua");
        let string = sp.display();
        blua::util::search_path::append(&lua, &string.to_string())?;
    }

    let mut script = &*tokio::fs::read(&path).await?;

    if script.len() >= 2 && script[0] as char == '#' && script[1] as char == '!' {
        if let Some(pos) = script.iter().position(|&x| x as char == '\n') {
            script = &script[pos..];
        }
    }

    if let Err(err) = lua.load(script).set_name(path).eval_async::<()>().await {
        eprintln!("{err}");
    }

    Ok(())
}

async fn types_command<P: Into<PathBuf>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.into();

    if !path.is_dir() {
        tokio::fs::create_dir_all(&path).await?;
    }

    tokio::task::spawn_blocking(move || blua::write_definitions(&path)).await??;

    Ok(())
}
