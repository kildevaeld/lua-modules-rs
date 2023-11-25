use mlua::UserDataRef;
use std::{ops::Deref, process::Stdio};

#[derive(Debug, Clone)]
pub struct Exec {
    cmd: String,
    args: Vec<String>,
}

impl Exec {
    pub fn from(cmd: &str) -> Exec {
        let mut cmds = cmd.split(' ').map(|m| m.to_string());
        let cmd = cmds.next().expect("cmd").to_string();
        let args = cmds.collect();
        Exec { cmd, args }
    }

    pub fn new(cmd: String, args: Vec<String>) -> Exec {
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
        methods.add_async_method("status", |_vm, this, _: ()| async move {
            let status = tokio::process::Command::new(&this.cmd)
                .args(&this.args)
                .status()
                .await
                .map_err(mlua::Error::external)?;

            Ok(status.code().unwrap_or_default())
        });

        methods.add_async_method("output", |_vm, this, _: ()| async move {
            let status = tokio::process::Command::new(&this.cmd)
                .args(&this.args)
                .output()
                .await
                .map_err(mlua::Error::external)?;

            Ok(String::from_utf8(status.stdout).unwrap())
        });

        methods.add_async_method("pipe", |_vm, this, exec: UserDataRef<Exec>| async move {
            Ok(Pipe {
                cmds: vec![this.clone(), exec.deref().clone()],
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

        let _len = rest.len();

        for (_i, next) in rest.iter().enumerate() {
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
        methods.add_async_method("status", |_vm, this, _: ()| async move {
            let mut child = this.run().await.map_err(mlua::Error::external)?;

            let output = child.wait().await.map_err(mlua::Error::external)?;

            Ok(output.code().unwrap_or_default())
        });

        methods.add_async_method("output", |_vm, this, _: ()| async move {
            let child = this.run().await.map_err(mlua::Error::external)?;

            let output = child
                .wait_with_output()
                .await
                .map_err(mlua::Error::external)?;

            Ok(String::from_utf8(output.stdout).unwrap())
        });

        methods.add_async_method_mut("pipe", |_vm, this, exec: UserDataRef<Exec>| async move {
            this.cmds.push(exec.clone());
            Ok(this.clone())
        })
    }
}
