use gmod::lua::{LuaString};

/// Trait to allow converting to a lua string.
pub trait LuaStringable {
    /// Converts to a lua string.
    fn to_lua_string(self) -> LuaString;
}

impl LuaStringable for &str {
    fn to_lua_string(self) -> LuaString {
        self.as_ptr() as LuaString
    }
}

impl LuaStringable for String {
    fn to_lua_string(self) -> LuaString {
        self.as_ptr() as LuaString
    }
}

impl LuaStringable for LuaString {
    fn to_lua_string(self) -> LuaString {
        self
    }
}
