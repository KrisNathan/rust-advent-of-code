use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("inputs/input2")?;
    let reader = BufReader::new(file);

    let num: u32 = reader
        .lines()
        .enumerate()
        .map(|(i, line)| parse(&line.unwrap()) * (i + 1) as u32)
        .sum();

    println!("{num}");
    Ok(())
}

fn parse(line: &String) -> u32 {
    let (_, sets) = line.split_once(": ").unwrap();
    for set in sets.split("; ") {
        for cube in set.split(", ") {
            let (amount, color) = cube.split_once(" ").unwrap();
            let max = match color {
                "red" => 12,
                "green" => 13,
                "blue" => 14,
                _ => 0,
            };
            if amount.parse::<u32>().unwrap() > max {
                return 0;
            }
        }
    }
    1
}
