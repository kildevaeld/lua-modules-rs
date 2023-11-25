use std::{marker::PhantomPinned, pin::Pin};

use mlua::MetaMethod;
use regex::Regex;

use super::r#match::LuaMatch;

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

pub struct LuaCaptures(Pin<Box<CapturesInner>>);

impl LuaCaptures {
    pub fn new(reg: &Regex, haystack: String) -> LuaCaptures {
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

        LuaCaptures(boxed)
    }
}

impl mlua::UserData for LuaCaptures {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get", |_vm, this, item: u32| {
            let idx = (item.max(1) - 1) as usize;
            let capture = this.0.captures.as_ref().and_then(|m| {
                m.get(idx).and_then(|mat| {
                    Some(LuaMatch {
                        string: mat.as_str().to_string(),
                        start: mat.start(),
                        end: mat.end(),
                    })
                })
            });

            Ok(capture)
        });

        methods.add_meta_method(MetaMethod::Len, |_vm, _this, _: ()| {
            // Ok(this
            //     .0
            //     .captures
            //     .as_ref()
            //     .map(|m| m.len())
            //     .unwrap_or_default())
            Ok(0)
        })
    }
}
