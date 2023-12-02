use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("inputs/input1")?;
    let reader = BufReader::new(file);

    let nums: u32 = reader.lines().map(|line| parse(&line.unwrap())).sum();
    println!("{nums}");
    Ok(())
}

fn parse(line: &String) -> u32 {
    let numbers: Vec<u32> = line
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| if let Some(n) = c.to_digit(10) { n } else { 0 })
        .collect();

    numbers[0] * 10 + numbers[numbers.len() - 1]
}
