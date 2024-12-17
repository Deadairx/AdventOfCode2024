use std::collections::HashMap;
use std::env;
use std::fs;
use std::vec;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Please provide an input file and a mode (-p1 or -p2).");
        std::process::exit(1);
    }
    let input_file = &args[1];
    let mode = &args[2];
    let file_contents = match fs::read_to_string(input_file) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    match mode.as_str() {
        "-p1" => process_part1(file_contents),
        "-p2" => process_part2(file_contents),
        _ => {
            eprintln!("Invalid mode. Use -p1 or -p2.");
            std::process::exit(1);
        }
    }
}

fn parse_rules(raw_rules: &str) -> HashMap<usize, Vec<usize>> {
    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();

    for line in raw_rules.lines() {
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
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        rules.entry(key).or_default().extend(values);
    }

    rules
}

fn is_valid(input: &Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> bool {
    let mut prev: Vec<usize> = vec![];

    let mut valid = true;

    for num in input {
        let bad_vec = Vec::<usize>::new();
        let bad = rules.get(num).unwrap_or(&bad_vec);

        if prev.iter().any(|p| bad.contains(p)) {
            valid = false;
        }

        prev.push(*num);
    }

    valid
}

fn process_part1(contents: String) {
    let parts: Vec<&str> = contents.splitn(2, "\n\n").collect();
    if parts.len() < 2 {
        eprintln!("The file does not contain the expected delimiter.");
        std::process::exit(1);
    }
    let part1 = parts[0];
    let part2 = parts[1];

    let rules = parse_rules(part1);

    let mut sum = 0;

    for line in part2.lines() {
        let parsed_line: Vec<usize> = line
            .split(',')
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .collect();

        let valid = is_valid(&parsed_line, &rules);

        if valid {
            sum += get_mid(parsed_line);
        }
    }

    println!("{}", sum);
}

fn process_part2(contents: String) {
    let parts: Vec<&str> = contents.splitn(2, "\n\n").collect();
    if parts.len() < 2 {
        eprintln!("The file does not contain the expected delimiter.");
        std::process::exit(1);
    }
    let part1 = parts[0];
    let part2 = parts[1];

    let rules = parse_rules(part1);

    let mut sum = 0;

    for line in part2.lines() {
        let parsed_line: Vec<usize> = line
            .split(',')
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .collect();

        let valid = is_valid(&parsed_line, &rules);

        if !valid {
            let corrected_line = make_valid(parsed_line, &rules);

            sum += get_mid(corrected_line);
        }
    }

    println!("{}", sum);
}

fn make_valid(mut parsed_line: Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut valid = false;

    // println!("Fixing: {:?}", parsed_line);

    while !valid {
        let mut prev: Vec<usize> = vec![];
        let clone_line = parsed_line.clone();

        for num in clone_line {
            let bad_vec = Vec::<usize>::new();
            let bad = rules.get(&num).unwrap_or(&bad_vec);

            if let Some((index, problem)) = prev.iter().enumerate().find(|(_, p)| bad.contains(*p))
            {
                parsed_line.remove(index);
                parsed_line.push(*problem);
                break;
            }

            prev.push(num);
        }

        valid = is_valid(&parsed_line, rules);
    }

    parsed_line
}

fn get_mid(vec: Vec<usize>) -> usize {
    let mid = vec.len() / 2;
    vec[mid]
}
