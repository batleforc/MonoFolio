use markdown_parser::read_file;
extern crate back;

use back::markdown::doc_header::DocHeader;

pub fn main() {
    let origin = read_file("../../content/project/portfolio.md");
    let content = match origin {
        Ok(content) => content,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    match serde_yaml::from_str::<DocHeader>(content.front_matter()) {
        Ok(md) => {
            println!("{:?}", md);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    // let md = content.adapt::<YamlAdapter, DocHeader>();
    // match md {
    //     Ok(md) => {
    //         println!("{:?}", md);
    //     }
    //     Err(e) => {
    //         println!("Error: {}", e);
    //     }
    // }
}
