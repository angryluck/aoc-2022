use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

#[derive(Debug)]
struct Rucksack(HashSet<char>);

#[derive(Debug)]
struct ElfGroup(Rucksack, Rucksack, Rucksack);

impl<S: Into<String>> From<S> for Rucksack {
    fn from(value: S) -> Self {
        let items = value.into().chars().collect();
        Rucksack(items)
    }
}

impl<S: Into<String>> From<(S, S, S)> for ElfGroup {
    fn from(value: (S, S, S)) -> Self {
        ElfGroup(
            Rucksack::from(value.0),
            Rucksack::from(value.1),
            Rucksack::from(value.2),
        )
    }
}

impl ElfGroup {
    fn badge(mut self) -> char {
        self.0
             .0
            .retain(|item| self.1 .0.contains(item) & self.2 .0.contains(item));
        let mut badge_iter = self.0 .0.into_iter();
        match badge_iter.next() {
            Some(x) => match badge_iter.next() {
                None => x,
                Some(y) => {
                    dbg!(x, y);
                    panic!("There was more than one intersection of badges")
                },
            },
            None => panic!("There was no intersection of badges"),
        }
    }

    fn priority(self, priority_list: &HashMap<char, u32>) -> u32 {
        priority_list[&self.badge()]
    }
}

fn main() {
    run();
}

fn read_input() -> Vec<String> {
    // Should work for any AOC project
    let file = std::fs::File::open("input.txt").expect("File does not exist");
    let buf = std::io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn parse_input(input: Vec<String>) -> Vec<ElfGroup> {
    input
        .iter()
        .tuples()
        .map(|(elf1, elf2, elf3)| ElfGroup::from((elf1, elf2, elf3)))
        .collect()
}

fn priority_sum(elf_groups: Vec<ElfGroup>, priority_list: &HashMap<char, u32>) -> u32 {
    elf_groups
        .into_iter()
        .map(|group| group.priority(&priority_list))
        .sum()
}

fn run() {
    let priority_list: HashMap<char, u32> = ('a'..='z').chain('A'..='Z').zip(1..=52).collect();
    let elf_groups = read_input();
    let elf_groups = parse_input(elf_groups);
    let total = priority_sum(elf_groups, &priority_list);
    println!("{}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_sample() {
        let priority_list: HashMap<char, u32> = ('a'..='z').chain('A'..='Z').zip(1..=52).collect();
        let input_string: Vec<&str> = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            .split('\n')
            .collect();
        let input_string = input_string
            .into_iter()
            .map(|line| line.to_owned())
            .collect();
        let elf_groups = parse_input(input_string);
        let total = priority_sum(elf_groups, &priority_list);
        assert_eq!(70, total);
    }
}
