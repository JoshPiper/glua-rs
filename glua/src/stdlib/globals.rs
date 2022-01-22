use std::fmt::Display;
use gmod::lua::{LuaReference, State};
use crate::stdlib::enums::{FORCE, HasLuaGlobal};

/// Reads a string key from _G.
/// If the value is not a function, an error is raised and the function longjmp's out.
macro_rules! global_fn {
    ($state:ident, $name:literal) => {
        $state.get_global(lua_string!($name));
        let func = $state.is_function(-1);
        if !func {
            $state.error(concat!($name, " could not be found or was not a function."));
        }
    }
}

pub trait Globals {
    /// Adds simple accessor functions to the table at the given index.
    /// See <https://wiki.facepunch.com/gmod/Global.AccessorFunc>
    #[allow(non_snake_case)]
    unsafe fn AccessorFunc<F: Into<Option<FORCE>>>(&self, tab: LuaReference, key: &str, name: &str, force: F);

    /// Adds a file to the client download list.
    /// NOOP on the Client.
    /// See <https://wiki.facepunch.com/gmod/Global.AddCSLuaFile>
    #[allow(non_snake_case)]
    unsafe fn AddCSLuaFile<F: Into<Option<String>>>(&self, file: F);
}

impl Globals for State {
    #[allow(non_snake_case)]
    unsafe fn AccessorFunc<F: Into<Option<FORCE>>>(&self, tab: LuaReference, key: &str, name: &str, force: F){
        global_fn!(self, "AccessorFunc");
        self.from_reference(tab);
        self.push_string(key);
        self.push_string(name);

        let forced = force.into();
        match forced {
            None => self.push_nil(),
            Some(ref force) => self.get_global(force.global())
        }

        self.call(4, 0);
    }

    #[allow(non_snake_case)]
    unsafe fn AddCSLuaFile<F: Into<Option<String>>>(&self, file: F){
        global_fn!(self, "AddCSLuaFile");
        let filed = file.into();
        match filed {
            None => self.push_nil(),
            Some(ref file) => self.push_string(file)
        }

        self.call(1, 0);
    }
}
