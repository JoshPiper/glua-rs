#![feature(slice_take)]

extern crate proc_macro;
use proc_macro::TokenStream;
use std::collections::VecDeque;

#[derive(Debug)]
struct EnumArg {
    raw: String,
    name: String,
    value: String
}

#[proc_macro]
pub fn glua_enum(item: TokenStream) -> TokenStream {
    if item.is_empty() {
        return "".parse().unwrap()
    }

    let arg_str = item.to_string();
    let arg_iter = arg_str.split(",");
    let mut args: VecDeque<&str> = arg_iter.collect();
    let name = args.pop_front().unwrap().trim();

    let mut parsed_args: Vec<EnumArg> = Vec::new();
    for arg in args {
        let count = arg.split("=").count();
        let mut split = arg.split("=");
        parsed_args.push(match count {
            1 => {
                let val_name = split.next().unwrap().trim();
                let mut val = String::from(name);
                val.push_str("_");
                val.push_str(val_name);

                EnumArg {
                    raw: val_name.to_string(),
                    name: val.clone(),
                    value: val.clone()
                }
            },
            2 => {
                let val_name = split.next().unwrap().trim();
                let mut val = String::from(name);
                val.push_str("_");
                val.push_str(val_name);

                let mut second: String = split.next().unwrap().parse().unwrap();
                second = second.trim().parse().unwrap();
                let mut chars = second.chars();
                chars.next();
                chars.next_back();
                second = chars.as_str().parse().unwrap();

                EnumArg {
                    raw: val_name.to_string(),
                    name: val.clone(),
                    value: second
                }
            },
            _ => panic!("Failed to parse argument to glua_enum")
        });
    }

    let mut es: Vec<String> = Vec::new();
    let mut csts: Vec<String> = Vec::new();
    let mut matches: Vec<String> = Vec::new();
    for arg in parsed_args {
        es.push(format!("\t{}", arg.raw));
        csts.push(format!("const {}: LuaString = lua_string!(\"{}\");", arg.name, arg.value));
        matches.push(format!("\t\t\t{}::{} => {}", name, arg.raw, arg.name))
    }

    let out = [
        format!("{}", csts.join("\n")),
        "#[derive(Clone)]".to_string(),
        "#[derive(Copy)]".to_string(),
        format!("#[doc=\"{} Enumeration.\"]", name),
        format!("#[doc=\"See <https://wiki.facepunch.com/gmod/Enums/{}>\"]", name),
        format!("pub enum {} {{", name),
        format!("{}", es.join(",\n")),
        "}".to_string(),
        "".to_string(),
        format!("impl HasLuaGlobal for {} {{", name),
        "\tfn global(self) -> LuaString {".to_string(),
        "\t\tmatch self {".to_string(),
        matches.join(",\n"),
        "\t\t}".to_string(),
        "\t}".to_string(),
        "}".to_string()
    ].join("\n");

    println!("{}", out);
    let val = out.parse().unwrap();
    val
}
