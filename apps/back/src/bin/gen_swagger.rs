use std::{env, fs};

use back::api::apidocs::ApiDocs;
use utoipa::OpenApi;

extern crate back;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("Usage: gen_swagger <output_file>");
            std::process::exit(1);
        }
        _ => {}
    }
    let mut openapi = ApiDocs::openapi();
    openapi.info.version = env!("CARGO_PKG_VERSION").to_string();
    fs::write(args[1].clone(), openapi.to_pretty_json().unwrap()).expect("Unable to write file");
}
