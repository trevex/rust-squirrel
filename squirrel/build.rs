#![feature(path)]
#![feature(os)]
#![feature(collections)]

extern crate gcc;


#[cfg(target_arch = "x86_64")]
fn get_definitions() -> Vec<(String, Option<String>)> {
    vec![("_SQ64".to_string(), None)]
}

#[cfg(not(target_arch = "x86_64"))]
fn get_definitions() -> Vec<(String, Option<String>)> {
    vec![]
}

fn main() {
    println!("cargo:rustc-flags=-l dylib=stdc++");

    let root = Path::new(std::os::getenv("CARGO_MANIFEST_DIR").unwrap())
        .join("squirrel");

    let config = gcc::Config {
        include_directories: vec![
            root.join("include"),
        ],
        definitions: get_definitions(),
        objects: vec![],
        flags: vec![
            "-fno-exceptions".to_string(),
            "-fno-rtti".to_string(),
            "-fno-strict-aliasing".to_string()
        ],
    };

    println!("cargo:include={}", root.join("include").display());

    gcc::compile_library("libsquirrel.a", &config, &[
        "squirrel/src/sqapi.cpp",
        "squirrel/src/sqbaselib.cpp",
        "squirrel/src/sqfuncstate.cpp",
        "squirrel/src/sqdebug.cpp",
        "squirrel/src/sqlexer.cpp",
        "squirrel/src/sqobject.cpp",
        "squirrel/src/sqcompiler.cpp",
        "squirrel/src/sqstate.cpp",
        "squirrel/src/sqtable.cpp",
        "squirrel/src/sqmem.cpp",
        "squirrel/src/sqvm.cpp",
        "squirrel/src/sqclass.cpp",
        "squirrel/std/sqstdblob.cpp",
        "squirrel/std/sqstdio.cpp",
        "squirrel/std/sqstdstream.cpp",
        "squirrel/std/sqstdmath.cpp",
        "squirrel/std/sqstdsystem.cpp",
        "squirrel/std/sqstdstring.cpp",
        "squirrel/std/sqstdaux.cpp",
        "squirrel/std/sqstdrex.cpp"
    ]);
}
