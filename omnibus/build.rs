use std::{env, process::Command};

fn main() {
    //cc::Build::new()
    //    .file("src/omnibus.c")
    //    .compile("omnibus");

// ----

    //let out_dir = env::var("OUT_DIR").unwrap();

    //println!("\n1: out_dir={out_dir}\n");

    //let mut gcc = cc::Build::new();
    ////gcc.cpp(true);
    //gcc.include(".");
    //let mut cmd = gcc.get_compiler().to_command();
    //cmd
    //    //.arg("-o")
    //    //.arg("{out_dir}/omnibus")
    //    //.arg("omnibus.executable")
    //    .arg("src/omnibus.c");
    //assert!(cmd.status().unwrap().success());

    //println!("\n2: out_dir={out_dir}\n");


    //let out_dir = env::var("OUT_DIR").unwrap();

    // Note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.

// ----

    let out_dir = env::var("OUT_DIR").unwrap();

    //println!("\n1: out_dir={out_dir}\n");

    let mut cmd = Command::new("gcc");
    let cmd = cmd.args(&["src/omnibus.c", "-fPIC", "-o"]);
    let cmd = cmd.arg(&format!("{}/omnibus", out_dir));

    println!("cmd: {cmd:?}");

    cmd.status().unwrap();
    //Command::new("ar").args(&["crus", "libhello.a", "hello.o"])
    //                  .current_dir(&Path::new(&out_dir))
    //                  .status().unwrap();

    //println!("cargo:rustc-link-search=native={}", out_dir);
    //println!("cargo:rustc-link-lib=static=hello");
    //println!("cargo:rerun-if-changed=src/hello.c");

    println!("cargo:rerun-if-changed=src/omnibus.c");

}
