use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("inputs/input3")?;
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    let width = buf.lines().next().unwrap_or("").len() + 2; // add +2 to incl \r\n
    println!("{}", width);
    println!("'{}'", buf.as_bytes()[width - 1] as char); // 140 is \r 141 is \n
    let re = Regex::new("[@#$%&*+=-]+")?;
    for (i, cap) in re.captures_iter(&buf).enumerate() {
        if i > 10 {
            return Ok(());
        }
        let pos = cap.get(0).unwrap().start();
        let symbol = cap.get(0).unwrap().as_str();
        let (x, y) = pos_to_xy(pos, width);
        println!("{pos}({x}, {y}){symbol}{}", buf.as_bytes()[xy_to_pos(x, y, width)] as char);
    }

    // println!("{num}");
    Ok(())
}

fn pos_to_xy(pos: usize, width: usize) -> (usize, usize) {
    (
        pos % width,
        (pos - pos%width)/width,
    )
}
fn xy_to_pos(x: usize, y: usize, width: usize) -> usize {
    width * y + x
}

/*
012345__
*/

fn parse(line: &String) -> u32 {
    1
}
