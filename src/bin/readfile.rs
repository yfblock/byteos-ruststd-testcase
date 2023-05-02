use std::fs;

fn main() {
    match fs::read("/123") {
        Ok(data) => {
            println!("{:?}", String::from_utf8_lossy(&data));
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
    let metadata = fs::metadata("/123");
    println!("{:?}", metadata);
}