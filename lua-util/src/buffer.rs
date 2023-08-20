use bytes::{Buf, Bytes};

pub struct LuaBuffer(pub Bytes);

impl<'a> From<&'a [u8]> for LuaBuffer {
    fn from(value: &'a [u8]) -> Self {
        LuaBuffer(Bytes::copy_from_slice(value))
    }
}

impl From<Vec<u8>> for LuaBuffer {
    fn from(value: Vec<u8>) -> Self {
        LuaBuffer(Bytes::from(value))
    }
}

impl mlua::UserData for LuaBuffer {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("length", |vm, this| Ok(this.0.len()))
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("toString", |vm, this, encoding: mlua::String| {
            let bytes = this.0.chunk();

            let out = match encoding.to_str()? {
                "hex" => data_encoding::HEXLOWER.encode(bytes),
                "base64" => data_encoding::BASE64.encode(bytes),
                "utf8" | "utf-8" => {
                    String::from_utf8(bytes.to_vec()).map_err(mlua::Error::external)?
                }
                encoding => {
                    return Err(mlua::Error::external(format!(
                        "invalid encoding: {encoding}"
                    )));
                }
            };

            Ok(out)
        });
    }
}
