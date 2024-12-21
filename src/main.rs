use std::{fs, path::Path};

/// Check if the file path exist or not
///
/// [Return True if exist] - [Return False if not exist]
fn is_path_exist(file_path: &str) -> bool {
    let path_exist = Path::new(file_path).try_exists();

    match path_exist {
        Ok(b) => b,
        Err(_) => false,
    }
}

fn search_pattern(pattern: &str, text: &str) -> i64 {
    let text_len = text.len();
    let pattern_len = pattern.len();

    if pattern_len > text_len {
        println!("Pattern is longer than the text.");
        return -1;
    }

    for i in 0..=(text_len - pattern_len) {
        let mut match_found = true;

        for j in 0..pattern_len {
            if text.as_bytes()[i + j] != pattern.as_bytes()[j] {
                match_found = false;
                break;
            }
        }

        if match_found {
            return i as i64;
        }
    }
    -1
}

fn main() {
    let file_path = "test.txt";

    match is_path_exist(&file_path) {
        true => {
            let contents = fs::read_to_string(file_path);

            match contents {
                Ok(c) => {
                    let pattern = "TEST";
                    let text = c;

                    let result = search_pattern(pattern, &text);

                    println!("Result Index: {}", result);
                }
                Err(e) => {
                    println!("Error reading file: {}", e);
                }
            }
        }
        false => {
            println!("File [{}] does not exist", &file_path)
        }
    }
}
