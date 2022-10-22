#[cfg(feature = "dom")]
use lua_dom::{
    bindings::{StringList, StringRef},
    StrTendril,
};

use value::{Map, Value};

pub fn to_lua<'js>(vm: &'js mlua::Lua, value: Value) -> mlua::Value<'js> {
    match value {
        Value::Bool(b) => mlua::Value::Boolean(b),
        Value::Number(n) => {
            if n.is_float() {
                mlua::Value::Number(n.as_f64())
            } else {
                mlua::Value::Integer(n.as_i64())
            }
        }
        Value::String(s) => mlua::Value::String(vm.create_string(&s).unwrap()),
        Value::Map(m) => {
            //
            let iter = m.into_iter().map(|(k, v)| (k, to_lua(vm, v)));
            mlua::Value::Table(vm.create_table_from(iter).expect("map"))
        }
        Value::List(list) => {
            let iter = list.into_iter().map(|v: Value| to_lua(vm, v)).enumerate();
            mlua::Value::Table(vm.create_table_from(iter).expect("list"))
        }
        _ => {
            panic!("unimplemented")
        }
    }
}

fn is_array<'lua>(table: mlua::Table<'lua>) -> Result<bool, mlua::Error> {
    if table.raw_len() > 0 {
        return Ok(true);
    }

    for _ in table.pairs::<mlua::Value, mlua::Value>() {
        return Ok(false);
    }

    Ok(true)
}

fn from_lua_table<'lua>(table: mlua::Table<'lua>) -> Result<Value, mlua::Error> {
    if is_array(table.clone())? {
        let v = table
            .sequence_values::<mlua::Value>()
            .map(|ret| match ret {
                Ok(ret) => from_lua(ret),
                Err(err) => Err(err),
            })
            .collect::<Result<_, _>>()?;
        Ok(Value::List(v))
    } else {
        let mut map = Map::default();
        for pair in table.pairs::<mlua::String, mlua::Value>() {
            let (k, v) = pair?;
            map.insert(k.to_str()?, from_lua(v)?);
        }
        Ok(Value::Map(map))
    }
}

pub fn from_lua<'lua>(value: mlua::Value<'lua>) -> Result<Value, mlua::Error> {
    let ret = match value {
        mlua::Value::Boolean(b) => Value::Bool(b),
        mlua::Value::Error(err) => return Err(err),
        mlua::Value::Integer(i) => Value::Number(i.into()),
        mlua::Value::Number(n) => Value::Number(n.into()),
        mlua::Value::String(s) => Value::String(s.to_string_lossy().to_string()),
        mlua::Value::Nil => Value::None,
        mlua::Value::Table(m) => from_lua_table(m)?,
        #[allow(unused)]
        mlua::Value::UserData(user) => {
            #[cfg(feature = "dom")]
            if user.is::<StringRef<String>>() {
                let string_ref = user.borrow::<StringRef<String>>()?;
                return Ok(Value::String(string_ref.to_string()));
            } else if user.is::<lua_dom::bindings::StringList<String>>() {
                let string_list = user.borrow::<StringList<String>>()?;
                return Ok(Value::List(
                    string_list.iter().map(|m| m.to_string().into()).collect(),
                ));
            } else if user.is::<StringRef<StrTendril>>() {
                let string_ref = user.borrow::<StringRef<StrTendril>>()?;
                return Ok(Value::String(string_ref.to_string()));
            } else if user.is::<lua_dom::bindings::StringList<StrTendril>>() {
                let string_list = user.borrow::<StringList<StrTendril>>()?;
                return Ok(Value::List(
                    string_list.iter().map(|m| m.to_string().into()).collect(),
                ));
            }

            return Err(mlua::Error::external("could not serialize userdata"));
        }
        _ => {
            //
            return Err(mlua::Error::external("could not serialize"));
        }
    };

    Ok(ret)
}
