use gmod::lua::{LuaString};

pub trait LuaStringable {
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
