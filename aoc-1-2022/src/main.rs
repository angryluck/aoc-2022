use std::{fs::File, io::{BufReader, BufRead}};
use itertools::Itertools;

fn main() {
    let input = read_input();
    // dbg!(input);
    let elf_list = parse_input(input);
    let max_calorie_list: Vec<u32> = elf_list.iter()
        .map(|elf| elf.total_calories()).sorted().rev().collect();
    dbg!(&max_calorie_list[..3]);
    let x: u32 = max_calorie_list[..3].iter().sum();
    dbg!(x);
    // let max = elf_list.iter().map(|elf| elf.total_calories()).max();
    // println!("{}", max.unwrap());
}

#[derive(Debug)]
struct Elf {
    calorie_list: Vec<u32>,
}

impl Elf {
    fn total_calories(&self) -> u32 {
        self.calorie_list.iter().sum()
    }
}

fn read_input() -> Vec<String> {
    let file = File::open("input.txt").expect("File does not exist");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}

fn parse_input(calories: Vec<String>) -> Vec<Elf> {
    let mut elves = Vec::new();
    for (key, group) in &calories.into_iter().group_by(|x| *x != "") {
        if key {
            let group = group.map(|s| s.parse().unwrap() ).collect();
            elves.push(Elf {calorie_list: group})
        }
    }
    elves
}
