// build.rs

use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    Command::new("npx")
        .args(&["webpack"])
        .current_dir(&Path::new(&"./ui"))
        .status().unwrap();

    println!("warning=dssdfsdfsdfsdfs");
}