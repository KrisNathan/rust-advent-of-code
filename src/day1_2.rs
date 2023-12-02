use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("inputs/input1")?;
    let reader = BufReader::new(file);

    let nums: u32 = reader.lines().map(|line| parse(&line.unwrap())).sum();
    println!("{nums}");
    Ok(())
}

const NUMBERS_STR: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

const NUMBERS_U32: [u32; 18] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9];

fn parse(line: &String) -> u32 {
    let mut index_first: usize = usize::MAX;
    let mut number_first: u32 = 0;
    let mut index_last: usize = 0;
    let mut number_last: u32 = 0;
    for (num_id, num_str) in NUMBERS_STR.iter().enumerate() {
        if let Some(id) = line.find(num_str) {
            if id < index_first {
                index_first = id;
                number_first = NUMBERS_U32[num_id];
            }
        }
        if let Some(id) = line.rfind(num_str) {
            if id >= index_last {
                index_last = id;
                number_last = NUMBERS_U32[num_id];
            }
        }
    }
    number_first * 10 + number_last
}
