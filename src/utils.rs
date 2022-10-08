use std::{
    fs,
    path::{Path, PathBuf},
};

static FILE_TYPES: [&str; 6] = ["png", "jpg", "jpeg", "webp", "bmp", "gif"];

#[derive(Debug, Clone)]
pub enum RunMode {
    CLI,
    TUI,
}

#[derive(Debug, Clone)]
pub enum ImageMode {
    Rgba,
    GrayScale,
}

pub fn get_image_paths(path: &str) -> Vec<PathBuf> {
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

    result.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    result
}

pub fn select_mode(args: &[String]) -> RunMode {
    match args.len() {
        0 => RunMode::TUI,
        1 => {
            if Path::new(&args[0]).is_dir() {
                RunMode::TUI
            } else if FILE_TYPES
                .contains(&Path::new(&args[0]).extension().unwrap().to_str().unwrap())
            {
                RunMode::CLI
            } else {
                eprintln!("The argument must be a directory or a single image file.");
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("The argument must be a directory or a single image file.");
            std::process::exit(1);
        }
    }
}
