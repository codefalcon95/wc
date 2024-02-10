
pub mod wc {
    use std::fs;

  pub fn process_file(file_path: &str, commands: Vec<&str>) {
    let file_size = get_file_size_in_byte(file_path);
    println!("{} {}", file_size, file_path);
  }

  fn get_file_size_in_byte(file_path: &str) -> u64 {
    let f = match fs::metadata(file_path) {
      Ok(metadata) => {
        metadata
      },
      Err(e) => {
        println!("Error: {}", e);
        return 0;
      }

   }; 

    return f.len();
  }
}