use mlua::LuaSerdeExt;
use serde_json::Value;
use toback::Toback;

pub fn init(vm: &mlua::Lua) -> Result<ConfigModule, mlua::Error> {
    Ok(ConfigModule::new(vm))
}

#[non_exhaustive]
pub struct ConfigModule {}

impl ConfigModule {
    pub fn new(vm: &mlua::Lua) -> ConfigModule {
        let toback: toback::Toback<serde_json::Value> = toback::TobackBuilder::default().build();
        vm.set_app_data(toback);
        ConfigModule {}
    }
}

impl mlua::UserData for ConfigModule {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_function("read", |vm, path: mlua::String| async move {
            let path = path.to_str()?;

            let content = tokio::fs::read(path).await.map_err(mlua::Error::external)?;

            let toback = vm
                .app_data_ref::<Toback<Value>>()
                .ok_or_else(|| mlua::Error::external("toback"))?;

            let encoder = toback
                .encoder_from_path(path)
                .ok_or_else(|| mlua::Error::external("invalid ext"))?;

            let output = encoder.load(&content).map_err(mlua::Error::external)?;

            vm.to_value(&output)
        });

        methods.add_async_function(
            "write",
            |vm, (path, content): (mlua::String, mlua::Value)| async move {
                let path = path.to_str()?;

                let toback = vm
                    .app_data_ref::<Toback<Value>>()
                    .ok_or_else(|| mlua::Error::external("toback"))?;

                let encoder = toback
                    .encoder_from_path(path)
                    .ok_or_else(|| mlua::Error::external("invalid ext"))?;

                let content: Value = vm.from_value(content)?;

                let output = encoder
                    .save_pretty(&content)
                    .map_err(mlua::Error::external)?;

                drop(toback);

                tokio::fs::write(path, output).await?;

                Ok(())
            },
        );
    }
}
