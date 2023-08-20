use std::path::PathBuf;

use lua_fs::module::DirEntry;
use lua_util::stream::DynamicStreamExt;

use crate::exec::Exec;

pub struct ShellSettings {}

#[derive(Debug, Clone)]
pub struct Shell {
    work_dir: PathBuf,
}

impl Shell {
    pub fn new(vm: &mlua::Lua, work_dir: PathBuf) -> Shell {
        vm.set_app_data(ShellSettings {});

        Shell { work_dir }
    }
}

impl mlua::UserData for Shell {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("pwd", |_, this| Ok(this.work_dir.display().to_string()))
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_function("ls", |vm, path: mlua::String| async move {
            let stream = lua_fs::module::read_dir(vm, path).await?;

            let stream = stream
                .try_map(|item: DirEntry| item.path().display().to_string())
                .lua_stream();

            Ok(stream)
        });

        methods.add_async_function("cat", |vm, path: mlua::String| async move {
            lua_fs::module::read_file(vm, path).await
        });

        methods.add_async_function(
            "write",
            |vm, args: (mlua::String, mlua::String)| async move {
                lua_fs::module::write_file(vm, args).await
            },
        );

        methods.add_async_function(
            "test",
            |_vm, (path, ftype): (mlua::String, Option<mlua::String>)| async move {
                let Ok(meta) = tokio::fs::metadata(path.to_str()?).await else {
                return Ok(false)
            };

                let ret = if let Some(filetype) = ftype {
                    match filetype.to_str()? {
                        "file" => meta.is_file(),
                        "dir" | "directory" => meta.is_dir(),
                        ty => {
                            return Err(mlua::Error::external(format!("invalid file type: {ty}")))
                        }
                    }
                } else {
                    true
                };

                Ok(ret)
            },
        );

        methods.add_async_function("mkdir", |vm, path: mlua::String| async move {
            tokio::fs::create_dir_all(path.to_str()?)
                .await
                .map_err(mlua::Error::external)?;

            Ok(())
        });

        methods.add_function("exec", |ctx, args: mlua::String| {
            Ok(Exec::from(args.to_str()?))
        });

        methods.add_function("sh", |ctx, args: mlua::String| {
            Ok(Exec::new(
                "sh".to_string(),
                vec!["-c".to_string(), args.to_str()?.to_string()],
            ))
        });
    }
}
