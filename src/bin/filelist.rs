use std::{path::PathBuf, fs};

fn main() {
    let dir_path = PathBuf::from("/");
    match fs::read_dir(dir_path) {
        Ok(filelist) => {
            for entry in filelist {
                let path = entry.unwrap().path();
                if path.is_file() {
                    println!("{}", path.display());
                }
            }
        },
        Err(e) => {
            println!("error: {:?}", e);
        },
    }
    
}