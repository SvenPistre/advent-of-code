use anyhow::Result;
use regex::Regex;
use std::fmt::Display;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn second_puzzle() -> () {
    if let Ok(parsed_instructions) = parse_input() {
        let mut activation = 1;
        let mut sum = 0;
        for instruction in parsed_instructions {
            match instruction {
                Instruction::Do => activation = 1,
                Instruction::DoNot => activation = 0,
                Instruction::Mul(factor1, factor2) => sum += activation * factor1 * factor2,
            };
        }
        println!("The sum of all enabled multiplications is {}", sum)
    }
}

pub fn parse_input() -> Result<Vec<Instruction>> {
    let file = File::open(Path::new("./day-3/inputs/input.txt"))?;

    let mut parsed_instructions: Vec<_> = vec![];
    for line in io::BufReader::new(file).lines().flatten() {
        let re = Regex::new(r"(do\(\))|(don\'t\(\))|(mul\()([0-9]+),([0-9]+)\)").unwrap();
        let mut matches: Vec<_> = re
            .find_iter(&line)
            .map(|m| match m.as_str() {
                "do()" => Instruction::Do,
                "don't()" => Instruction::DoNot,
                match_string => {
                    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
                    let captures = re.captures(&match_string).unwrap();
                    return Instruction::Mul(
                        captures[1].parse().unwrap(),
                        captures[2].parse().unwrap(),
                    );
                }
            })
            .collect();
        parsed_instructions.push(Instruction::Do);
        parsed_instructions.append(&mut matches);
    }
    Ok(parsed_instructions)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instruction {
    Do,
    DoNot,
    Mul(u32, u32),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Instruction::Do => write!(f, "Do!"),
            Instruction::DoNot => write!(f, "Don't!"),
            Instruction::Mul(factor1, factor2) => write!(f, "{} * {}", factor1, factor2),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_when_decreasing_windows_by_1_or_2_then_return_safe() {
        
        assert_eq!(
            evaluate_status(&Report(vec![7, 6, 4, 2, 1])),
            ReportStatus::Safe
        );
    }
}
