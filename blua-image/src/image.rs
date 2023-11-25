use std::{io::Cursor, path::Path};

use image::{imageops::FilterType, DynamicImage};
use mlua::{FromLua, IntoLua, UserData};

pub struct LuaImageFormat(image::ImageOutputFormat);

impl<'lua> FromLua<'lua> for LuaImageFormat {
    fn from_lua(
        value: mlua::prelude::LuaValue<'lua>,
        lua: &'lua mlua::prelude::Lua,
    ) -> mlua::prelude::LuaResult<Self> {
        let Some(string) = value.as_str() else {
            return Err(mlua::Error::external("expected string"));
        };

        let fmt = match string {
            "jpeg" | "jpg" => image::ImageOutputFormat::Jpeg(60),
            "png" => image::ImageOutputFormat::Png,
            #[cfg(feature = "webp")]
            "webp" => image::ImageOutputFormat::WebP,
            _ => return Err(mlua::Error::external(format!("expected: png, jpeg, jpg"))),
        };

        Ok(LuaImageFormat(fmt))
    }
}

impl<'lua> IntoLua<'lua> for LuaImageFormat {
    fn into_lua(
        self,
        lua: &'lua mlua::prelude::Lua,
    ) -> mlua::prelude::LuaResult<mlua::prelude::LuaValue<'lua>> {
        use image::ImageOutputFormat::*;
        let fmt = match self.0 {
            Png => "png",
            Jpeg(_) => "jpg",
            #[cfg(feature = "webp")]
            Webp => "webp",
            _ => return Err(mlua::Error::external("unsupported debug")),
        };

        fmt.into_lua(lua)
    }
}

pub struct LuaFilterType(image::imageops::FilterType);

impl<'lua> FromLua<'lua> for LuaFilterType {
    fn from_lua(
        value: mlua::prelude::LuaValue<'lua>,
        _lua: &'lua mlua::prelude::Lua,
    ) -> mlua::prelude::LuaResult<Self> {
        let Some(m) = value.as_str() else {
            return Err(mlua::Error::external("expected string"));
        };

        let ty = match m {
            "nearest" => FilterType::Nearest,
            "triangle" => FilterType::Triangle,
            "catmullrom" => FilterType::CatmullRom,
            "gaussian" => FilterType::Gaussian,
            "lanczos3" => FilterType::Lanczos3,
            _ => return Err(mlua::Error::external(format!("invalid filter type: {m}"))),
        };

        Ok(LuaFilterType(ty))
    }
}

impl<'lua> IntoLua<'lua> for LuaFilterType {
    fn into_lua(
        self,
        lua: &'lua mlua::prelude::Lua,
    ) -> mlua::prelude::LuaResult<mlua::prelude::LuaValue<'lua>> {
        use image::imageops::FilterType::*;
        let ty = match self.0 {
            Nearest => "nearest",
            Triangle => "triangle",
            CatmullRom => "callmullrom",
            Lanczos3 => "lanczos3",
            Gaussian => "gaussian",
        };

        ty.into_lua(lua)
    }
}

pub struct LuaImage(pub DynamicImage);

impl UserData for LuaImage {
    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("size", |_, this| Ok(this.0.as_bytes().len()));

        fields.add_field_method_get("width", |_, this| Ok(this.0.width()));

        fields.add_field_method_get("height", |_, this| Ok(this.0.height()));
    }

    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method(
            "write",
            |_, this, (path, format): (mlua::String, Option<LuaImageFormat>)| async move {
                let path = path.to_str()?;

                let path = Path::new(path);

                let format = if let Some(fmt) = format {
                    fmt.0
                } else {
                    let Some(ext) = path.extension() else {
                        return Err(mlua::Error::external("could not infer image type"));
                    };

                    let Some(ext) = ext.to_str() else {
                        return Err(mlua::Error::external("invalid format"));
                    };

                    let format = match ext {
                        "png" => image::ImageOutputFormat::Png,
                        "jpg" | "jpeg" => image::ImageOutputFormat::Jpeg(60),
                        #[cfg(feature = "webp")]
                        "webp" => image::ImageOutputFormat::WebP,
                        ext => return Err(mlua::Error::external(format!("invalid format: {ext}"))),
                    };

                    format
                };

                let mut bytes: Vec<u8> = Vec::new();
                this.0
                    .write_to(&mut Cursor::new(&mut bytes), format)
                    .map_err(mlua::Error::external)?;

                Ok(())
            },
        );

        methods.add_method(
            "thumbnail",
            |_, this, (w, h, exact): (u32, u32, Option<bool>)| {
                let exact = exact.unwrap_or_default();

                let image = if exact {
                    this.0.thumbnail_exact(w, h)
                } else {
                    this.0.thumbnail(w, h)
                };

                Ok(LuaImage(image))
            },
        );

        methods.add_method(
            "thumbnail",
            |_, this, (w, h, ty, exact): (u32, u32, LuaFilterType, Option<bool>)| {
                let exact = exact.unwrap_or_default();

                let image = if exact {
                    this.0.resize_exact(w, h, ty.0)
                } else {
                    this.0.resize(w, h, ty.0)
                };

                Ok(LuaImage(image))
            },
        );

        methods.add_method("blur", |_, this, sigma: f32| {
            let image = this.0.blur(sigma);
            Ok(LuaImage(image))
        });
    }
}
