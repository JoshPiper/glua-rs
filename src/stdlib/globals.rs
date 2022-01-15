use gmod::lua::{LuaReference, State};
use crate::luatypes::LuaStringable;
use crate::stdlib::enums::{FORCE, HasLuaGlobal};

/// Reads a string key from _G.
/// If the value is not a function, an error is raised and the function longjmp's out.
unsafe fn global_fn<T: LuaStringable + Display + Copy>(state: &State, name: T){
    state.get_global(name.to_lua_string());
    let func = state.is_function(-1);
    if !func {
        state.error(format!("{} could not be found or was not a function.", name));
        unreachable!();
    }
}

pub trait Globals {
    /// Adds simple accessor functions to the table at the given index.
    /// See https://wiki.facepunch.com/gmod/Global.AccessorFunc
    unsafe fn AccessorFunc(&self, tab: i32, key: i32, name: &str, force: Option<FORCE>);
}

impl Globals for State {
    /// Adds simple accessor functions to the table at the given index.
    /// See https://wiki.facepunch.com/gmod/Global.AccessorFunc
    unsafe fn AccessorFunc(&self, tab: i32, key: i32, name: &str, force: Option<FORCE>){
        global_fn(self, "AccessorFunc");
        self.push_value(tab);
        self.push_value(key);
        self.push_string(name);
        // match force {
        //     None => self.push_nil(),
        //     Some(force) => {
        //         match FORCE_LOOKUP[force] {
        //             None => self.push_nil(),
        //             Some(force) => self.get_global(force as LuaString)
        //         }
        //     }
        // }
        self.call(4, 0);
    }
}
