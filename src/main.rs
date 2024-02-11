mod wc;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
use wc::wc::process_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { println!("Please provide a filename."); return; }

    let default_args: String = String::new();
    let command: &str = args.get(1).unwrap_or(&default_args).as_str();
    let mut file_path: &str = args.get(2).unwrap_or(&default_args).as_str();

    if file_path == default_args { file_path = args.get(1).unwrap().as_str(); }

    let mut temp_filename = String::new();
    let mut show_file_path = true;

    if !fs::metadata(file_path).is_ok() {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();

        if !buffer.is_empty() {
            let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(n) => n.as_secs(),
                Err(_) => 0,
            };
            temp_filename = format!("{}_{}.txt", file_path, now);
            let mut file = fs::File::create(&temp_filename).expect("Unable to create file");
            file.write_all(buffer.as_bytes())
                .expect("Unable to write data");
            file_path = temp_filename.as_str();
            show_file_path = false;
        }
    }

    process_file(command, file_path, show_file_path);
    
    if fs::metadata(&temp_filename).is_ok() {
        fs::remove_file(temp_filename).unwrap();
    }
}
