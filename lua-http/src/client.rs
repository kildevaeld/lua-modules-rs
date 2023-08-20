
use mlua::{MetaMethod};
use reqwest::{Method};

pub fn init(vm: &mlua::Lua, table: &mlua::Table) -> mlua::Result<()> {
    let client_new = vm.create_function(|_vm, _: ()| {
        let client = reqwest::Client::new();
        Ok(Client { client })
    })?;

    table.set("client", client_new)?;

    Ok(())
}

#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
}

impl mlua::UserData for Client {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get", |_vm, this, url: String| {
            Ok(Request::new(this.client.clone(), Method::GET, url))
        })
    }
}

struct ClientResponse {
    resp: reqwest::Response,
}

#[derive(Clone)]
struct Request {
    client: reqwest::Client,
    url: String,
    method: reqwest::Method,
}

impl Request {
    pub fn new(client: reqwest::Client, method: Method, url: impl ToString) -> Request {
        Request {
            client,
            url: url.to_string(),
            method,
        }
    }

    async fn send(&self) -> mlua::Result<ClientResponse> {
        let resp = self
            .client
            .request(self.method.clone(), &self.url)
            .send()
            .await
            .map_err(mlua::Error::external)?;

        Ok(ClientResponse { resp })
    }
}

impl mlua::UserData for Request {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_meta_method(MetaMethod::Call, |_vm, _this, _: ()| async move {
            //

            Ok(())
        })
    }
}
