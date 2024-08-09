use back::{api::init::init_api, config};
use markdown_struct::{
    blog_timeline::BlogTimeline, doc_sidebar::DocCategory, folder_struct, page_database::DbFolder,
};
use tool_tracing::init::init_tracing;
use tracing::{error, info};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = config::parse_local_config();
    init_tracing(config.clone().tracing, config.get_name());

    // Init ContentDB
    let folder_struct = match folder_struct::process_folder_struct(config.content_path.clone()) {
        Ok(folder_struct) => folder_struct,
        Err(e) => {
            error!("Error processing folder struct: {:?}", e);
            return Ok(());
        }
    };
    let db_folder = match DbFolder::generate_database(folder_struct) {
        Ok(db_folder) => db_folder,
        Err(e) => {
            error!("Error generating database: {:?}", e);
            return Ok(());
        }
    };
    let blog_timeline = BlogTimeline::generate_timeline_from_db(db_folder.clone(), "".to_string());
    let doc_sidebar = DocCategory::generate_sidebar_from_db(db_folder.clone(), "".to_string());
    init_api(config.port, db_folder, blog_timeline, doc_sidebar).await?;
    info!("API stopped {}", config.get_name());
    Ok(())
}
