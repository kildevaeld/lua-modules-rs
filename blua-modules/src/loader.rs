use std::path::{Path, PathBuf};

use relative_path::RelativePath;

use crate::error::LoadError;
#[cfg(feature = "vfs")]
use vfs::{OpenOptions, VFileExt, VPath};
#[cfg(all(feature = "vfs", feature = "async"))]
use vfs::{VAsyncFileExt, VAsyncPath};

pub trait ModuleLoader {
    fn resolve(&self, module: &str, parent: Option<&str>) -> Option<String>;
    fn load<'lua>(
        &self,
        vm: &'lua mlua::Lua,
        env: mlua::Value<'lua>,
        resolved: &str,
    ) -> Result<mlua::Value<'lua>, LoadError>;
}

#[cfg(feature = "async")]
#[async_trait::async_trait(?Send)]
pub trait AsyncModuleLoader {
    async fn resolve(&self, module: &str, parent: Option<&str>) -> Option<String>;
    async fn load<'lua>(
        &self,
        vm: &'lua mlua::Lua,
        env: mlua::Value<'lua>,
        resolved: &str,
    ) -> Result<mlua::Value<'lua>, LoadError>;
}

pub struct DirLoader(PathBuf);

impl DirLoader {
    pub fn new(path: PathBuf) -> DirLoader {
        DirLoader(path)
    }
}

impl ModuleLoader for DirLoader {
    fn resolve(&self, module: &str, parent: Option<&str>) -> Option<String> {
        if let Some(parent) = parent {
            let dir = RelativePath::new(parent)
                .parent()
                .unwrap_or_else(|| RelativePath::new(""));

            let path = dir.join_normalized(module);

            let path = path.to_path(&self.0);

            if !path.exists() {
                None
            } else {
                path.canonicalize().ok().map(|m| m.display().to_string())
            }
        } else {
            let module = Path::new(module);
            if !module.exists() {
                None
            } else {
                module.canonicalize().ok().map(|m| m.display().to_string())
            }
        }
    }

    fn load<'lua>(
        &self,
        vm: &'lua mlua::Lua,
        env: mlua::Value<'lua>,
        resolved: &str,
    ) -> Result<mlua::Value<'lua>, LoadError> {
        let Ok(content) = std::fs::read(resolved) else {
            return Err(LoadError::NotFound);
        };

        println!("LOAD");

        let value = vm
            .load(content)
            .set_name(resolved)
            .set_environment(env)
            .eval()?;

        Ok(value)
    }
}

#[cfg(feature = "vfs")]
pub struct VFSLoader<T> {
    fs: T,
}

#[cfg(feature = "vfs")]
impl<T> VFSLoader<T> {
    pub fn new(fs: T) -> VFSLoader<T> {
        VFSLoader { fs }
    }
}

#[cfg(feature = "vfs")]
impl<T: vfs::VFS> ModuleLoader for VFSLoader<T> {
    fn resolve(&self, module: &str, parent: Option<&str>) -> Option<String> {
        if let Some(parent) = parent {
            let dir = RelativePath::new(parent);

            let path = dir.join_normalized(module);

            let Ok(path) = self.fs.path(path.as_str()) else {
                return None;
            };

            if !path.exists() {
                None
            } else {
                Some(path.to_string())
            }
        } else {
            let Ok(path) = self.fs.path(module) else {
                return None;
            };

            if !path.exists() {
                None
            } else {
                Some(path.to_string())
            }
        }
    }

    fn load<'lua>(
        &self,
        vm: &'lua mlua::Lua,
        env: mlua::Value<'lua>,
        resolved: &str,
    ) -> Result<mlua::Value<'lua>, LoadError> {
        let mut file = self
            .fs
            .path(resolved)
            .map_err(|_| LoadError::NotFound)?
            .open(OpenOptions::default().read(true))
            .map_err(|_| LoadError::NotFound)?;

        let mut buf = Vec::default();

        file.read_to_end(&mut buf).expect("read");

        let value = vm
            .load(buf)
            .set_name(resolved)
            .set_environment(env)
            .eval()?;

        Ok(value)
    }
}

#[cfg(all(feature = "vfs", feature = "async"))]
pub struct VFSAsyncLoader<T> {
    fs: T,
}

#[cfg(all(feature = "vfs", feature = "async"))]
impl<T> VFSAsyncLoader<T> {
    pub fn new(fs: T) -> VFSAsyncLoader<T> {
        VFSAsyncLoader { fs }
    }
}

#[cfg(all(feature = "vfs", feature = "async"))]
#[async_trait::async_trait(?Send)]
impl<T: vfs::VAsyncFS> AsyncModuleLoader for VFSAsyncLoader<T>
where
    <T::Path as VAsyncPath>::File: Unpin,
{
    async fn resolve(&self, module: &str, parent: Option<&str>) -> Option<String> {
        if let Some(parent) = parent {
            let dir = RelativePath::new(parent);

            let path = dir.join_normalized(module);

            let Ok(path) = self.fs.path(path.as_str()) else {
                return None;
            };

            if !path.exists().await {
                None
            } else {
                Some(path.to_string())
            }
        } else {
            let Ok(path) = self.fs.path(module) else {
                return None;
            };

            if !path.exists().await {
                None
            } else {
                Some(path.to_string())
            }
        }
    }

    async fn load<'lua>(
        &self,
        vm: &'lua mlua::Lua,
        env: mlua::Value<'lua>,
        resolved: &str,
    ) -> Result<mlua::Value<'lua>, LoadError> {
        let mut file = self
            .fs
            .path(resolved)
            .map_err(|_| LoadError::NotFound)?
            .open(OpenOptions::default().read(true))
            .await
            .map_err(|_| LoadError::NotFound)?;

        let mut buf = Vec::default();

        file.read_to_end(&mut buf).await.expect("read");

        let value = vm
            .load(buf)
            .set_name(resolved)
            .set_environment(env)
            .eval()?;

        Ok(value)
    }
}

pub struct BuiltIn;

impl ModuleLoader for BuiltIn {
    fn resolve(&self, module: &str, _parent: Option<&str>) -> Option<String> {
        match module {
            "table" | "coroutine" | "string" => Some(module.to_string()),
            _ => None,
        }
    }

    fn load<'lua>(
        &self,
        vm: &'lua mlua::Lua,
        _env: mlua::Value<'lua>,
        resolved: &str,
    ) -> Result<mlua::Value<'lua>, LoadError> {
        let pkg = vm
            .globals()
            .get::<_, mlua::Table>("package")?
            .get::<_, mlua::Table>("preload")?;

        let resolved: mlua::Function = pkg.get(resolved).map_err(|_| LoadError::NotFound)?;

        Ok(resolved.call(())?)
    }
}
