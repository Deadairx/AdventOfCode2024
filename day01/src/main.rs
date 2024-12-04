use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    for line in contents.lines() {
        let mut numbers = line.split_whitespace();
        if let (Some(num1), Some(num2)) = (numbers.next(), numbers.next()) {
            if let (Ok(n1), Ok(n2)) = (num1.parse::<i32>(), num2.parse::<i32>()) {
                column1.push(n1);
                column2.push(n2);
            }
        }
    }

    column1.sort();
    column2.sort();

    let mut sum = 0;

    for &n1 in &column1 {
        let count: i32 = column2.iter().filter(|&&n2| n2 == n1).count().try_into().unwrap();

        sum += n1 * count;
    }

    println!("{}", sum);

    Ok(())
}
