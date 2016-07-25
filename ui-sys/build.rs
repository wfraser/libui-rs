extern crate cmake;
extern crate make_cmd;

use std::path::Path;
use std::process::Command;

fn main() {
    if !Path::new("libui/.git").exists() {
        Command::new("git").args(&["submodule", "update", "--init"]).status().unwrap();
    }

    let dst = cmake::Config::new("libui")
                            .build_target("")
                            .build();
    let out_dir = dst.display();

    println!("cargo:rustc-link-lib=dylib=ui");
    println!("cargo:rustc-link-search=native={}", out_dir);
}
