use std::collections::HashMap;
use std::env;
use std::fs;
use std::vec;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide an input file.");
        std::process::exit(1);
    }
    let input_file = &args[1];
    let file_contents = match fs::read_to_string(input_file) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    let parts: Vec<&str> = file_contents.splitn(2, "\n\n").collect();
    if parts.len() < 2 {
        eprintln!("The file does not contain the expected delimiter.");
        std::process::exit(1);
    }
    let part1 = parts[0];
    let part2 = parts[1];

    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();

    for line in part1.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 2 {
            eprintln!("Invalid rule format: {}", line);
            continue;
        }

        let key: usize = match parts[0].trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid key in rule: {}", parts[0]);
                continue;
            }
        };

        let values: Vec<usize> = parts[1]
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        rules.entry(key).or_insert_with(Vec::new).extend(values);
    }

    let mut sum = 0;

    for line in part2.lines() {
        let parsed_line = line.split(',').filter_map(|s| s.trim().parse::<usize>().ok());
        let mut prev: Vec<usize> = vec![];

        let mut valid = true;

        for num in parsed_line {
            let bad_vec = Vec::<usize>::new();
            let bad = rules.get(&num).unwrap_or(&bad_vec);

            if prev.iter().any(|p| bad.contains(p)) {
                // TODO: this line is invalid, break this loop and continue to next line
                valid = false;
            }

            prev.push(num);
        }

        if valid {
            sum += get_mid(prev);
        }
    }

    println!("{}", sum);
}

fn get_mid(vec: Vec<usize>) -> usize {
    let mid = vec.len() / 2;
    vec[mid]
}

