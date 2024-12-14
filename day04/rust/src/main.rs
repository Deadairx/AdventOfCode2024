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
    ) -> bool {
        let sequence = [(-1, -1), (-1, 1)];

        let mut cross_count = 0;

        fn matches_target(
            char_map: &HashMap<usize, HashMap<usize, char>>,
            row: usize,
            col: usize,
            target: char,
        ) -> bool {
            if let Some(row_map) = char_map.get(&row) {
                if let Some(&c) = row_map.get(&col) {
                    return c == target;
                }
            }
            false
        }

        for (d_row, d_col) in &sequence {
            let current_row = (start_row as isize + d_row) as usize;
            let current_col = (start_col as isize + d_col) as usize;
            let opposite_row = (start_row as isize - d_row) as usize;
            let opposite_col = (start_col as isize - d_col) as usize;

            if matches_target(char_map, current_row, current_col, 'M') 
                && matches_target(char_map, opposite_row, opposite_col, 'S') {
                cross_count += 1;
            }

            if matches_target(char_map, current_row, current_col, 'S') 
                && matches_target(char_map, opposite_row, opposite_col, 'M') {
                cross_count += 1;
            }
        }

        cross_count == 2
    }

    for (row, row_map) in &char_map {
        for (col, c) in row_map {
            if *c == 'A' {
                if check_sequence(&char_map, *row, *col) {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
