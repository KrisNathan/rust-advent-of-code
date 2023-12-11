use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("inputs/input4")?;
    let reader = BufReader::new(file.try_clone()?);
    let line_count = reader.lines().count();
    let mut card_counts: Vec<u32> = vec![1; line_count];

    let file = File::open("inputs/input4")?;
    let reader = BufReader::new(file);
    for (i, li) in reader.lines().enumerate() {
        let line = li.unwrap();
        let wins = count_wins(&line);
        if wins < 1 {
            continue;
        }
        for a in i + 1..i + 1 + wins as usize {
            if a >= line_count {
                continue;
            }
            card_counts[a] += card_counts[i];
        }
    }

    println!("{}", card_counts.iter().sum::<u32>());

    Ok(())
}

fn count_wins(line: &String) -> u32 {
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

    win_count
}
