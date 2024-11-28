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

    fn _signal_strengths_sum(&self) -> i32 {
        let mut total = 0;
        let indices = [20, 60, 100, 140, 180, 220];
        for index in indices {
            total += self.register_x[index] * index as i32;
        }
        total
    }

    fn sprite_is_visible(&self, pixel: usize) -> bool {
        let index = pixel + 1;
        let pixel = pixel % 40;
        (self.register_x[index] - pixel as i32).abs() <= 1
    }

    fn draw_line(&self, start_pixel: usize) -> String {
        let mut output = String::new();
        for pixel in start_pixel..=start_pixel + 39 {
            match self.sprite_is_visible(pixel) {
                true => output += "#",
                false => output += ".",
            }
        }
        output
    }

    fn draw_crt(&self) -> String {
        let mut output = String::new();
        for line in 0..6 {
            output += &self.draw_line(line * 40);
            output += "\n";
        }
        output
    }
}

fn main() -> Result<()> {
    let result = run("input.txt")?;
    println!("The result is:\n{}", result);
    Ok(())
}

fn run(input: &str) -> Result<String> {
    let input = read_input(input)?;
    let instructions = parse_input(input)?;
    let mut cpu = CPU::new();
    for instruction in instructions {
        cpu.do_instruction(instruction);
    }
    Ok(cpu.draw_crt())
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
        dbg!(result);
        panic!()
    }

    #[test]
    fn draw_first_line() -> Result<()> {
        let input = read_input("test.txt")?;
        let instructions = parse_input(input)?;
        let mut cpu = CPU::new();
        for instruction in instructions {
            cpu.do_instruction(instruction);
        }
        println!("{}", cpu.draw_crt());
        panic!();
    }
}
