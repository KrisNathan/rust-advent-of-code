use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("inputs/input2")?;
    let reader = BufReader::new(file);

    let num: u32 = reader.lines().map(|line| parse(&line.unwrap())).sum();

    println!("{num}");
    Ok(())
}

fn parse(line: &String) -> u32 {
    let mut red: u32 = 1;
    let mut green: u32 = 1;
    let mut blue: u32 = 1; // not sure how to deal with color that doesnt exist
    let (_, sets) = line.split_once(": ").unwrap();
    for set in sets.split("; ") {
        for cube in set.split(", ") {
            let (amount_str, color) = cube.split_once(" ").unwrap();
            let amount = amount_str.parse::<u32>().unwrap();
            match color {
                "red" => {
                    if amount > red {
                        red = amount;
                    }
                }
                "green" => {
                    if amount > green {
                        green = amount;
                    }
                }
                "blue" => {
                    if amount > blue {
                        blue = amount;
                    }
                }
                _ => {}
            };
        }
    }
    red * green * blue
}
