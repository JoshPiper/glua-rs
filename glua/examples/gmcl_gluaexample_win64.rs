#![feature(c_unwind)]

#![macro_use] extern crate gmod;
extern crate glua;

use gmod::gmcl::override_stdout;
use gmod::gmod13_open;
use gmod::lua_string;
use glua::{State};
use glua::stdlib::enums::{FORCE};
use glua::stdlib::globals::Globals;


#[gmod13_open]
unsafe fn gmod13_open(lua: State) -> i32 {
    override_stdout();
    println!("opening module");

    // Make a new table.
    lua.new_table();
    let tab_ref = lua.reference();

    // Call AccessorFunc
    println!("setting test_key to test_func on table");
    lua.AccessorFunc(tab_ref, "test_key", "test_func", FORCE::STRING);

    // Finally, set our table in _G
    lua.from_reference(tab_ref);
    lua.dereference(tab_ref);
    lua.set_global(lua_string!("test_table"));
    0
}
