use eyre::Result;
use std::path::PathBuf;
use tokio::fs;

pub async fn get_image_paths(path: &str) -> Result<Vec<PathBuf>> {
    static FILE_TYPES: [&str; 3] = ["png", "jpg", "jpeg"];

    let mut entries = fs::read_dir(path).await.unwrap();
    let mut result = vec![];

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if FILE_TYPES.iter().any(|&f| f == ext) {
                result.push(path);
            }
        }
    }

    Ok(result)
}
