use std::path::PathBuf;

use markdown_struct::{
    content_struct::PageV2, folder_struct::process_folder_struct, mdast_to_schema::Node,
    page_database::DbFolder,
};
extern crate back;

pub fn main() {
    let mut ressources_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    ressources_dir.push("../../folio_content/content/");
    println!("{:?}", ressources_dir);
    let content_folder =
        match process_folder_struct(ressources_dir.clone().to_str().unwrap().to_string()) {
            Ok(folder) => folder,
            Err(e) => {
                eprintln!("Error: {:?}", e);
                return;
            }
        };

    let db_folder = match DbFolder::generate_database(content_folder) {
        Ok(folder) => folder,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            return;
        }
    };
    ressources_dir.push("..");
    // create folder for mermaid diagrams
    let path = PathBuf::from(format!(
        "{}/media/mermaid",
        ressources_dir.into_os_string().into_string().unwrap()
    ));
    process_folder_struct_recursive(db_folder, path.to_str().unwrap().to_string());
}

pub fn process_folder_struct_recursive(folder: DbFolder, path: String) {
    for page in folder.pages {
        println!("Processing page: {} => {}", page.name.clone(), path.clone());
        process_page(&page, path.clone());
    }
    for (name, sub_folder) in folder.sub_folders {
        let mut path = PathBuf::from(path.clone());
        path.push(name);
        process_folder_struct_recursive(sub_folder, path.to_str().unwrap().to_string());
    }
}

// create a function that process a page, during the process detect if the page has a mermaid diagram
// for each mermaid diagram, generate a mmd file named after the page and folder
pub fn process_page(page: &PageV2, path: String) {
    let mut mmd_files = vec![];
    let mut mmd_content = vec![];
    let mut mmd_index = 0;
    // test if page.parsed_content is of type Root
    if let Node::Root(truc) = &page.parsed_content {
        // do nothing
        for child in truc.children.clone() {
            process_page_recursive(
                child.clone(),
                page.name.clone(),
                &mut mmd_index,
                &mut mmd_files,
                &mut mmd_content,
            );
        }
    } else {
        return;
    }

    if mmd_files.len() > 0 {
        std::fs::create_dir_all(format!("{}/{}", path.clone(), page.clone().name)).unwrap();
    }

    for (mmd_file, mmd_content) in mmd_files.iter().zip(mmd_content.iter()) {
        let mut path = PathBuf::from(path.clone());
        path.push(page.name.clone());
        path.push(mmd_file);
        println!("Writing file: {}", path.to_str().unwrap().to_string());
        std::fs::write(path, mmd_content).unwrap();
    }
}

fn process_page_recursive(
    node: Node,
    path: String,
    mmd_index: &mut usize,
    mmd_files: &mut Vec<String>,
    mmd_content: &mut Vec<String>,
) {
    let next_path: String = path.clone();
    if let Node::Code(code) = node.clone() {
        if code.lang == Some("mermaid".to_string()) {
            let mmd_file = format!("{}_{}.mmd", path, mmd_index);
            mmd_files.push(mmd_file);
            mmd_content.push(code.value);
            *mmd_index += 1;
        }
    }
    if let Some(iter) = node.get_iter_children() {
        for child in iter {
            let path_to_push = next_path.clone();
            process_page_recursive(
                child.clone(),
                path_to_push,
                mmd_index,
                mmd_files,
                mmd_content,
            );
        }
    }
}
