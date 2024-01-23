#![allow(dead_code)]

use ryst_lib::run_rsync;
use std::path::PathBuf;

fn main() {
    let mut path = PathBuf::new();
    path.push(".ryst");
    if path.as_path().exists() {
        println!("path exists = {:?}", path);
    }
    let path_str = path.to_str().ok_or("Failed to convert PathBuf to str");
    let _output = run_rsync(path_str.unwrap());
}
