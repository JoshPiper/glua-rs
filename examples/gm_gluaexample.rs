#![feature(c_unwind)]

#![macro_use] extern crate gmod;
extern crate glua;

use gmod::gmod13_open;
use gmod::lua_string;
use glua::{State, ExtendedLuaState};
use glua::stdlib::enums::{FORCE, HasLuaGlobal};
use glua::stdlib::globals::Globals;


#[gmod13_open]
unsafe fn gmod13_open(lua: State) -> i32 {
    // Make a new table.
    lua.new_table();
    let tab_ref = lua.reference();
    lua.from_reference(tab_ref);

    // The key which we'll be filling.
    lua.push_string("test_key");

    // Call AccessorFunc
    lua.AccessorFunc(-2, -1, "test_func", Some(FORCE::STRING));

    // Finally, set our table in _G
    lua.from_reference(tab_ref);
    lua.dereference(tab_ref);
    lua.set_global(lua_string!("test_table"));
    0
}
