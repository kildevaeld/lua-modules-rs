use std::path::PathBuf;

use mlua::{AppDataRef, AppDataRefMut, ExternalResult};
use relative_path::RelativePathBuf;

use super::env::Environ;

pub struct EnvSettings {
    pub work_dir: RelativePathBuf,
    pub root_dir: PathBuf,
    pub args: Vec<String>,
    pub env: Environ,
}

impl EnvSettings {
    fn default() -> mlua::Result<EnvSettings> {
        let root = PathBuf::from("/");
        let cwd = std::env::current_dir()?;
        let diff = pathdiff::diff_paths(cwd, &root).ok_or(mlua::Error::external("diff"))?;

        let args = std::env::args().collect();

        Ok(EnvSettings {
            work_dir: RelativePathBuf::from_path(diff).into_lua_err()?,
            root_dir: root,
            args,
            env: Environ::from_env(),
        })
    }
}

pub fn get(vm: &mlua::Lua) -> mlua::Result<AppDataRef<'_, EnvSettings>> {
    if let Some(ret) = vm.app_data_ref::<EnvSettings>() {
        Ok(ret)
    } else {
        vm.set_app_data(EnvSettings::default()?);
        vm.app_data_ref()
            .ok_or_else(|| mlua::Error::external("should not happen"))
    }
}

pub fn get_mut(vm: &mlua::Lua) -> mlua::Result<AppDataRefMut<'_, EnvSettings>> {
    if let Some(ret) = vm.app_data_mut::<EnvSettings>() {
        Ok(ret)
    } else {
        vm.set_app_data(EnvSettings::default()?);
        vm.app_data_mut()
            .ok_or_else(|| mlua::Error::external("should not happen"))
    }
}
