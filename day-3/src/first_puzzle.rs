use anyhow::Result;
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn first_puzzle() -> () {
    if let Ok(parsed_multiplications) = parse_input() {
        let mut sum = 0;
        for mul in parsed_multiplications {
            let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
            let captures = re.captures(&mul).unwrap();
            let factor1: u32 = captures[1].parse().unwrap();
            let factor2: u32 = captures[2].parse().unwrap();
            sum += factor1 * factor2;
        }
        println!("The sum of all multiplications is {}", sum)
    }
}

pub fn parse_input() -> Result<Vec<String>> {
    let file = File::open(Path::new("./day-3/inputs/input.txt"))?;

    let mut parsed_multiplications: Vec<_> = vec![];
    for line in io::BufReader::new(file).lines().flatten() {
        let re = Regex::new(r"(mul\([0-9]+,[0-9]+\))").unwrap();
        let mut matches: Vec<_> = re
            .find_iter(&line)
            .map(|m| String::from(m.as_str()))
            .collect();
        parsed_multiplications.append(&mut matches);
    }
    Ok(parsed_multiplications)
}
