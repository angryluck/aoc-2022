// use std::error::Error;
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// #[derive(Debug)]
// struct _CrateStacks(HashMap<u32, Vec<char>>);

#[derive(Debug, PartialEq)]
struct MoveInstructions {
    amount: u32,
    from: u32,
    to: u32,
}

impl MoveInstructions {
    fn do_instruction(&self, crate_stack: &mut HashMap<u32, Vec<char>>) {
        let stack_from = crate_stack
            .get_mut(&self.from)
            .expect("Should be stack taking from");
        let mut moved_crates = Vec::new();
        for _ in 0..self.amount {
            moved_crates.push(stack_from.pop().expect("Should be crate  to move"));
        }
        let stack_to = crate_stack
            .get_mut(&self.to)
            .expect("Should be stack moving to");
        for crat in moved_crates {
            stack_to.push(crat);
        }
    }
}

fn main() {
    run().expect("Something went wrong");
}

fn run() -> Result<()> {
    let input = read_input("input.txt")?;
    let (mut stacks, instructions) = parse_input(input);
    for instruction in &instructions {
        instruction.do_instruction(&mut stacks);

    }
    println!("{}", get_answer(stacks));
    Ok(())
}

fn get_answer(crate_stack: HashMap<u32, Vec<char>>) -> String {
    let mut output = String::new();
    let mut stack_nr: u32 = 1;
    loop {
        if let Some(stack) =crate_stack.get(&stack_nr) {
            output.push(stack.last().expect("No stack should be empty").to_owned());
        } else {
            break;
        }
        stack_nr += 1;
    }
    output
}

// Using "anyhow" for error handeling here
// JUST RETURN A VEC - an iterator has to locate memomry anyways, and "Vec"
// seems like obvious way to do it
fn read_input(input_file: &str) -> Result<Vec<String>> {
    // Should work for any AOC project
    let file = File::open(input_file)?;
    let buf = BufReader::new(file);
    buf.lines().map(|l| Ok(l?)).collect()
}

fn parse_input(input: Vec<String>) -> (HashMap<u32, Vec<char>>, Vec<MoveInstructions>) {
    let mut input = input.into_iter();
    let mut stacks = input
        .by_ref()
        .take_while(|l| l != "")
        .collect::<Vec<String>>()
        .into_iter()
        .rev();
    let mut crate_stacks = initialize_stacks(stacks.next().expect("First line should be here"));

    dbg!(&crate_stacks);

    for stack in stacks {
        parse_stack_line(stack, &mut crate_stacks);
    }
    let instructions = input;

    dbg!(&instructions);

    let instructions = instructions
        .map(|line| parse_instruction_line(line))
        .collect();

    (crate_stacks, instructions)
}

fn parse_stack_line(line: String, crate_stacks: &mut HashMap<u32, Vec<char>>) {
    // "crate" is protected word
    for (mut kasse, stack) in line.chars().chunks(4).into_iter().zip(1u32..) {
        match kasse.nth(1) {
            Some(' ') => (),
            Some(x) => crate_stacks
                .get_mut(&stack)
                .expect("Stack index should exist")
                .push(x),
            None => panic!("There should be a character here"),
        }
    }
}

fn parse_instruction_line(line: String) -> MoveInstructions {
    let mut line = line.split(' ');
    let amount: u32 = line
        .nth(1)
        .expect("char should be here")
        .parse()
        .expect("should be a number");
    let from: u32 = line
        .nth(1)
        .expect("char should be here")
        .parse()
        .expect("should be a number");
    let to: u32 = line
        .nth(1)
        .expect("char should be here")
        .parse()
        .expect("should be a number");
    MoveInstructions { amount, from, to }
}

fn initialize_stacks(line: String) -> HashMap<u32, Vec<char>> {
    line.split_whitespace()
        .map(|num| (num.parse().unwrap(), Vec::new()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_sample() {
        let input = read_input("test-input.txt").unwrap();
        let (mut stacks, instructions) = parse_input(input);
        dbg!(&instructions);
        for instruction in &instructions {
            instruction.do_instruction(&mut stacks);
            println!("Stacks after instruction: {:?}", &stacks)
        }
        // dbg!(&stacks);
        assert_eq!("CMZ", get_answer(stacks))
    }

    #[test]
    fn one_line() {
        let mut crate_stacks: HashMap<u32, Vec<char>> =
            HashMap::from([(1, Vec::new()), (2, Vec::new()), (3, Vec::new())]);
        println!(
            "{:?}, {:?}, {:?}",
            crate_stacks[&1], crate_stacks[&2], crate_stacks[&3]
        );
        parse_stack_line("[Z] [M]    ".to_owned(), &mut crate_stacks);
        dbg!(crate_stacks);
        panic!();
    }

    #[test]
    fn simple_stack() {
        let input = " 1   2   3 ".to_owned();
        let output: HashMap<u32, Vec<char>> =
            HashMap::from([(1, Vec::new()), (2, Vec::new()), (3, Vec::new())]);
        assert_eq!(initialize_stacks(input), output);
    }

    #[test]
    fn simple_instruction() {
        let instruction = "move 2 from 3 to 1".to_owned();
        let target_output = MoveInstructions {amount: 2, from: 3, to: 1};
        assert_eq!(parse_instruction_line(instruction), target_output);
    }
}
