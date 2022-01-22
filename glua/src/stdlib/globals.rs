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
    unsafe fn AccessorFunc<F: Into<Option<FORCE>> + Display>(&self, tab: LuaReference, key: &str, name: &str, force: F);
}

impl Globals for State {
    #[allow(non_snake_case)]
    unsafe fn AccessorFunc<F: Into<Option<FORCE>> + Display>(&self, tab: LuaReference, key: &str, name: &str, force: F){
        global_fn!(self, "AccessorFunc");
        println!("got table ref: {}", tab);
        self.from_reference(tab);
        println!("got key: {}", key);
        self.push_string(key);
        println!("got func name: {}", name);
        self.push_string(name);

        println!("got force state: {}", force);
        let forced = force.into();
        match forced {
            None => self.push_nil(),
            Some(ref force) => self.get_global(force.global())
        }

        if forced.is_some() {
            let f = forced.unwrap();
            println!("unwrapped: {}", f);
            let global = f.global();
            println!("global: {:?}", global);

            self.get_global(f.global());
            println!("{}", self.get_type(-1));

            let v = self.check_number(-1);
            println!("value: {}", v);
            self.pop();
        }

        self.call(4, 0);
    }
}
