use std::process::Stdio;

use lua_fs::module::DirEntry;
use lua_util::stream::DynamicStreamExt;

use crate::shell::Shell;

pub fn init<'a>(vm: &'a mlua::Lua, module: &mlua::Table<'a>) -> mlua::Result<Shell> {
    /*
    let ls = vm.create_async_function(|vm, path: mlua::String| async move {
        let stream = lua_fs::module::read_dir(vm, path).await?;

        let stream = stream
            .try_map(|item: DirEntry| item.path().display().to_string())
            .lua_stream();

        Ok(stream)
    })?;
    module.set("ls", ls)?;

    let cat = vm.create_async_function(|vm, path: mlua::String| async move {
        let output = tokio::fs::read_to_string(path.to_str()?)
            .await
            .map_err(mlua::Error::external)?;

        Ok(output)
    })?;
    module.set("cat", cat)?;

    let cd = vm.create_async_function(|vm, path: mlua::String| async move {
        let output = tokio::fs::read_to_string(path.to_str()?)
            .await
            .map_err(mlua::Error::external)?;

        Ok(output)
    })?;
    module.set("cd", cd)?;

    let exec = vm.create_function(|ctx, args: mlua::String| {
        //
        Ok(Exec::new(args.to_str()?))
    })?;

    module.set("exec", exec)?;
    */

    let shell = Shell::new(vm, std::env::current_dir().expect("pwd"));

    Ok(shell)
}

#[derive(Debug, Clone)]
pub struct Exec {
    cmd: String,
    args: Vec<String>,
}

impl Exec {
    pub fn new(cmd: &str) -> Exec {
        let mut cmds = cmd.split(' ').map(|m| m.to_string());
        let cmd = cmds.next().expect("cmd").to_string();
        let args = cmds.collect();
        Exec { cmd, args }
    }

    fn build_cmd(&self) -> tokio::process::Command {
        let mut cmd = tokio::process::Command::new(&self.cmd);
        cmd.args(&self.args);
        cmd
    }
}

impl mlua::UserData for Exec {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method("run", |vm, this, _: ()| async move {
            let status = tokio::process::Command::new(this.cmd)
                .args(this.args)
                .status()
                .await
                .map_err(mlua::Error::external)?;

            Ok(status.code().unwrap_or_default())
        });

        methods.add_async_method("status", |vm, this, _: ()| async move {
            let status = tokio::process::Command::new(this.cmd)
                .args(this.args)
                .output()
                .await
                .map_err(mlua::Error::external)?;

            Ok(String::from_utf8(status.stdout).unwrap())
        });

        methods.add_async_method("pipe", |vm, this, exec: Exec| async move {
            Ok(Pipe {
                cmds: vec![this, exec],
            })
        })
    }
}

#[derive(Debug, Clone)]
pub struct Pipe {
    cmds: Vec<Exec>,
}

impl Pipe {
    async fn run(&self) -> std::io::Result<tokio::process::Child> {
        let Some((first, rest)) = self.cmds.split_first() else {
            panic!("no exec")
        };

        let first = first.build_cmd().stdout(Stdio::piped()).spawn()?;

        let mut children = vec![first];

        let len = rest.len();

        for (i, next) in rest.iter().enumerate() {
            let prev: Stdio = children
                .last_mut()
                .unwrap()
                .stdout
                .take()
                .expect("")
                .try_into()?;

            // let child = if i == (len - 1) {
            //     next.build_cmd().stdin(prev).spawn()?
            // } else {
            //     next.build_cmd()
            //         .stdin(prev)
            //         .stdout(Stdio::piped())
            //         .spawn()?
            // };

            let child = next
                .build_cmd()
                .stdin(prev)
                .stdout(Stdio::piped())
                .spawn()?;

            children.push(child);
        }

        let last = children.pop().expect("last");

        for mut child in children {
            child.wait().await?;
        }

        Ok(last)
    }
}

impl mlua::UserData for Pipe {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method("run", |vm, this, _: ()| async move {
            let mut child = this.run().await.map_err(mlua::Error::external)?;

            let output = child.wait().await.map_err(mlua::Error::external)?;

            Ok(output.code().unwrap_or_default())
        });

        methods.add_async_method("status", |vm, this, _: ()| async move {
            let child = this.run().await.map_err(mlua::Error::external)?;

            let output = child
                .wait_with_output()
                .await
                .map_err(mlua::Error::external)?;

            Ok(String::from_utf8(output.stdout).unwrap())
        });

        methods.add_async_method("pipe", |vm, mut this, exec: Exec| async move {
            this.cmds.push(exec);
            Ok(this)
        })
    }
}
