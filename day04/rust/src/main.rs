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

    for (row, row_map) in &char_map {
        for (col, c) in row_map {
            if *c == 'X' {
                // Check all 8 surrounding positions
                for d_row in -1..=1 {
                    for d_col in -1..=1 {
                        if d_row == 0 && d_col == 0 {
                            continue; // Skip the current position
                        }

                        let neighbor_row = (*row as isize + d_row) as usize;
                        let neighbor_col = (*col as isize + d_col) as usize;

                        if let Some(neighbor_row_map) = char_map.get(&neighbor_row) {
                            if let Some(&neighbor_c) = neighbor_row_map.get(&neighbor_col) {
                                if neighbor_c== 'M' {
                                    let neighbor_row2 = (neighbor_row as isize + d_row) as usize;
                                    let neighbor_col2 = (neighbor_col as isize + d_col) as usize;
                                    if let Some(neighbor_row_map2) = char_map.get(&neighbor_row2) {
                                        if let Some(&neighbor_c2) = neighbor_row_map2.get(&neighbor_col2) {
                                            if neighbor_c2== 'A'{
                                                let neighbor_row3 = (neighbor_row2 as isize + d_row) as usize;
                                                let neighbor_col3 = (neighbor_col2 as isize + d_col) as usize;
                                                if let Some(neighbor_row_map3) = char_map.get(&neighbor_row3) {
                                                    if let Some(&neighbor_c3) = neighbor_row_map3.get(&neighbor_col3) {
                                                        if neighbor_c3== 'S'{
                                                            count += 1;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);
}
