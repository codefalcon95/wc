mod wc;
use std::env;
use wc::wc::process_file;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a filename.");
        return;
    }
    let filename: &str = &args[1];
    let commands: Vec<&str> = args.iter().skip(2).map(String::as_str).collect();

   process_file(filename, commands);

}
