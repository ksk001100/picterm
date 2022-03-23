use std::fs;
use std::path::PathBuf;

pub fn get_image_paths(path: &str) -> Vec<PathBuf> {
    static FILE_TYPES: [&str; 6] = ["png", "jpg", "jpeg", "webp", "bmp", "gif"];

    let paths = fs::read_dir(path).unwrap();
    let mut result = vec![];

    for path in paths {
        let path = path.unwrap().path();
        if let Some(ext) = path.extension() {
            if FILE_TYPES.iter().any(|&f| f == ext) {
                result.push(path);
            }
        }
    }

    result
}
