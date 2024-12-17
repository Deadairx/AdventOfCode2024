use std::{env, fs, collections::HashMap};

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

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn change_dir(self) -> Self {
        match self {
            Direction::North => Self::East,
            Direction::East => Self::South,
            Direction::South => Self::West,
            Direction::West => Self::North,
        }
    }

    fn move_coordinates(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1,0),
            Direction::East => (0,1),
            Direction::South => (1,0),
            Direction::West => (0,-1),
        }
    }
}

struct GuardMap {
    map: HashMap<(isize, isize), bool>,
    start_pos: (isize, isize),
    start_dir: Direction,
}

impl GuardMap {
    fn populate_map(input: String) -> GuardMap {
        let mut start_pos: (isize, isize) = (0, 0);
        let mut map: HashMap<(isize, isize), bool> = HashMap::new();

        for (x, line) in input.lines().enumerate() {
            for (y, char) in line.chars().enumerate() {
                map.insert((x as isize, y as isize), char == '#');
                if char == '^' {
                    start_pos = (x as isize, y as isize);
                }
            }
        }

        GuardMap { map, start_pos, start_dir: Direction::North }
    }

    fn get_pos(&self, current_pos: (isize, isize)) -> Grid {
        match self.map.get(&current_pos) {
            Some(obstructed) => {
                if *obstructed { Grid::Obstructed }
                else { Grid::Free }
            },
            None => Grid::OffGrid,
        }
    }
}

fn process_part1(input: String) {
    let map = GuardMap::populate_map(input);
    let mut current_pos = map.start_pos;
    let mut current_dir = map.start_dir;
    let mut visited: HashMap<(isize, isize), bool> = HashMap::new();
    let mut on_grid = true;

    while on_grid {
        visited.insert(current_pos, true);

        if map.get_pos(next_move(current_pos, &current_dir)) == Grid::Obstructed {
            current_dir = current_dir.change_dir();
        }

        if map.get_pos(next_move(current_pos, &current_dir)) == Grid::OffGrid {
            on_grid = false;
        }

        current_pos = next_move(current_pos, &current_dir);
    }

    println!("{}", visited.len());
}

#[derive(PartialEq, Eq)]
enum Grid {
    Free,
    Obstructed,
    OffGrid,
}

fn next_move(current_pos: (isize, isize), current_dir: &Direction) -> (isize, isize) {
    let (x, y) = current_dir.move_coordinates();
    (current_pos.0 + x, current_pos.1 + y)
}

fn process_part2(input: String) {
    todo!()
}
