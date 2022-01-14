
/// A trait which shows a rust enum has a equivalent lua enum, stored as a string key in _G
pub trait HasLuaGlobal {
    /// Fetch the _G key for this enum value.
    fn global(self) -> &'static str;
}

pub enum FORCE {
    STRING = 0x01,
    NUMBER = 0x02,
    BOOL = 0x03,
    ANGLE = 0x04,
    COLOR = 0x05,
    VECTOR = 0x06
}

impl HasLuaGlobal for FORCE {
    fn global(self) -> &'static str {
        match self {
            FORCE::STRING => "FORCE_STRING",
            FORCE::NUMBER => "FORCE_NUMBER",
            FORCE::BOOL => "FORCE_BOOL",
            FORCE::ANGLE => "FORCE_ANGLE",
            FORCE::COLOR => "FORCE_COLOR",
            FORCE::VECTOR => "FORCE_VECTOR",
        }
    }
}
