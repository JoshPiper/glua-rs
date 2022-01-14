#![feature(c_unwind)]
#![feature(const_for)]

pub mod luatypes;
pub mod stdlib;


pub use gmod::lua::{State};
use stdlib::globals::{};

#[macro_use] extern crate gmod;

pub enum GLuaRealm {
    Client,
    Server,
    Menu,
    Unknown
}

pub trait ExtendedLuaState {
    /// Returns if this is the menu state or not.
    unsafe fn is_menu(&self) -> bool;

    /// Returns an enum, representing the current lua realm.
    unsafe fn realm(&self) -> GLuaRealm;
}

impl ExtendedLuaState for State {
    unsafe fn is_menu(&self) -> bool {
        self.get_global(lua_string!("MENU_DLL"));
        let in_menu = self.get_boolean(-1);
        self.pop();
        in_menu
    }

    unsafe fn realm(&self) -> GLuaRealm {
        let mn = self.is_menu();
        if mn {
            return GLuaRealm::Menu;
        };

        let cl = self.is_client();
        if cl {
            return GLuaRealm::Client;
        }

        let sv = self.is_server();
        if sv {
            return GLuaRealm::Server;
        }

        GLuaRealm::Unknown
    }
}
