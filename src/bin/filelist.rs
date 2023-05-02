use std::{path::PathBuf, fs};

fn main() {
    let dir_path = PathBuf::from("/");
    match fs::read_dir(dir_path) {
        Ok(filelist) => {
            println!("start to print filelist");
            for entry in filelist {
                let file =  entry.unwrap();
                if file.file_type().unwrap().is_file() {
                    println!("file: {}  file_size: {}", file.path().display(), file.metadata().unwrap().len());
                } else {
                    println!("dir: {}", file.path().display());
                }
            }
        },
        Err(e) => {
            println!("error: {:?}", e);
        },
    }
}