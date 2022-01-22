use std::fmt::{Display, Formatter};
use gmod::lua::LuaString;
use glua_macros::glua_enum;

/// A trait which shows a rust enum has a equivalent lua enum, stored as a string key in _G
pub trait HasLuaGlobal {
    /// Fetch the _G key for this enum value.
    fn global(self) -> LuaString;
}

glua_enum!(FORCE, STRING, NUMBER, BOOL, ANGLE, COLOR, VECTOR);
glua_enum!(ACT, INVALID, RESET, IDLE, TRANSITION, COVER, COVER_MED, COVER_LOW, WALK, WALK_AIM, WALK_CROUCH, WALK_CROUCH_AIM);

