use std::{marker::PhantomPinned, pin::Pin};

use mlua::MetaMethod;
use regex::Regex;

struct LuaRegex(Regex);

impl mlua::UserData for LuaRegex {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("is_match", |_vm, this, haystack: String| {
            Ok(this.0.is_match(&haystack))
        });

        methods.add_method("captures", |_vm, this, haystack: String| {
            Ok(Captures::new(&this.0, haystack))
        })
    }
}

pub struct CapturesInner {
    haystack: String,
    captures: Option<regex::Captures<'static>>,
    _pin: PhantomPinned,
}

impl Drop for CapturesInner {
    fn drop(&mut self) {
        inner_drop(unsafe { Pin::new_unchecked(self) });
        fn inner_drop(this: Pin<&mut CapturesInner>) {
            let this = unsafe { Pin::get_unchecked_mut(this) };
            let _ = this.captures.take();
        }
    }
}

pub struct Captures(Pin<Box<CapturesInner>>);

impl Captures {
    pub fn new(reg: &Regex, haystack: String) -> Captures {
        let res = CapturesInner {
            haystack,
            // we only create the pointer once the data is in place
            // otherwise it will have already moved before we even started
            captures: None,
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res);

        unsafe {
            let captures = std::mem::transmute(reg.captures(&boxed.haystack));
            let mut_ref: Pin<&mut CapturesInner> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).captures = captures;
        };

        Captures(boxed)
    }
}

impl mlua::UserData for Captures {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {}

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get", |vm, this, item: u32| {
            let idx = (item.min(1) - 1) as usize;

            let capture = this.0.captures.and_then(|m| {
                m.get(idx).and_then(|mat| {
                    Some(Match {
                        string: mat.as_str().to_string(),
                        start: mat.start(),
                        end: mat.end(),
                    })
                })
            });

            Ok(capture)
        });

        methods.add_meta_method(MetaMethod::Len, |vm, this, args: ()| {
            Ok(this.0.captures.map(|m| m.len()).unwrap_or_default())
        })
    }
}

pub struct Match {
    string: String,
    start: usize,
    end: usize,
}

impl mlua::UserData for Match {}

pub struct LuaIter<T>(T);

impl<T> mlua::UserData for LuaIter<T>
where
    T: Iterator,
    for<'lua> T::Item: mlua::IntoLua<'lua>,
{
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method_mut(MetaMethod::Call, |_vm, this, _args: ()| {
            let next = this.0.next();
            Ok(next)
        })
    }
}
