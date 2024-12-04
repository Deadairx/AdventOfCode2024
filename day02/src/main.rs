use std::fs::File;
use std::io::{self, BufRead};

use std::str::FromStr;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut arrays: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| i32::from_str(s).ok())
            .collect();
        arrays.push(numbers);
    }

    let mut count = 0;

    for array in &arrays {
        let mut dec = false;
        let mut inc = false;
        let mut safe = true;

        if let Some(first) = array.first() {
            let mut previous = *first;
            for &num in array.iter().skip(1) {
                if num > previous {
                    inc = true;
                    if num - previous > 3 {
                        safe = false;
                    }
                } else if num < previous {
                    dec = true;
                    if previous - num > 3 {
                        safe = false;
                    }
                } else if num == previous {
                    safe = false;
                }

                if inc && dec {
                    safe = false;
                }

                previous = num;
            }
        }

        if safe {
            count += 1;
        }
    }

    println!("{}", count);

    Ok(())
}
