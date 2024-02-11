pub mod wc {
    use std::fs;
    use std::io::prelude::*;
    use std::io::BufReader;

    #[derive(PartialEq)]
    enum Command {
        Line,
        Word,
        Size,
        Character,
        All,
    }

    pub fn process_file(command: &str, file_path: &str, show_file_path: bool) {
        let command = process_wc_command(command);
        let file_path_to_print = if show_file_path { file_path } else { "" };
        match command {
            Command::Size => {
                let file_size = get_file_size_in_byte(file_path);
                println!("{} {}", file_size, file_path_to_print);
            }
            Command::Line => {
                let lines = get_lines_in_file(file_path);
                println!("{:?} {}", lines, file_path_to_print);
            }
            Command::Word => {
                let words = get_words_in_file(file_path);
                println!("{:?} {}", words, file_path_to_print);
            }
            Command::Character => {
                let characters = get_characters_in_file(file_path);
                println!("{:?} {}", characters, file_path_to_print);
            }
            _ => {
                let file_size = get_file_size_in_byte(file_path);
                let lines = get_lines_in_file(file_path);
                let words = get_words_in_file(file_path);
                println!(" {} {} {} {}", lines, words, file_size, file_path_to_print);
            }
        }
    }

    fn get_file_size_in_byte(file_path: &str) -> u64 {
        let f = match fs::metadata(file_path) {
            Ok(metadata) => metadata,
            Err(e) => {
                println!("Error: {}", e);
                return 0;
            }
        };
        f.len()
    }

    fn process_wc_command(command: &str) -> Command {
        match command {
            "-l" => Command::Line,
            "-w" => Command::Word,
            "-c" => Command::Size,
            "-m" => Command::Character,
            _ => Command::All,
        }
    }

    fn get_lines_in_file(file_path: &str) -> usize {
        let f = fs::File::open(file_path).unwrap();
        let reader = BufReader::new(f);
        let total_lines = reader.lines().count();
        total_lines
    }

    fn get_words_in_file(file_path: &str) -> usize {
        let f = fs::File::open(file_path).unwrap();
        let mut reader = BufReader::new(f);
        let mut buff = String::new();
        let _ = reader.read_to_string(&mut buff);
        buff.split_whitespace().count()
    }

    fn get_characters_in_file(file_path: &str) -> usize {
        let f = fs::File::open(file_path).unwrap();
        let mut reader = BufReader::new(f);
        let mut buff = String::new();
        let _ = reader.read_to_string(&mut buff);
        buff.chars().count()
    }
}
