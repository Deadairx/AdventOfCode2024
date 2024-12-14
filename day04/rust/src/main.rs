use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    // Collect the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a file path is provided
    if args.len() < 2 {
        eprintln!("Please provide a file path as an argument.");
        return;
    }

    // Get the file path from the arguments
    let file_path = &args[1];

    // Read the file content and store it in a variable
    let file_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut char_map: HashMap<usize, HashMap<usize, char>> = HashMap::new();

    for (row, line) in file_content.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let row_map = char_map.entry(row).or_insert_with(HashMap::new);
            row_map.insert(col, c);
        }
    }

    let mut count = 0;

    fn check_sequence(
        char_map: &HashMap<usize, HashMap<usize, char>>,
        start_row: usize,
        start_col: usize,
        d_row: isize,
        d_col: isize,
    ) -> bool {
        let sequence = ['M', 'A', 'S'];
        let mut current_row = start_row;
        let mut current_col = start_col;

        for &ch in &sequence {
            current_row = (current_row as isize + d_row) as usize;
            current_col = (current_col as isize + d_col) as usize;

            if let Some(row_map) = char_map.get(&current_row) {
                if let Some(&c) = row_map.get(&current_col) {
                    if c != ch {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    for (row, row_map) in &char_map {
        for (col, c) in row_map {
            if *c == 'X' {
                // Check all 8 surrounding positions
                for d_row in -1..=1 {
                    for d_col in -1..=1 {
                        if d_row == 0 && d_col == 0 {
                            continue; // Skip the current position
                        }

                        if check_sequence(&char_map, *row, *col, d_row, d_col) {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);
}
