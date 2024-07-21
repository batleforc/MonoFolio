use std::path::PathBuf;
extern crate back;

pub fn main() {
    let mut ressources_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    ressources_dir.push("../../content/");
    println!("{:?}", ressources_dir);
}
