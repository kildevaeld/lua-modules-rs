use std::collections::HashMap;

use blua_shared::bytes::LuaBuffer;
use bytes::Bytes;
use encoding_rs::{Encoding, UTF_8};
use hyper::header::CONTENT_TYPE;
use mime::Mime;
use mlua::{LuaSerdeExt, UserData};

use crate::util::to_bytes;

pub struct LuaResponse(pub reqwest::Response);

impl LuaResponse {
    pub async fn to_bytes(&mut self) -> mlua::Result<Bytes> {
        to_bytes(&mut self.0).await.map_err(mlua::Error::external)
    }

    pub async fn to_string(&mut self, default_encoding: &str) -> mlua::Result<String> {
        let content_type = self
            .0
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<Mime>().ok());
        let encoding_name = content_type
            .as_ref()
            .and_then(|mime| mime.get_param("charset").map(|charset| charset.as_str()))
            .unwrap_or(default_encoding);
        let encoding = Encoding::for_label(encoding_name.as_bytes()).unwrap_or(UTF_8);

        let full = self.to_bytes().await?;

        let (text, _, _) = encoding.decode(&full);
        Ok(text.into_owned())
    }
}

impl UserData for LuaResponse {
    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("status", |_, this| Ok(this.0.status().as_u16()));
        fields.add_field_method_get("headers", |_, this| {
            let headers = this
                .0
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.as_bytes().to_vec()))
                .collect::<HashMap<_, _>>();

            Ok(headers)
        });
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method_mut("text", |_, this, _: ()| async move {
            this.to_string("utf-8").await
        });

        methods.add_async_method_mut("bytes", |_, this, _: ()| async move {
            let bytes = this.to_bytes().await?;
            Ok(LuaBuffer(bytes))
        });

        methods.add_async_method_mut("json", |vm, this, _: ()| async move {
            let bytes = this.to_bytes().await?;

            let value: serde_json::Value =
                serde_json::from_slice(bytes.as_ref()).map_err(mlua::Error::external)?;

            vm.to_value(&value)
        });
    }
}
