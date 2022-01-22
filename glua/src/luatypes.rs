use std::ffi::{CString};
use gmod::lua::{LuaString};

/// Trait to allow converting to a lua string.
pub trait LuaStringable {
    /// Converts to a lua string.
    fn to_lua_string(self) -> LuaString;
}

impl LuaStringable for CString {
    fn to_lua_string(self) -> LuaString {
        self.as_ptr()
    }
}

impl LuaStringable for &str {
    fn to_lua_string(self) -> LuaString {
        let str = CString::new(self).unwrap_or_default();
        str.as_ptr()
    }
}

impl LuaStringable for String {
    fn to_lua_string(self) -> LuaString {
        let str = CString::new(self).unwrap_or_default();
        str.as_ptr()
    }
}

impl LuaStringable for LuaString {
    fn to_lua_string(self) -> LuaString {
        self
    }
}
