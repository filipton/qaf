const PAGES_DIR: &'static str = "pages";

use std::path::PathBuf;

fn main() {
    let mut path = PathBuf::new();
    path.push("src");
    path.push(PAGES_DIR);

    //panic!("{:?}", path);

    println!("cargo:rerun-if-changed=build.rs");
}
