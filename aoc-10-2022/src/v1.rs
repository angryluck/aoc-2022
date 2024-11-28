use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Result};

#[derive(Debug)]
struct CPU {
    // JUST ADD A 1 TO THE START OF THE VEC!!!!
    register_x: Vec<i32>,
}

enum Instruction {
    Addx(i32),
    Noop,
}

impl TryFrom<String> for Instruction {
    type Error = anyhow::Error;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        let mut string = string.split_whitespace();
        match string.next() {
            Some(x) if x == "noop" => Ok(Self::Noop),
            Some(x) if x == "addx" => {
                let value = string.next().ok_or(anyhow!("No value given"))?.parse()?;
                Ok(Self::Addx(value))
            }
            Some(_) => Err(anyhow!("Invalid command")),
            None => Err(anyhow!("Empty string")),
        }
    }
}

impl CPU {
    fn new() -> Self {
        CPU {
            register_x: vec![1, 1],
        }
    }

    fn do_instruction(&mut self, instruction: Instruction) {
        let current_val = self.register_x.last().expect("Should not be empty").clone();
        match instruction {
            Instruction::Noop => self.register_x.push(current_val),
            Instruction::Addx(v) => {
                self.register_x.push(current_val);
                self.register_x.push(current_val + v);
            }
        }
    }

    fn signal_strengths_sum(&self) -> i32 {
        let mut total = 0;
        let indices = [20, 60, 100, 140, 180, 220];
        for index in indices {
            total += self.register_x[index] * index as i32;
        }
        total
    }
}

fn main() -> Result<()> {
    let result = run("input.txt")?;
    println!("The result is: {}", result);
    Ok(())
}

fn run(input: &str) -> Result<i32> {
    let input = read_input(input)?;
    let instructions = parse_input(input)?;
    let mut cpu = CPU::new();
    for instruction in instructions {
        cpu.do_instruction(instruction);
    }
    // dbg!(&cpu);
    // let test = cpu.register_x;
    // dbg!(test[20], test[60], test[100], test[140], test[180], test[220]);

    Ok(cpu.signal_strengths_sum())
}

fn read_input(input: &str) -> Result<Vec<String>> {
    let file = File::open(input)?;
    let buf = BufReader::new(file);
    buf.lines().map(|l| Ok(l?)).collect()
}

fn parse_input(input: Vec<String>) -> Result<Vec<Instruction>> {
    input
        .into_iter()
        .map(|l| Ok(Instruction::try_from(l)?))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_sample() -> Result<()> {
        let result = run("test.txt")?;
        assert_eq!(result, 13140);
        Ok(())
    }
}
