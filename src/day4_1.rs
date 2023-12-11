use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("inputs/input4")?;
    let reader = BufReader::new(file);

    let num: u32 = reader.lines().map(|line| parse(&line.unwrap())).sum();
    println!("{num}");

    Ok(())
}

fn parse(line: &String) -> u32 {
    let (_, r) = line.split_once(": ").unwrap();
    let (win_str, nums_str) = r.split_once(" | ").unwrap();
    let win_nums: Vec<&str> = win_str
        .split(" ")
        .map(|c| c.trim())
        .filter(|c| !c.is_empty())
        .collect();
    let nums = nums_str
        .split(" ")
        .map(|c| c.trim())
        .filter(|c| !c.is_empty());

    let mut win_count = 0;
    for num in nums {
        'a: for win_num in win_nums.iter() {
            if num != *win_num {
                continue;
            }
            win_count += 1;
            break 'a;
        }
    }

    if win_count == 0 {
        return 0;
    }

    (2 as u32).pow(win_count - 1)
}
